use std::collections::HashMap;

use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use super::BybitClient;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Res {
    margin_mode: String,
    updated_time: String,
    unified_margin_status: u32,
    dcp_status: String,
    time_window: u32,
    smp_group: u32,
    is_master_trader: bool,
    spot_headging_status: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: Res,
}

impl BybitClient {
    pub const RECV_WINDOW: u16 = 20000;

    pub async fn get_account_info(&self) -> Result<AccountInfo, reqwest::Error> {
        let params = HashMap::new();
        let timestamp = Utc::now().timestamp_millis();
        let signature = self.create_signature(timestamp, params);

        let path = "/v5/account/info";
        let url = format!("{}{}", self.api_url, path);

        let mut headers = HeaderMap::new();
        headers.insert("X-BAPI-SIGN-TYPE", HeaderValue::from_str("2").unwrap());
        headers.insert(
            "X-BAPI-API-KEY",
            HeaderValue::from_str(self.api_key.as_ref()).unwrap(),
        );
        headers.insert(
            "X-BAPI-RECV-WINDOW",
            HeaderValue::from_str(&Self::RECV_WINDOW.to_string()).unwrap(),
        );
        headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&signature).unwrap());
        headers.insert(
            "X-BAPI-TIMESTAMP",
            HeaderValue::from_str(&timestamp.to_string()).unwrap(),
        );

        let client = reqwest::Client::new();

        let response = client
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .json::<AccountInfo>()
            .await?;

        Ok(response)
    }
}
