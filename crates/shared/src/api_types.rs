use super::models::*;
use serde::{Deserialize, Serialize};

/// API request/response types

#[derive(Debug, Serialize, Deserialize)]
pub struct CompareAssetsRequest {
    pub asset_ids: Vec<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompareAssetsResponse {
    pub assets: Vec<Asset>,
    pub price_data: Vec<PricePoint>,
}
