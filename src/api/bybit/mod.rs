use std::collections::HashMap;

use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Debug, Clone)]
pub struct BybitClient {
    api_key: String,
    api_secret: String,
}

type HmacSha256 = Hmac<Sha256>;

impl BybitClient {
    pub fn new(api_key: String, api_secret: String) -> Self {
        BybitClient {
            api_key,
            api_secret,
        }
    }

    pub fn get_account_info(&self) -> String {
        // Implementation for fetching account info from Bybit
        let recv_window = 20000;
        let params = HashMap::new();
        self.create_signature(4214124124, recv_window, params)
    }

    pub fn place_order(&self) {
        // Implementation for placing an order on Bybit
    }

    fn create_signature(
        &self,
        timestamp: u64,
        recv_window: u64,
        params: HashMap<String, String>,
    ) -> String {
        let query_string = serde_urlencoded::to_string(&params).unwrap_or_default();
        let meta = format!(
            "{}{}{}{}",
            timestamp, &self.api_key, recv_window, query_string
        );

        let mut mac = HmacSha256::new_from_slice(&self.api_key.clone().into_bytes())
            .expect("HMAC can take key of any size");
        mac.update(&meta.into_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        hex::encode(code_bytes)
    }
}
