use crate::session::manager::SessionManagerCmd;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct AppState {
    pub session_tx: Sender<SessionManagerCmd>,
}

#[derive(Debug)]
pub struct AppError(anyhow::Error);

#[derive(Deserialize)]
pub struct AddClient {
    ip: String,
}

pub async fn root() -> &'static str {
    "Welcome to the root endpoint!"
}

pub async fn start(
    State(app_state): State<AppState>,
    Json(add_client): Json<AddClient>,
) -> impl IntoResponse {
    log::trace!("endpoint /wor reached");
    let cmd = SessionManagerCmd::AddSession { 0: add_client.ip };
    match app_state.session_tx.send(cmd).await {
        Err(e) => {
            let msg = "Failed to notify session_manager";
            log::error!("{}: {}", msg, e);
            (StatusCode::from_u16(500).unwrap(), msg)
        }
        Ok(()) => {
            (StatusCode::OK, "")
        }
    }
}

pub async fn stop(
    State(app_state): State<AppState>,
    Json(add_client): Json<AddClient>,
) -> impl IntoResponse {
    log::trace!("endpoint /wor reached");
    let cmd = SessionManagerCmd::RemoveSession { 0: add_client.ip };
    match app_state.session_tx.send(cmd).await {
        Err(e) => {
            let msg = "Failed to notify session_manager";
            log::error!("{}: {}", msg, e);
            (StatusCode::from_u16(500).unwrap(), msg)
        }
        Ok(()) => {
            (StatusCode::OK, "")
        }
    }
}

pub fn get_router(state: AppState) -> Router {
    Router::new() //
        .route("/", get(root))
        .route("/start", post(start))
        .route("/stop", post(stop))
        .with_state(state)
}
