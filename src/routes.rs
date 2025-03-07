use axum::{Router, routing::get};

mod health;
//mod metrics;
mod system;

pub fn create_router() -> Router {
    Router::new()
        .nest("/system", system::routes())
        //.nest("/metrics", metrics::routes())
        .nest("/health", health::routes())
}
