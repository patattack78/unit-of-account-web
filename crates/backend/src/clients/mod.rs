// External API clients for fetching price data
// Each client will implement a common trait for consistency

pub mod coingecko;
pub mod alpha_vantage;

use shared::PricePoint;
use chrono::{DateTime, Utc};
use async_trait::async_trait;

#[async_trait]
pub trait PriceDataClient {
    /// Fetch historical price data for an asset
    async fn fetch_historical(
        &self,
        symbol: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> anyhow::Result<Vec<PricePoint>>;

    /// Fetch the latest price for an asset
    async fn fetch_latest(&self, symbol: &str) -> anyhow::Result<PricePoint>;
}
