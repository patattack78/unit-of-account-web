use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build the router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .layer(CorsLayer::permissive());

    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Backend server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Portfolio Tracker API - Ready!"
}

async fn health() -> &'static str {
    "OK"
}
