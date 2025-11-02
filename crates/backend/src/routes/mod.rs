use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use shared::{
    ComparisonRequest, ComparisonResponse, GetAssetsResponse, RefreshDataRequest,
    RefreshDataResponse, Asset, ErrorResponse,
};

use crate::db::DbPool;

pub fn api_routes() -> Router<DbPool> {
    Router::new()
        .route("/assets", get(get_assets))
        .route("/comparison", post(get_comparison))
        .route("/refresh", post(refresh_data))
}

async fn get_assets(
    State(_pool): State<DbPool>,
) -> Result<Json<GetAssetsResponse>, (StatusCode, Json<ErrorResponse>)> {
    // For now, return the default assets
    // Later, we'll fetch from database
    let assets = Asset::all_default();

    Ok(Json(GetAssetsResponse { assets }))
}

async fn get_comparison(
    State(_pool): State<DbPool>,
    Json(_request): Json<ComparisonRequest>,
) -> Result<Json<ComparisonResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement comparison logic
    // 1. Fetch price data from database
    // 2. Normalize to initial_amount
    // 3. Calculate metrics
    // 4. Return time series + metrics

    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse {
            error: "Not implemented yet".to_string(),
            details: Some("Comparison endpoint is under development".to_string()),
        }),
    ))
}

async fn refresh_data(
    State(_pool): State<DbPool>,
    Json(_request): Json<RefreshDataRequest>,
) -> Result<Json<RefreshDataResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implement data refresh
    // 1. Fetch latest prices from external APIs
    // 2. Store in database
    // 3. Return success response

    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse {
            error: "Not implemented yet".to_string(),
            details: Some("Refresh endpoint is under development".to_string()),
        }),
    ))
}
