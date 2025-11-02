use shared::{PricePoint, PerformanceMetrics};
use chrono::{DateTime, Utc};

/// Calculate performance metrics from price data
pub fn calculate_metrics(
    asset_id: &str,
    prices: &[PricePoint],
) -> Option<PerformanceMetrics> {
    if prices.len() < 2 {
        return None;
    }

    let start_price = prices.first()?.price;
    let end_price = prices.last()?.price;
    let start_date = prices.first()?.timestamp;
    let end_date = prices.last()?.timestamp;

    // Calculate total return
    let total_return_pct = ((end_price / start_price) - 1.0) * 100.0;

    // Calculate annualized return
    let days = (end_date - start_date).num_days() as f64;
    let years = days / 365.25;
    let annualized_return_pct = if years > 0.0 {
        (((end_price / start_price).powf(1.0 / years)) - 1.0) * 100.0
    } else {
        0.0
    };

    // Calculate volatility (standard deviation of returns)
    let returns: Vec<f64> = prices
        .windows(2)
        .map(|window| {
            let prev_price = window[0].price;
            let curr_price = window[1].price;
            (curr_price / prev_price) - 1.0
        })
        .collect();

    let volatility = calculate_std_dev(&returns) * 100.0;

    Some(PerformanceMetrics {
        asset_id: asset_id.to_string(),
        total_return_pct,
        annualized_return_pct,
        volatility,
        start_date,
        end_date,
    })
}

fn calculate_std_dev(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / values.len() as f64;

    variance.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_std_dev() {
        let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let std_dev = calculate_std_dev(&values);
        assert!((std_dev - 2.0).abs() < 0.1);
    }
}
