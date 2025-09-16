use std::env;

#[derive(Debug)]
pub struct Env {
    pub api_key_bybit: String,
    pub secret_bybit: String,
    pub api_bybit_url: String,
}

impl Env {
    pub fn from_env() -> Self {
        Self {
            api_key_bybit: env::var("API_KEY_BYBIT").expect("Missing API_KEY_BYBIT"),
            secret_bybit: env::var("SECRET_BYBIT").expect("Missing SECRET_BYBIT"),
            api_bybit_url: env::var("API_BYBIT_URL")
                .unwrap_or_else(|_| "https://api.bybit.com".to_string()),
        }
    }
}
