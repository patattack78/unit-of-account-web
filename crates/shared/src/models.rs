use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Types of assets we track
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AssetType {
    Stock,     // QQQ, SPY, etc.
    Crypto,    // BTC
    Commodity, // Gold
}

/// Individual asset (e.g., "BTC", "QQQ")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub asset_type: AssetType,
}

/// Price point at a specific time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub asset_id: String,
    pub timestamp: DateTime<Utc>,
    pub price: f64,
}

/// Portfolio composition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: String,
    pub name: String,
    pub assets: Vec<Asset>,
}
