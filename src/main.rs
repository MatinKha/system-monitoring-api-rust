use axum::{Router, routing::get};
use std::net::SocketAddr;
mod handlers;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let app = routes::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running at http://");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
