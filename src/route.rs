use axum::routing::get;
use axum::Router;

pub async fn root() -> &'static str {
    "Welcome to the root endpoint!"
}

pub async fn wor() -> &'static str {
    "Welcome to the /wor endpoint!"
}

pub fn get_router() -> Router {
    Router::new().route("/", get(root)).route("/wor", get(wor))
}
