use axum::Json;
use axum::Router;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use sysinfo::System;

use crate::repository::system_info;

pub async fn get_unamed_inf() -> Response {
    let mut sys = System::new_all();
    Json(sys.global_cpu_usage()).into_response()
}

pub fn routes() -> Router {
    Router::new().route("/test", get(get_unamed_inf))
}
