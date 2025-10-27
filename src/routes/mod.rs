use crate::api::bybit::BybitClient;
use axum::{http::StatusCode, Json};

pub async fn get_account_info() -> (StatusCode, Json<serde_json::Value>) {
    let bybit_client = BybitClient::new();

    match bybit_client.get_account_info().await {
        Ok(info) => {
            let v = serde_json::to_value(info).unwrap();
            (StatusCode::OK, Json(v))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("Request failed: {}", e) })),
        ),
    }
}
