use serde::{Deserialize, Serialize};

use super::BybitClient;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderReq {
    pub category: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub qty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRes {
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
}

impl BybitClient {
    pub async fn place_order(&self, req: PlaceOrderReq) -> anyhow::Result<PlaceOrderRes> {
        let body_json = serde_json::to_string(&req)?;

        let ts = (time::OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as i64;
        let sign_input = format!("{}{}{}{}", ts, self.api_key, self.recv_window_ms, body_json);

        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes())?;
        mac.update(sign_input.as_bytes());
        let sign = hex::encode(mac.finalize().into_bytes());

        let url = format!("{}/v5/order/create", self.base);
        let resp = self
            .http
            .post(&url)
            .header("X-BAPI-API-KEY", &self.api_key)
            .header("X-BAPI-TIMESTAMP", ts.to_string())
            .header("X-BAPI-RECV-WINDOW", self.recv_window_ms.to_string())
            .header("X-BAPI-SIGN", sign)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body_json)
            .send()
            .await?;

        let raw: BybitEnvelope<PlaceOrderRes> = resp.json().await?;

        if raw.ret_code == 0 {
            Ok(raw.result.unwrap_or(PlaceOrderRes::default()))
        } else {
            anyhow::bail!(
                "Bybit error {}: {} | ext={:?}",
                raw.ret_code,
                raw.ret_msg,
                raw.ret_ext_info
            )
        }
    }
}
