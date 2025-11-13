use crate::shared::utils::deserializer::de_u64_str_ok;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BybitClient;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub margin_mode: Option<String>,
    #[serde(deserialize_with = "de_u64_str_ok")]
    pub updated_time: u64,
    pub unified_margin_status: Option<i32>,
    pub dcp_status: Option<String>,
    pub time_window: Option<i64>,
    pub smp_group: Option<i64>,
    pub is_master_trader: Option<bool>,
    pub spot_hedging_status: Option<String>,
}

impl BybitClient {
    pub async fn get_account_info(&self) -> Result<AccountInfo> {
        self.get_json("/v5/account/info", "").await
    }
}
