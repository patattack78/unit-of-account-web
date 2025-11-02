use super::PriceDataClient;
use shared::PricePoint;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;

pub struct CoinGeckoClient {
    client: Client,
    base_url: String,
}

impl CoinGeckoClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.coingecko.com/api/v3".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CoinGeckoPrice {
    // TODO: Define actual response structure
}

// #[async_trait]
// impl PriceDataClient for CoinGeckoClient {
//     async fn fetch_historical(
//         &self,
//         symbol: &str,
//         start_date: DateTime<Utc>,
//         end_date: DateTime<Utc>,
//     ) -> anyhow::Result<Vec<PricePoint>> {
//         // TODO: Implement CoinGecko API call
//         todo!("Implement CoinGecko historical data fetch")
//     }

//     async fn fetch_latest(&self, symbol: &str) -> anyhow::Result<PricePoint> {
//         // TODO: Implement CoinGecko API call
//         todo!("Implement CoinGecko latest price fetch")
//     }
// }
