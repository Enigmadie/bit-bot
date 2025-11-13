use anyhow::Result;

use crate::infrastructure::external::bybit::{account::AccountInfo, BybitClient};

#[derive(Clone)]
pub struct AccountService {
    bybit: BybitClient,
}

impl AccountService {
    pub fn new(bybit: BybitClient) -> Self {
        Self { bybit }
    }

    pub async fn get_account_info(&self) -> Result<AccountInfo> {
        self.bybit.get_account_info().await
    }
}
