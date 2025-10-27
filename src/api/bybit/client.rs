use reqwest::Client;

use crate::utils::config::Env;

#[derive(Debug, Clone)]
pub struct BybitClient {
    pub(super) api_key: String,
    pub(super) secret_key: String,
    pub(super) base_url: String,
    pub(super) client: Client,
    pub(super) recv_window_ms: u16,
}

impl Default for BybitClient {
    fn default() -> Self {
        Self::new()
    }
}

impl BybitClient {
    pub fn new() -> Self {
        let env = Env::from_env();
        Self {
            api_key: env.api_key_bybit,
            secret_key: env.secret_bybit,
            base_url: env.api_bybit_url,
            client: Client::new(),
            recv_window_ms: 5000,
        }
    }
}
