mod cli;
mod ext;
mod route;
mod session;

use crate::route::AppState;
use crate::session::manager::SessionManager;
use clap::Parser;
use ext::net::is_ipv4_local;
use itertools::Itertools;
use simple_logger::SimpleLogger;
use std::fs;
use std::process::exit;
use tokio::select;


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
    let invalid_ips = config.names.iter()
        .map(|name| { name.ip_address })
        .filter(|ip| { !is_ipv4_local(ip) })
        .map(|ip| { ip.to_string() })
        .join(",");
    if !invalid_ips.is_empty() {
        panic!("Following ips are not local: {}", invalid_ips);
    }
    log::info!("Parsed Config: {:#?}", config);

    // Start session manager
    let mut session_mgr = SessionManager::new();
    let session_tx = session_mgr.task_queue_tx.clone();
    log::info!("Starting session manager");
    let session_mgr_fiber = session_mgr.run();

    // Launch an HTTP server using a basic example
    let app_state = AppState { session_tx };
    let app = route::get_router(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("Starting HTTP server on http://127.0.0.1:3000");
    let http_fiber = axum::serve(listener, app);

    // Await any fibers finishing
    select! {
        _ = session_mgr_fiber => {
            println!("Session Manager closed.");
        }
        val = http_fiber => {
            match val {
                Ok(()) => println!("HTTP Server closed."),
                Err(e) => {
                    println!("HTTP Server Error: {}", e);
                    exit(2)
                }
            }
        }
    }
}

