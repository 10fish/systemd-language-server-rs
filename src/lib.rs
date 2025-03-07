use ini::configparser::ini::Ini;
use log::info;
use std::collections::HashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

pub struct Backend {
    client: Client,
    // Store opened file contents
    documents: HashMap<Url, String>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: HashMap::new(),
        }
    }

    // Parse systemd unit file
    fn parse_unit_file(&self, content: &str) -> anyhow::Result<Ini> {
        let mut ini = Ini::new();
        if let Err(e) = ini.read(content.to_string()) {
            return Err(anyhow::anyhow!(e));
        }
        Ok(ini)
    }

    // Generate diagnostics
    fn generate_diagnostics(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        match self.parse_unit_file(content) {
            Ok(_) => {
                // File format is correct, no diagnostics needed
            }
            Err(e) => {
                // Add syntax error diagnostic
                let diagnostic = Diagnostic {
                    range: Range {
                        start: Position::new(0, 0),
                        end: Position::new(0, 1),
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("systemd-lsp".into()),
                    message: format!("Systemd unit file syntax error: {}", e),
                    related_information: None,
                    tags: None,
                    data: None,
                };
                diagnostics.push(diagnostic);
            }
        }

        // Check for common systemd configuration errors
        self.check_common_errors(content, &mut diagnostics);

        diagnostics
    }

    // Check for common systemd configuration errors
    fn check_common_errors(&self, content: &str, diagnostics: &mut Vec<Diagnostic>) {
        let lines: Vec<&str> = content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let line_num = i as u32;

            // Check line format
            if line.contains('=') && !line.trim().starts_with('#') && !line.trim().starts_with('[')
            {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();

                    // Check for empty values
                    if value.is_empty() {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position::new(line_num, 0),
                                end: Position::new(line_num, line.len() as u32),
                            },
                            severity: Some(DiagnosticSeverity::WARNING),
                            message: format!("Key '{}' has an empty value", key),
                            source: Some("systemd-lsp".into()),
                            ..Default::default()
                        });
                    }

                    // Check for common configuration errors
                    match key {
                        "ExecStart" => {
                            if !value.starts_with('/') && !value.starts_with('-') {
                                diagnostics.push(Diagnostic {
                                    range: Range {
                                        start: Position::new(line_num, 0),
                                        end: Position::new(line_num, line.len() as u32),
                                    },
                                    severity: Some(DiagnosticSeverity::WARNING),
                                    message: "ExecStart should use absolute paths".to_string(),
                                    source: Some("systemd-lsp".into()),
                                    ..Default::default()
                                });
                            }
                        }
                        "Type" => {
                            let valid_types =
                                ["simple", "forking", "oneshot", "dbus", "notify", "idle"];
                            if !valid_types.contains(&value) {
                                diagnostics.push(Diagnostic {
                                    range: Range {
                                        start: Position::new(line_num, 0),
                                        end: Position::new(line_num, line.len() as u32),
                                    },
                                    severity: Some(DiagnosticSeverity::ERROR),
                                    message: format!(
                                        "Invalid service type: '{}'. Valid types: {:?}",
                                        value, valid_types
                                    ),
                                    source: Some("systemd-lsp".into()),
                                    ..Default::default()
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Get completion items
    fn get_completion_items(&self, position: &Position, document_uri: &Url) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        // Get current document content
        if let Some(content) = self.documents.get(document_uri) {
            let lines: Vec<&str> = content.lines().collect();

            // Get current line
            if let Some(line) = lines.get(position.line as usize) {
                let line = *line;

                // Check if currently in a section name
                if line.trim().starts_with('[') && !line.contains(']') {
                    // Provide section name completions
                    items.extend(vec![
                        CompletionItem::new_simple(
                            "Unit]".into(),
                            "Unit configuration section".into(),
                        ),
                        CompletionItem::new_simple(
                            "Service]".into(),
                            "Service configuration section".into(),
                        ),
                        CompletionItem::new_simple(
                            "Install]".into(),
                            "Install configuration section".into(),
                        ),
                        CompletionItem::new_simple(
                            "Socket]".into(),
                            "Socket configuration section".into(),
                        ),
                        CompletionItem::new_simple(
                            "Mount]".into(),
                            "Mount configuration section".into(),
                        ),
                        CompletionItem::new_simple(
                            "Timer]".into(),
                            "Timer configuration section".into(),
                        ),
                    ]);
                } else {
                    // Provide key completions based on current section
                    let current_section = self.get_current_section(lines, position.line as usize);

                    match current_section.as_deref() {
                        Some("Unit") => {
                            items.extend(vec![
                                CompletionItem::new_simple(
                                    "Description=".into(),
                                    "Unit description".into(),
                                ),
                                CompletionItem::new_simple(
                                    "Documentation=".into(),
                                    "Documentation URL".into(),
                                ),
                                CompletionItem::new_simple(
                                    "Requires=".into(),
                                    "Strong dependencies".into(),
                                ),
                                CompletionItem::new_simple(
                                    "Wants=".into(),
                                    "Weak dependencies".into(),
                                ),
                                CompletionItem::new_simple(
                                    "After=".into(),
                                    "Start order dependency".into(),
                                ),
                                CompletionItem::new_simple(
                                    "Before=".into(),
                                    "Start order dependency".into(),
                                ),
                                CompletionItem::new_simple(
                                    "Conflicts=".into(),
                                    "Conflicting units".into(),
                                ),
                            ]);
                        }
                        Some("Service") => {
                            items.extend(vec![
                                CompletionItem::new_simple("Type=".into(), "Service type".into()),
                                CompletionItem::new_simple(
                                    "ExecStart=".into(),
                                    "Start command".into(),
                                ),
                                CompletionItem::new_simple(
                                    "ExecStop=".into(),
                                    "Stop command".into(),
                                ),
                                CompletionItem::new_simple(
                                    "Restart=".into(),
                                    "Restart policy".into(),
                                ),
                                CompletionItem::new_simple(
                                    "RestartSec=".into(),
                                    "Restart interval".into(),
                                ),
                                CompletionItem::new_simple("User=".into(), "Run as user".into()),
                                CompletionItem::new_simple("Group=".into(), "Run as group".into()),
                                CompletionItem::new_simple(
                                    "WorkingDirectory=".into(),
                                    "Working directory".into(),
                                ),
                            ]);
                        }
                        Some("Install") => {
                            items.extend(vec![
                                CompletionItem::new_simple(
                                    "WantedBy=".into(),
                                    "Wanted by targets".into(),
                                ),
                                CompletionItem::new_simple(
                                    "RequiredBy=".into(),
                                    "Required by targets".into(),
                                ),
                                CompletionItem::new_simple("Alias=".into(), "Unit alias".into()),
                            ]);
                        }
                        Some("Socket") => {
                            items.extend(vec![
                                CompletionItem::new_simple(
                                    "ListenStream=".into(),
                                    "Listen on TCP port".into(),
                                ),
                                CompletionItem::new_simple(
                                    "ListenDatagram=".into(),
                                    "Listen on UDP port".into(),
                                ),
                                CompletionItem::new_simple(
                                    "Accept=".into(),
                                    "Accept connections".into(),
                                ),
                            ]);
                        }
                        Some("Timer") => {
                            items.extend(vec![
                                CompletionItem::new_simple(
                                    "OnBootSec=".into(),
                                    "Delay after boot".into(),
                                ),
                                CompletionItem::new_simple(
                                    "OnUnitActiveSec=".into(),
                                    "Delay after unit activation".into(),
                                ),
                                CompletionItem::new_simple(
                                    "OnCalendar=".into(),
                                    "Calendar-based trigger".into(),
                                ),
                            ]);
                        }
                        _ => {
                            // Default to providing all section names
                            items.extend(vec![
                                CompletionItem::new_simple(
                                    "[Unit]".into(),
                                    "Unit configuration section".into(),
                                ),
                                CompletionItem::new_simple(
                                    "[Service]".into(),
                                    "Service configuration section".into(),
                                ),
                                CompletionItem::new_simple(
                                    "[Install]".into(),
                                    "Install configuration section".into(),
                                ),
                                CompletionItem::new_simple(
                                    "[Socket]".into(),
                                    "Socket configuration section".into(),
                                ),
                                CompletionItem::new_simple(
                                    "[Mount]".into(),
                                    "Mount configuration section".into(),
                                ),
                                CompletionItem::new_simple(
                                    "[Timer]".into(),
                                    "Timer configuration section".into(),
                                ),
                            ]);
                        }
                    }
                }
            }
        }

        items
    }

    // Get current section
    fn get_current_section(&self, lines: Vec<&str>, current_line: usize) -> Option<String> {
        let mut current_section = None;

        for (i, line) in lines.iter().enumerate() {
            if i > current_line {
                break;
            }

            let line = line.trim();
            if line.starts_with('[') && line.ends_with(']') {
                let section = line[1..line.len() - 1].to_string();
                current_section = Some(section);
            }
        }

        current_section
    }

    // Get hover information
    fn get_hover_info(&self, position: &Position, document_uri: &Url) -> Option<Hover> {
        if let Some(content) = self.documents.get(document_uri) {
            let lines: Vec<&str> = content.lines().collect();

            if let Some(line) = lines.get(position.line as usize) {
                let line = *line;

                // Check if hovering over a section name
                if line.trim().starts_with('[') && line.trim().ends_with(']') {
                    let section = line.trim()[1..line.trim().len() - 1].to_string();

                    let hover_text = match section.as_str() {
                        "Unit" => {
                            "The Unit section contains basic information about the unit, such as description and dependencies."
                        }
                        "Service" => {
                            "The Service section contains service configuration, such as start commands and restart policies."
                        }
                        "Install" => {
                            "The Install section contains installation information, such as which targets want this unit."
                        }
                        "Socket" => {
                            "The Socket section contains socket configuration, such as listening addresses and ports."
                        }
                        "Mount" => "The Mount section contains mount point configuration.",
                        "Timer" => {
                            "The Timer section contains timer configuration, used for scheduled service activation."
                        }
                        _ => return None,
                    };

                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: hover_text.to_string(),
                        }),
                        range: Some(Range {
                            start: Position::new(position.line, 0),
                            end: Position::new(position.line, line.len() as u32),
                        }),
                    });
                }

                // Check if hovering over a key-value pair
                if line.contains('=') && !line.trim().starts_with('#') {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim();

                        // Provide hover information based on key
                        let hover_text = match key {
                            "Description" => "Describes the unit's function and purpose.",
                            "After" => {
                                "Defines start order, this unit will start after the specified units."
                            }
                            "Before" => {
                                "Defines start order, this unit will start before the specified units."
                            }
                            "Requires" => {
                                "Strong dependency relationship, if the dependency fails, this unit will also fail."
                            }
                            "Wants" => {
                                "Weak dependency relationship, dependency failure won't affect this unit."
                            }
                            "ExecStart" => {
                                "Defines the command to execute when the service starts. Should use absolute paths."
                            }
                            "ExecStop" => "Defines the command to execute when the service stops.",
                            "Type" => {
                                "Defines the service type, can be simple, forking, oneshot, dbus, notify, or idle."
                            }
                            "Restart" => "Defines the restart policy when the service exits.",
                            "WantedBy" => {
                                "Specifies which targets want this unit, used for enabling the unit."
                            }
                            _ => return None,
                        };

                        return Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: hover_text.to_string(),
                            }),
                            range: Some(Range {
                                start: Position::new(position.line, 0),
                                end: Position::new(position.line, key.len() as u32),
                            }),
                        });
                    }
                }
            }
        }

        None
    }
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
                    trigger_characters: Some(vec!["[".to_string(), "=".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "systemd-language-server".to_string(),
                version: Some("0.1.0".to_string()),
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
        info!("Systemd Language Server is shutting down");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        info!("File opened: {:?}", params.text_document.uri);

        // Store document content
        let mut documents = self.documents.clone();
        documents.insert(
            params.text_document.uri.clone(),
            params.text_document.text.clone(),
        );

        // Generate diagnostics
        let diagnostics = self.generate_diagnostics(&params.text_document.text);

        // Publish diagnostics
        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        info!("File changed: {:?}", params.text_document.uri);

        if let Some(change) = params.content_changes.get(0) {
            // Update document content
            let mut documents = self.documents.clone();
            documents.insert(params.text_document.uri.clone(), change.text.clone());

            // Generate diagnostics
            let diagnostics = self.generate_diagnostics(&change.text);

            // Publish diagnostics
            self.client
                .publish_diagnostics(params.text_document.uri.clone(), diagnostics, None)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        info!("File closed: {:?}", params.text_document.uri);

        // Remove document content
        let mut documents = self.documents.clone();
        documents.remove(&params.text_document.uri);

        // Clear diagnostics
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let position = params.text_document_position.position;
        let document_uri = params.text_document_position.text_document.uri;

        let items = self.get_completion_items(&position, &document_uri);

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let position = params.text_document_position_params.position;
        let document_uri = params.text_document_position_params.text_document.uri;

        Ok(self.get_hover_info(&position, &document_uri))
    }
}

// Export public function for testing
pub fn parse_unit_file(content: &str) -> anyhow::Result<Ini> {
    let mut ini = Ini::new();
    if let Err(e) = ini.read(content.to_string()) {
        return Err(anyhow::anyhow!(e));
    }
    Ok(ini)
}
