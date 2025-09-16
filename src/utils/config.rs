use std::env;

#[derive(Debug)]
pub struct ApiKeys {
    pub api_key_bybit: String,
    pub secret_bybit: String,
    pub api_bybit_url: String,
}

impl ApiKeys {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            api_key_bybit: env::var("API_KEY_BYBIT")?,
            secret_bybit: env::var("SECRET_BYBIT")?,
            api_bybit_url: env::var("SECRET_BYBIT")?,
        })
    }
}
