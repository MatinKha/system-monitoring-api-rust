use axum::{Router, response::IntoResponse, routing::get};

pub fn routes() -> Router {
    Router::new().route("/", get(health_check))
}

pub async fn health_check() -> impl IntoResponse {
    "OK"
}
