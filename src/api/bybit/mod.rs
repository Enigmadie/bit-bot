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
    secret_key: String,
    api_url: String,
}

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize)]
pub struct Res {
    marginMode: String,
    updatedTime: String,
    unifiedMarginStatus: u32,
    dcpStatus: String,
    timeWindow: u32,
    smpGroup: u32,
    isMasterTrader: bool,
    spotHedgingStatus: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountInfo {
    pub retCode: i32,
    pub retMsg: String,
    pub result: Res,
}

impl Default for BybitClient {
    fn default() -> Self {
        Self::new()
    }
}

impl BybitClient {
    pub const RECV_WINDOW: u16 = 20000;

    pub fn new() -> Self {
        let env = Env::from_env();
        Self {
            api_key: env.api_key_bybit,
            secret_key: env.secret_bybit,
            api_url: env.api_bybit_url,
        }
    }

    pub async fn get_account_info(&self) -> Result<AccountInfo, reqwest::Error> {
        let params = HashMap::new();
        let timestamp = Utc::now().timestamp_millis();
        let signature = self.create_signature(timestamp, params);

        let path = "/v5/account/info";
        let url = format!("{}{}", &self.api_url, path);

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

        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(&meta.into_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_signature() {
        let client = BybitClient {
            api_key: "test_key".to_string(),
            secret_key: "test_secret".to_string(),
            api_url: "https://api.bybit.com".to_string(),
        };
        let timestamp = 1697587200000;
        let params = HashMap::new();
        let signature = client.create_signature(timestamp, params);
        assert_eq!(signature.len(), 64);
    }
}
