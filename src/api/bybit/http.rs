use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use hmac::{Hmac, Mac};
use serde::{de::DeserializeOwned, Deserialize};
use sha2::Sha256;

use super::BybitClient;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiResp<T> {
    ret_code: i32,
    ret_msg: String,
    result: Option<T>,
    #[serde(default)]
    ret_ext_info: Option<serde_json::Value>,
}

impl BybitClient {
    pub fn sign(&self, timestamp: i64, qs: &str) -> String {
        let meta = format!("{}{}{}{}", timestamp, self.api_key, self.recv_window_ms, qs);
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(&meta.into_bytes());
        let result = mac.finalize().into_bytes();
        hex::encode(result)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, path: &str, qs: &str) -> Result<T> {
        let ts = Utc::now().timestamp_millis();
        let sign = self.sign(ts, qs);
        let url = if qs.is_empty() {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}{}?{}", self.base_url, path, qs)
        };

        let resp = self
            .client
            .get(&url)
            .header("X-BAPI-API-KEY", &self.api_key)
            .header("X-BAPI-TIMESTAMP", ts.to_string())
            .header("X-BAPI-RECV-WINDOW", self.recv_window_ms.to_string())
            .header("X-BAPI-SIGN", sign)
            .send()
            .await
            .context("network error (GET)")?;

        let status = resp.status();
        let text = resp.text().await.context("read body (GET)")?;

        if !status.is_success() {
            return Err(anyhow!("HTTP {}: {}", status.as_u16(), truncate(&text)));
        }

        let api: ApiResp<serde_json::Value> = serde_json::from_str(&text)
            .with_context(|| format!("decode ApiResp (GET): {}", truncate(&text)))?;

        if api.ret_code != 0 {
            return Err(anyhow!("Bybit {}: {}", api.ret_code, api.ret_msg));
        }

        let val = api.result.unwrap_or(serde_json::Value::Null);
        let typed: T = serde_json::from_value(val)
            .with_context(|| format!("decode result<T> (GET): {}", truncate(&text)))?;
        Ok(typed)
    }
}

fn truncate(s: &str) -> String {
    const N: usize = 300;
    if s.len() > N {
        format!("{}…", &s[..N])
    } else {
        s.to_string()
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
            base_url: "https://api.bybit.com".to_string(),
            recv_window_ms: 5000,
            client: reqwest::Client::new(),
        };
        let timestamp = 1697587200000;
        let signature = client.sign(timestamp, "");
        assert_eq!(signature.len(), 64);
    }
}
