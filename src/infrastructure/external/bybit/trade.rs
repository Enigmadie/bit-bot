use serde::{Deserialize, Serialize};

use super::BybitClient;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderReq {
    pub category: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub qty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRes {
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
}

impl BybitClient {}
