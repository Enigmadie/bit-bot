use axum::{routing::get, Router};
use dotenv::dotenv;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
