use crate::api::bybit::BybitClient;
use axum::{http::StatusCode, Json};
use serde_json::Value;

pub async fn get_account_info() -> (StatusCode, Json<Value>) {
    let bybit_client = BybitClient::new();

    match bybit_client.get_account_info().await {
        Ok(payload) => (StatusCode::OK, Json(serde_json::to_value(payload).unwrap())),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("Request failed: {}", e) })),
        ),
    }
}
