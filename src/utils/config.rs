use std::env;

pub fn get_api_key() -> String {
    env::var("CMC_API_KEY").expect("CMC_API_KEY not set")
}
