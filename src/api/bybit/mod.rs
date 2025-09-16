use std::collections::HashMap;

use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::utils::config::Env;

#[derive(Debug, Clone)]
pub struct BybitClient {
    api_key: String,
    api_secret: String,
}

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize)]
pub struct Res {
    marginMode: String,
    updatedTime: String,
    unifiedMarginStatus: u32,
    dcpStatus: String,
    timeWindow: u32,
    smpGroup: Vec<u32>,
    isMasterTrader: bool,
    spotHedgingStatus: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountInfo {
    pub retCode: i32,
    pub retMsg: String,
    pub result: Res,
}

impl BybitClient {
    pub const RECV_WINDOW: u16 = 20000;

    pub fn new(api_key: String, api_secret: String) -> Self {
        BybitClient {
            api_key,
            api_secret,
        }
    }

    pub async fn get_account_info(&self) -> Result<AccountInfo, reqwest::Error> {
        let params = HashMap::new();
        let timestamp = Utc::now().timestamp();
        let signature = self.create_signature(timestamp, params);

        let path = "/v5/account/info";
        let api_url = Env::from_env().api_bybit_url;
        let url = format!("{}{}", api_url, path);

        let mut headers = HeaderMap::new();
        headers.insert("X-BAPI-SIGN-TYPE", HeaderValue::from_str("2").unwrap());
        headers.insert(
            "X-BAPI-API-KEY",
            HeaderValue::from_str(&self.api_key).unwrap(),
        );
        headers.insert(
            "X-BAPI-RECV-WINDOW",
            HeaderValue::from_str(&Self::RECV_WINDOW.to_string()).unwrap(),
        );
        headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&signature).unwrap());
        headers.insert(
            "X-BAPI-TIMESTAMP",
            HeaderValue::from_str(&signature).unwrap(),
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

    pub fn place_order(&self) {
        // Implementation for placing an order on Bybit
    }

    fn create_signature(&self, timestamp: i64, params: HashMap<String, String>) -> String {
        let query_string = serde_urlencoded::to_string(&params).unwrap_or_default();
        let meta = format!(
            "{}{}{}{}",
            timestamp,
            &self.api_key,
            Self::RECV_WINDOW,
            query_string
        );

        let mut mac = HmacSha256::new_from_slice(&self.api_key.clone().into_bytes())
            .expect("HMAC can take key of any size");
        mac.update(&meta.into_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
    }
}
