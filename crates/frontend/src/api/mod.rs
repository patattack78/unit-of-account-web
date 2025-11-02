use gloo_net::http::Request;
use shared::{
    GetAssetsResponse, ComparisonRequest, ComparisonResponse,
    RefreshDataRequest, RefreshDataResponse,
};

const API_BASE_URL: &str = "http://localhost:3000/api";

pub async fn fetch_assets() -> Result<GetAssetsResponse, String> {
    let response = Request::get(&format!("{}/assets", API_BASE_URL))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch assets: {}", e))?;

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse assets: {}", e))
}

pub async fn fetch_comparison(request: ComparisonRequest) -> Result<ComparisonResponse, String> {
    let response = Request::post(&format!("{}/comparison", API_BASE_URL))
        .json(&request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to fetch comparison: {}", e))?;

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse comparison: {}", e))
}

pub async fn refresh_data(request: RefreshDataRequest) -> Result<RefreshDataResponse, String> {
    let response = Request::post(&format!("{}/refresh", API_BASE_URL))
        .json(&request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to refresh data: {}", e))?;

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse refresh response: {}", e))
}
