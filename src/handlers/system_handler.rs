use crate::services::system_info::fetch_system_info;
use axum::{Json, Router, response::IntoResponse, routing::get};

pub async fn get_system_info() -> impl IntoResponse {
    let info = fetch_system_info();
    Json("sdlfkj");
    return info;
}

pub async fn restart_system() -> impl IntoResponse {
    // Placeholder: Add logic to restart system safely
    Json("skldfjasdlfj");
    return "dkfjsdfkl";
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_system_info))
        .route("/restart", get(restart_system))
}
