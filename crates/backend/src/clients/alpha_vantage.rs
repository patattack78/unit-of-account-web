use super::PriceDataClient;
use shared::PricePoint;
use chrono::{DateTime, Utc};
use reqwest::Client;

pub struct AlphaVantageClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl AlphaVantageClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://www.alphavantage.co/query".to_string(),
        }
    }
}

// #[async_trait]
// impl PriceDataClient for AlphaVantageClient {
//     async fn fetch_historical(
//         &self,
//         symbol: &str,
//         start_date: DateTime<Utc>,
//         end_date: DateTime<Utc>,
//     ) -> anyhow::Result<Vec<PricePoint>> {
//         // TODO: Implement Alpha Vantage API call
//         todo!("Implement Alpha Vantage historical data fetch")
//     }

//     async fn fetch_latest(&self, symbol: &str) -> anyhow::Result<PricePoint> {
//         // TODO: Implement Alpha Vantage API call
//         todo!("Implement Alpha Vantage latest price fetch")
//     }
// }
