use axum::Router;
use bit_bot::{
    application::accounts::AccountService,
    infrastructure::external::bybit::BybitClient,
    presentation::http::{routes::create_router, state::AppState},
};
use dotenv::dotenv;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bybit_client = BybitClient::new();

    let account_service = AccountService::new(bybit_client);

    let state = AppState { account_service };

    let app: Router = create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
