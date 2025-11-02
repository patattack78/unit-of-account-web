use shared::{PricePoint, NormalizedPricePoint};
use chrono::{DateTime, Utc};

/// Normalize price points to start from a specific initial amount
pub fn normalize_prices(
    prices: &[PricePoint],
    initial_amount: f64,
) -> Vec<NormalizedPricePoint> {
    if prices.is_empty() {
        return vec![];
    }

    let first_price = prices[0].price;

    prices
        .iter()
        .map(|point| {
            let normalized_value = (point.price / first_price) * initial_amount;
            let return_pct = ((point.price / first_price) - 1.0) * 100.0;

            NormalizedPricePoint {
                timestamp: point.timestamp,
                normalized_value,
                return_pct,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_prices() {
        let prices = vec![
            PricePoint {
                asset_id: "TEST".to_string(),
                timestamp: Utc::now(),
                price: 100.0,
            },
            PricePoint {
                asset_id: "TEST".to_string(),
                timestamp: Utc::now(),
                price: 110.0,
            },
            PricePoint {
                asset_id: "TEST".to_string(),
                timestamp: Utc::now(),
                price: 95.0,
            },
        ];

        let normalized = normalize_prices(&prices, 10000.0);

        assert_eq!(normalized.len(), 3);
        assert_eq!(normalized[0].normalized_value, 10000.0);
        assert_eq!(normalized[1].normalized_value, 11000.0);
        assert_eq!(normalized[2].normalized_value, 9500.0);
    }
}
