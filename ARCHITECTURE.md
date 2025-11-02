# Architecture Overview

## System Design

```
┌─────────────────────────────────────────────────────────────┐
│                        USER BROWSER                          │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────────────────────────────────────────────┐   │
│  │         Frontend (Leptos + WebAssembly)             │   │
│  │                                                       │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────┐  │   │
│  │  │   Pages      │  │ Components   │  │   API    │  │   │
│  │  │              │  │              │  │  Client  │  │   │
│  │  │ - Home       │  │ - AssetCard  │  │          │  │   │
│  │  │ - Dashboard  │  │ - Chart      │  │ - fetch  │  │   │
│  │  └──────────────┘  └──────────────┘  └─────┬────┘  │   │
│  │                                              │        │   │
│  └──────────────────────────────────────────────┼───────┘   │
│                                                  │            │
└──────────────────────────────────────────────────┼───────────┘
                                                   │
                                            HTTP/JSON (CORS)
                                                   │
┌──────────────────────────────────────────────────┼───────────┐
│                     Backend Server               │            │
│                    (Axum + Tokio)                ▼            │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                    REST API                             │ │
│  │                                                          │ │
│  │  Routes:                                                 │ │
│  │  - GET  /                     (API info)                │ │
│  │  - GET  /health               (health check)            │ │
│  │  - GET  /api/compare          (compare assets)          │ │
│  │  - GET  /api/assets           (list assets)             │ │
│  │  - GET  /api/history/:symbol  (historical data)         │ │
│  └────────────────────────┬─────────────────────────────────┘ │
│                            │                                   │
│  ┌─────────────────────────┼──────────────────────────────┐  │
│  │         Services        │                               │  │
│  │                         ▼                               │  │
│  │  ┌──────────────────────────────────────────────────┐  │  │
│  │  │  PerformanceComparison                           │  │  │
│  │  │  - Normalize prices to equal starting values     │  │  │
│  │  │  - Calculate returns                              │  │  │
│  │  │  - Generate summary statistics                    │  │  │
│  │  └──────────────────────────────────────────────────┘  │  │
│  │                                                          │  │
│  │  ┌──────────────────────────────────────────────────┐  │  │
│  │  │  DataIngestion                                    │  │  │
│  │  │  - Fetch from external APIs                       │  │  │
│  │  │  - Store in database                              │  │  │
│  │  │  - Schedule periodic updates                      │  │  │
│  │  └──────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬────────────────────────────────┘  │
│                          │                                   │
│  ┌───────────────────────┼──────────────────────────────┐   │
│  │    External API       │          Database            │   │
│  │      Clients          │          (SQLite)            │   │
│  │                       │                              │   │
│  │  ┌──────────────┐     │    ┌─────────────────────┐  │   │
│  │  │ CoinGecko    │────────▶ │  assets             │  │   │
│  │  │ (Bitcoin)    │     │    │  - symbol (PK)      │  │   │
│  │  └──────────────┘     │    │  - name             │  │   │
│  │                       │    │  - asset_type       │  │   │
│  │  ┌──────────────┐     │    └─────────────────────┘  │   │
│  │  │ Alpha Vantage│     │                              │   │
│  │  │ (Stocks)     │────────▶ ┌─────────────────────┐  │   │
│  │  └──────────────┘     │    │  price_points       │  │   │
│  │                       │    │  - id (PK)          │  │   │
│  │  ┌──────────────┐     │    │  - asset_symbol(FK) │  │   │
│  │  │ Metals API   │────────▶ │  - timestamp        │  │   │
│  │  │ (Gold)       │     │    │  - price            │  │   │
│  │  └──────────────┘     │    │  - volume           │  │   │
│  │                       │    └─────────────────────┘  │   │
│  └───────────────────────┴──────────────────────────────┘   │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

## Data Flow

### 1. Historical Data Fetch
```
User Action → Frontend Request → Backend API → External APIs → Database → Response
```

### 2. Comparison View
```
1. User selects assets (QQQ, BTC, Gold)
2. User selects time range (1Y)
3. User sets initial value ($10,000)

Frontend sends:
{
  "assets": [Asset, Asset, Asset],
  "time_range": "OneYear",
  "initial_value": 10000.0
}

Backend:
1. Fetches price data from database
2. Normalizes to same starting value
3. Calculates performance metrics
4. Returns PerformanceData[]

Frontend:
1. Receives data
2. Renders chart using Plotters
3. Displays summary statistics
```

## Type Safety Flow

```rust
// 1. Define in shared/src/models.rs
#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub symbol: String,
    pub name: String,
    pub asset_type: AssetType,
}

// 2. Backend uses it (type-safe)
async fn get_assets() -> Json<Vec<Asset>> {
    Json(vec![Asset::bitcoin(), Asset::gold()])
}

// 3. Frontend uses it (type-safe)
let assets: Vec<Asset> = fetch_assets().await?;

// ✨ Full compile-time type checking from DB to UI!
```

## Key Design Decisions

### Why Leptos?
- Modern reactive framework
- Compiles to Wasm (no JS runtime)
- Fine-grained reactivity
- Great for data visualization

### Why WebAssembly?
- Type safety from backend to frontend
- Better performance for charts
- Learning opportunity for Rust ecosystem
- No need for separate JS/TS setup

### Why SQLite?
- Easy to set up
- No external database server needed
- Fast for read-heavy workloads
- Can migrate to Postgres later if needed

### Why Axum?
- Modern, tokio-based
- Type-safe routing
- Great ecosystem integration
- Excellent performance

## Performance Considerations

### Frontend
- Wasm bundle size: ~500KB (optimized)
- Chart rendering: Canvas-based (hardware accelerated)
- Data updates: Reactive signals (no unnecessary re-renders)

### Backend
- Connection pooling (SQLite)
- Async request handling (Tokio)
- External API rate limiting
- Caching strategy (future)

## Security

- CORS configured for development
- No authentication yet (future: JWT)
- Environment variables for API keys
- SQL injection prevented by sqlx

## Scalability Path

Current: Single server + SQLite
→ Add Redis cache for API responses
→ Switch to Postgres for better concurrency
→ Add separate data ingestion service
→ Deploy frontend as static files (CDN)
→ Scale backend horizontally

## Future Enhancements

1. **Real-time updates** (WebSocket)
2. **User accounts** (authentication)
3. **Custom portfolios** (save/load)
4. **More assets** (individual stocks, ETFs, etc.)
5. **Advanced charts** (candlesticks, indicators)
6. **Alerts** (price targets, performance thresholds)
7. **Export** (CSV, PDF reports)
