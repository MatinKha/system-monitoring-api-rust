use axum;
mod health;
mod system_handler;

pub fn create_router() -> axum::Router {
    axum::Router::new()
        .nest("/system", system_handler::routes())
        //.nest("/metrics", metrics::routes())
        .nest("/health", health::routes())
}
