use hyper::Server;

mod handlers;
mod repository;
mod services;

#[tokio::main]
async fn main() {
    let app = handlers::create_router();
    let address = "0.0.0.0:3000";

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("🚀 Server running at http://{}", address);
    let server = Server::new();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
