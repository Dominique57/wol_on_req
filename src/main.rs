mod cli;
mod ext;
mod route;

use clap::Parser;
use simple_logger::SimpleLogger;
use std::fs;

#[tokio::main]
async fn main() {
    // Init logger
    SimpleLogger::new().init().unwrap();

    // Parse args
    let cli = cli::Cli::parse();
    let config_content =
        fs::read_to_string(&cli.file).expect("Failed to read the configuration file.");
    let config: cli::Config =
        serde_json::from_str(&config_content).expect("Failed to parse the configuration file.");
    log::info!("Parsed Config: {:#?}", config);

    // Launch an HTTP server using a basic example
    let app = route::get_router();
    log::info!("Server running on http://127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
