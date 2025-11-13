use std::env;

#[derive(Debug)]
pub struct Env {
    pub api_key_bybit: String,
    pub secret_bybit: String,
    pub api_bybit_url: String,
}

impl Env {
    pub fn from_env() -> Self {
        let bybit_testnet = env::var("BYBIT_TESTNET").unwrap_or_else(|_| "false".into()) == "true";
        let api_bybit_url = if bybit_testnet {
            "https://api-testnet.bybit.com".to_string()
        } else {
            "https://api.bybit.com".to_string()
        };
        Self {
            api_key_bybit: env::var("API_KEY_BYBIT").expect("Missing API_KEY_BYBIT"),
            secret_bybit: env::var("SECRET_BYBIT").expect("Missing SECRET_BYBIT"),
            api_bybit_url,
        }
    }
}
