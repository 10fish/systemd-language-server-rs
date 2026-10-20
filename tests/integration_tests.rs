use futures::StreamExt;
use serde_json::{Value, json};
use systemd_language_server::Backend;
use tower::Service;
use tower_lsp::jsonrpc::Request;
use tower_lsp::{ClientSocket, LspService};

async fn call(
    service: &mut LspService<Backend>,
    method: &'static str,
    id: i64,
    params: Value,
) -> Value {
    service
        .call(Request::build(method).id(id).params(params).finish())
        .await
        .expect("LSP service call failed")
        .expect("request did not produce a response")
        .result()
        .cloned()
        .expect("request returned an error")
}

async fn notify_and_receive(
    service: &mut LspService<Backend>,
    socket: &mut ClientSocket,
    method: &'static str,
    params: Value,
) -> Request {
    let notification = service.call(Request::build(method).params(params).finish());
    let (response, outgoing) = tokio::join!(notification, socket.next());
    assert!(response.expect("LSP service call failed").is_none());
    outgoing.expect("server did not send a client notification")
}

#[tokio::test]
async fn document_lifecycle_updates_diagnostics_completion_and_hover() {
    let (mut service, mut socket) = LspService::new(Backend::new);
    let initialized = call(&mut service, "initialize", 1, json!({"capabilities": {}})).await;
    assert_eq!(initialized["capabilities"]["textDocumentSync"], 1);

    let uri = "file:///tmp/integration.service";
    let diagnostics = notify_and_receive(
        &mut service,
        &mut socket,
        "textDocument/didOpen",
        json!({
            "textDocument": {
                "uri": uri,
                "languageId": "systemd",
                "version": 1,
                "text": "[Service]\nTy"
            }
        }),
    )
    .await;
    assert_eq!(diagnostics.method(), "textDocument/publishDiagnostics");
    assert_eq!(
        diagnostics.params().unwrap()["diagnostics"][0]["code"],
        "syntax"
    );
    assert_eq!(
        diagnostics.params().unwrap()["diagnostics"][0]["range"]["start"]["line"],
        1
    );

    let completion = call(
        &mut service,
        "textDocument/completion",
        2,
        json!({
            "textDocument": {"uri": uri},
            "position": {"line": 1, "character": 2}
        }),
    )
    .await;
    assert!(
        completion
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["label"] == "Type=")
    );

    let diagnostics = notify_and_receive(
        &mut service,
        &mut socket,
        "textDocument/didChange",
        json!({
            "textDocument": {"uri": uri, "version": 2},
            "contentChanges": [{"text": "[Service]\nRestart=always"}]
        }),
    )
    .await;
    assert_eq!(diagnostics.params().unwrap()["version"], 2);
    assert_eq!(diagnostics.params().unwrap()["diagnostics"], json!([]));

    let hover = call(
        &mut service,
        "textDocument/hover",
        3,
        json!({
            "textDocument": {"uri": uri},
            "position": {"line": 1, "character": 2}
        }),
    )
    .await;
    assert!(
        hover["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("restart policy")
    );

    let diagnostics = notify_and_receive(
        &mut service,
        &mut socket,
        "textDocument/didClose",
        json!({"textDocument": {"uri": uri}}),
    )
    .await;
    assert_eq!(diagnostics.params().unwrap()["diagnostics"], json!([]));

    let completion = call(
        &mut service,
        "textDocument/completion",
        4,
        json!({
            "textDocument": {"uri": uri},
            "position": {"line": 1, "character": 2}
        }),
    )
    .await;
    assert!(completion.is_null());
}
