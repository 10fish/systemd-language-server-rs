use log::info;
use std::error::Error;
use systemd_language_server::Backend;
use tokio::io::{stdin, stdout};
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Initialize logging
    env_logger::init();
    info!("Starting Systemd Language Server...");

    // Create standard input/output streams
    let stdin = stdin();
    let stdout = stdout();

    // Create LSP service
    let (service, socket) = LspService::new(Backend::new);

    // Start server
    info!("Systemd Language Server started, waiting for client connection...");
    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
