mod document;
mod schema;

use document::{Document, DocumentState, DocumentStore, contains};
use ini::configparser::ini::Ini;
use log::info;
use schema::{
    UnitType, directive_documentation, directive_names, is_drop_in, section_documentation,
    value_completions,
};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

const SOURCE: &str = "systemd-language-server";

pub struct Backend {
    client: Client,
    documents: DocumentStore,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DocumentStore::default(),
        }
    }

    fn diagnostics_for(&self, uri: &Url, document: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = document
            .errors
            .iter()
            .map(|error| Diagnostic {
                range: error.range,
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("syntax".into())),
                source: Some(SOURCE.into()),
                message: error.message.clone(),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let unit_type = UnitType::from_uri(uri);
        let allowed_sections = unit_type.sections();

        for section in &document.sections {
            if !allowed_sections.contains(&section.name.as_str()) {
                diagnostics.push(Diagnostic {
                    range: section.name_range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("unknown-section".into())),
                    source: Some(SOURCE.into()),
                    message: format!("section [{}] is not valid for this unit type", section.name),
                    ..Default::default()
                });
            }

            for directive in &section.directives {
                if directive.value.is_empty() {
                    diagnostics.push(Diagnostic {
                        range: directive.value_range,
                        severity: Some(DiagnosticSeverity::WARNING),
                        code: Some(NumberOrString::String("empty-value".into())),
                        source: Some(SOURCE.into()),
                        message: format!("directive '{}' has an empty value", directive.key),
                        ..Default::default()
                    });
                }
            }
        }

        if !is_drop_in(uri)
            && let Some(required) = unit_type.specific_section()
            && !document
                .sections
                .iter()
                .any(|section| section.name == required)
        {
            diagnostics.push(Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("missing-section".into())),
                source: Some(SOURCE.into()),
                message: format!("required section [{required}] is missing"),
                ..Default::default()
            });
        }

        diagnostics
    }

    fn completion_items(
        &self,
        uri: &Url,
        state: &DocumentState,
        position: Position,
    ) -> Vec<CompletionItem> {
        let line = state
            .text
            .split('\n')
            .nth(position.line as usize)
            .unwrap_or_default();
        let prefix = utf16_prefix(line, position.character);
        let trimmed = prefix.trim_start();
        let unit_type = UnitType::from_uri(uri);

        if trimmed.starts_with('[') && !trimmed.contains(']') {
            return unit_type
                .sections()
                .into_iter()
                .map(|section| CompletionItem {
                    label: format!("{section}]"),
                    kind: Some(CompletionItemKind::MODULE),
                    detail: section_documentation(section).map(str::to_string),
                    ..Default::default()
                })
                .collect();
        }

        let Some(section) = state.parsed.section_at(position.line) else {
            return unit_type
                .sections()
                .into_iter()
                .map(|section| CompletionItem {
                    label: format!("[{section}]"),
                    kind: Some(CompletionItemKind::MODULE),
                    detail: section_documentation(section).map(str::to_string),
                    ..Default::default()
                })
                .collect();
        };

        if let Some((key, _)) = trimmed.split_once('=') {
            let values = value_completions(&section.name, key.trim());
            if !values.is_empty() {
                return values
                    .iter()
                    .map(|value| CompletionItem {
                        label: (*value).into(),
                        kind: Some(CompletionItemKind::VALUE),
                        ..Default::default()
                    })
                    .collect();
            }
        }

        directive_names(&section.name)
            .iter()
            .map(|key| CompletionItem {
                label: format!("{key}="),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: directive_documentation(key).map(str::to_string),
                ..Default::default()
            })
            .collect()
    }

    fn hover_at(&self, state: &DocumentState, position: Position) -> Option<Hover> {
        for section in &state.parsed.sections {
            if contains(section.name_range, position) {
                return hover(section_documentation(&section.name)?, section.name_range);
            }
            for directive in &section.directives {
                if contains(directive.key_range, position) {
                    return hover(
                        directive_documentation(&directive.key)?,
                        directive.key_range,
                    );
                }
            }
        }
        None
    }

    async fn publish(&self, uri: Url, version: Option<i32>, document: &Document) {
        let diagnostics = self.diagnostics_for(&uri, document);
        self.client
            .publish_diagnostics(uri, diagnostics, version)
            .await;
    }
}

fn hover(text: &str, range: Range) -> Option<Hover> {
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: text.into(),
        }),
        range: Some(range),
    })
}

