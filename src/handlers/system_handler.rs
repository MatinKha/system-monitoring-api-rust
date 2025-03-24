use crate::services::system_info::fetch_system_info;
use axum::{Json, Router, response::IntoResponse, routing::get};

pub async fn get_cpu() -> impl IntoResponse {
    let info = fetch_system_info();
    Json("sdlfkj");
    return info;
}

pub async fn get_ram() -> impl IntoResponse {
    // Placeholder: Add logic to restart system safely
    let json = Json("skldfjasdlfj");
    return json;
}

pub fn routes() -> Router {
    Router::new()
        .route("/cpu", get(get_cpu))
        .route("/ram", get(get_ram))
}
