use axum::{Router, routing::get};

use crate::handlers::system_handler::{get_system_info, restart_system};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_system_info))
        .route("/restart", get(restart_system))
}
