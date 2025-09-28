use std::collections::HashMap;

use hmac::{Hmac, Mac};
use sha2::Sha256;

use super::BybitClient;

type HmacSha256 = Hmac<Sha256>;

impl BybitClient {
    pub fn create_signature(&self, timestamp: i64, params: HashMap<String, String>) -> String {
        let query_string = serde_urlencoded::to_string(&params).unwrap_or_default();
        let meta = format!(
            "{}{}{}{}",
            timestamp,
            self.api_key,
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
