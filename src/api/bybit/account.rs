use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::BybitClient;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub total_equity: String,
    pub margin_balance: String,
}

impl BybitClient {
    pub async fn get_account_info(&self) -> Result<AccountInfo> {
        self.get_json("/v5/account/info", "").await
    }
}