fn utf16_prefix(line: &str, character: u32) -> &str {
    let mut utf16 = 0;
    for (byte, value) in line.char_indices() {
        if utf16 >= character {
            return &line[..byte];
        }
        utf16 += value.len_utf16() as u32;
    }
    line
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        info!("Systemd Language Server initialized");
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["[".into(), "=".into()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "systemd-language-server".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("Systemd Language Server is ready");
        self.client
            .log_message(MessageType::INFO, "Systemd Language Server has started")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let document = params.text_document;
        self.documents
            .open(document.uri.clone(), document.version, document.text);
        if let Some(state) = self.documents.get(&document.uri) {
            self.publish(document.uri, Some(state.version), &state.parsed)
                .await;
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let Some(change) = params.content_changes.into_iter().last() else {
            return;
        };
        let uri = params.text_document.uri;
        let version = params.text_document.version;
        if self.documents.change(uri.clone(), version, change.text)
            && let Some(state) = self.documents.get(&uri)
        {
            self.publish(uri, Some(state.version), &state.parsed).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.close(&params.text_document.uri);
        self.client
            .publish_diagnostics(params.text_document.uri, Vec::new(), None)
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let text = params.text_document_position;
        let Some(state) = self.documents.get(&text.text_document.uri) else {
            return Ok(None);
        };
        Ok(Some(CompletionResponse::Array(self.completion_items(
            &text.text_document.uri,
            &state,
            text.position,
        ))))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let text = params.text_document_position_params;
        let Some(state) = self.documents.get(&text.text_document.uri) else {
            return Ok(None);
        };
        Ok(self.hover_at(&state, text.position))
    }
}

/// Compatibility parser retained for callers of the 0.1 API.
pub fn parse_unit_file(content: &str) -> anyhow::Result<Ini> {
    let mut ini = Ini::new();
    ini.read(content.to_string()).map_err(anyhow::Error::msg)?;
    Ok(ini)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tower::Service;
    use tower_lsp::LspService;
    use tower_lsp::jsonrpc::Request;

    async fn call(
        service: &mut LspService<Backend>,
        request: Request,
    ) -> Option<serde_json::Value> {
        service
            .call(request)
            .await
            .unwrap()
            .and_then(|response| response.result().cloned())
    }

    #[tokio::test]
    async fn lsp_open_change_completion_hover_and_close() {
        let (mut service, socket) = LspService::new(Backend::new);
        drop(socket);
        let initialize = Request::build("initialize")
            .id(1)
            .params(json!({"capabilities": {}}))
            .finish();
        assert!(call(&mut service, initialize).await.is_some());

        let uri = "file:///tmp/example.service";
        service
            .call(
                Request::build("textDocument/didOpen")
                    .params(json!({
                        "textDocument": {
                            "uri": uri,
                            "languageId": "systemd",
                            "version": 1,
                            "text": "[Service]\nTy"
                        }
                    }))
                    .finish(),
            )
            .await
            .unwrap();

        let completion = call(
            &mut service,
            Request::build("textDocument/completion")
                .id(2)
                .params(json!({
                    "textDocument": {"uri": uri},
                    "position": {"line": 1, "character": 2}
                }))
                .finish(),
        )
        .await
        .unwrap();
        assert!(completion.to_string().contains("Type="));

        service
            .call(
                Request::build("textDocument/didChange")
                    .params(json!({
                        "textDocument": {"uri": uri, "version": 2},
                        "contentChanges": [{"text": "[Service]\nType=simple"}]
                    }))
                    .finish(),
            )
            .await
            .unwrap();

        let hover = call(
            &mut service,
            Request::build("textDocument/hover")
                .id(3)
                .params(json!({
                    "textDocument": {"uri": uri},
                    "position": {"line": 1, "character": 1}
                }))
                .finish(),
        )
        .await;
        assert!(hover.is_some());

        service
            .call(
                Request::build("textDocument/didClose")
                    .params(json!({"textDocument": {"uri": uri}}))
                    .finish(),
            )
            .await
            .unwrap();

        let completion = call(
            &mut service,
            Request::build("textDocument/completion")
                .id(4)
                .params(json!({
                    "textDocument": {"uri": uri},
                    "position": {"line": 1, "character": 1}
                }))
                .finish(),
        )
        .await;
        assert!(completion.is_some_and(|value| value.is_null()));
    }

    #[test]
    fn diagnostics_have_real_ranges_and_safe_unknown_type() {
        let document = Document::parse("[Wrong]\nKey=\n");
        let uri = Url::parse("file:///tmp/example.unknown").unwrap();
        let (service, socket) = LspService::new(Backend::new);
        drop(socket);
        let diagnostics = service.inner().diagnostics_for(&uri, &document);
        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0].range.start, Position::new(0, 1));
        assert_eq!(
            diagnostics[0].code,
            Some(NumberOrString::String("unknown-section".into()))
        );
        assert_eq!(diagnostics[1].range.start, Position::new(1, 4));
    }

    #[test]
    fn drop_in_does_not_require_the_unit_specific_section() {
        let document = Document::parse("[Unit]\nAfter=network.target\n");
        let uri = Url::parse("file:///tmp/example.service.d/override.conf").unwrap();
        let (service, socket) = LspService::new(Backend::new);
        drop(socket);
        assert!(service.inner().diagnostics_for(&uri, &document).is_empty());
    }
}
