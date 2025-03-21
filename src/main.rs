use axum::{Router, response::Result, routing::get};
use repository::{get_connection, system_info};
use std::net::SocketAddr;
mod handlers;
mod repository;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let app = routes::create_router();
    let writeresult = system_info::write_system_info().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running at http://");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
