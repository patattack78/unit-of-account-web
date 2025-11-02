# unit of account

# Portfolio Tracker

A full-stack Rust application for tracking and comparing investment performance across traditional equities (QQQ, S&P 500, NASDAQ) and hard assets (Bitcoin, Gold).

## 🎯 Project Goals

- **Learn Rust + WebAssembly**: Full-stack Rust with Wasm frontend for type-safe development
- **Compare Asset Classes**: Track historical and real-time performance of stocks vs. hard assets
- **Equal Starting Amount Analysis**: Normalize investments to compare percentage returns
- **Extensible Architecture**: Easy to add new portfolios and asset types

## 🏗️ Architecture

### Tech Stack

**Frontend (WebAssembly)**
- Framework: Leptos (signals-based reactivity)
- Charts: Plotters (pure Rust, compiles to Wasm) or custom implementation
- Build Tool: Trunk
- Type Safety: End-to-end Rust types

**Backend**
- Web Framework: Axum (async, tokio-based)
- Database: SQLite with sqlx (async, compile-time checked queries)
- Scheduler: tokio-cron-scheduler for periodic price updates
- HTTP Client: reqwest for external API calls

**Shared**
- Common types and models shared between frontend and backend
- Serialization: serde with feature flags for wasm-bindgen

### Data Sources

- **Stocks**: Alpha Vantage, Yahoo Finance API, or Twelve Data
- **Bitcoin**: CoinGecko API or CoinCap API  
- **Gold**: Alpha Vantage or Metals-API

## 📁 Project Structure

```
portfolio-tracker/
├── Cargo.toml                    # Workspace definition
├── README.md
├── .gitignore
│
├── crates/
│   ├── backend/                  # Axum REST API server
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs           # Application entry point
│   │   │   ├── routes/           # HTTP route handlers
│   │   │   ├── services/         # Business logic layer
│   │   │   ├── db/               # Database models & queries
│   │   │   └── clients/          # External API clients
│   │   └── migrations/           # SQL schema migrations
│   │
│   ├── frontend/                 # Leptos WebAssembly app
│   │   ├── Cargo.toml
│   │   ├── Trunk.toml            # Trunk build configuration
│   │   ├── index.html
│   │   ├── src/
│   │   │   ├── main.rs           # Wasm entry point
│   │   │   ├── components/       # Reusable UI components
│   │   │   ├── pages/            # Page-level views
│   │   │   ├── charts/           # Chart visualization components
│   │   │   └── api/              # Backend API client
│   │   └── static/               # CSS, images, other assets
│   │
│   ├── shared/                   # Shared types (FE + BE)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models.rs         # Core domain models
│   │       └── api_types.rs      # API request/response DTOs
│   │
│   └── charting/                 # Custom charting library (optional)
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── line_chart.rs     # Line chart implementation
│           └── canvas.rs         # Canvas rendering utilities
│
└── scripts/                      # Utility scripts
    └── seed_historical_data.rs   # Historical data import tool
```

## 🚀 Features

### Phase 1: MVP (Current)
- [ ] Project scaffolding and workspace setup
- [ ] Core data models (Asset, PricePoint, Portfolio)
- [ ] Historical data ingestion for BTC, Gold, QQQ, SPY, NASDAQ
- [ ] SQLite database with price history
- [ ] Basic REST API endpoints
- [ ] WebAssembly frontend with basic chart
- [ ] Performance comparison view (normalized to equal starting amounts)

### Phase 2: Enhancement
- [ ] Real-time price updates (scheduled jobs)
- [ ] Multiple timeframe views (1M, 3M, 1Y, 5Y, All)
- [ ] Key performance metrics (total return %, CAGR, volatility, Sharpe ratio)
- [ ] Responsive design for mobile
- [ ] Dark/light theme toggle

### Phase 3: Advanced
- [ ] Custom portfolio builder (add your own stock allocations)
- [ ] Portfolio rebalancing scenarios
- [ ] What-if analysis tools
- [ ] Export data (CSV, JSON)
- [ ] User authentication and saved portfolios
- [ ] Advanced charting (candlesticks, volume, indicators)

## 🔧 Development Setup

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add Wasm target
rustup target add wasm32-unknown-unknown

# Install Trunk (Wasm build tool)
cargo install trunk

# Install sqlx-cli for database migrations
cargo install sqlx-cli --no-default-features --features sqlite
```

### Running the Project

```bash
# Terminal 1: Run backend
cd crates/backend
cargo run

# Terminal 2: Run frontend (serves on http://localhost:8080)
cd crates/frontend
trunk serve
```

### Database Setup

```bash
# Create database and run migrations
cd crates/backend
sqlx database create
sqlx migrate run
```

## 📊 Data Models

### Core Types

```rust
// Asset types we track
pub enum AssetType {
    Stock,      // QQQ, SPY, etc.
    Crypto,     // BTC
    Commodity,  // Gold
}

// Individual price point
pub struct PricePoint {
    pub asset_id: String,
    pub timestamp: DateTime<Utc>,
    pub price: f64,
}

// Portfolio composition
pub struct Portfolio {
    pub id: String,
    pub name: String,
    pub allocations: Vec<Allocation>,
}

// Performance metrics
pub struct PerformanceMetrics {
    pub total_return_pct: f64,
    pub annualized_return_pct: f64,
    pub volatility: f64,
    pub sharpe_ratio: f64,
}
```

## 🎨 UI Design Principles

- **Clean & Minimal**: Focus on data visualization
- **Performance First**: Leverage Wasm for smooth chart rendering
- **Responsive**: Mobile-friendly design
- **Accessible**: Proper contrast, keyboard navigation
- **Type-Safe**: No runtime errors from API responses

## 📝 API Endpoints

```
GET  /api/assets                    # List all tracked assets
GET  /api/assets/:id/prices         # Get price history for asset
GET  /api/portfolio/:id              # Get portfolio details
GET  /api/compare                    # Compare multiple assets
POST /api/portfolio                  # Create new portfolio
```

## 🧪 Testing Strategy

- **Backend**: Unit tests for services, integration tests for API endpoints
- **Frontend**: Component tests with Leptos testing utilities
- **E2E**: Playwright tests for critical user flows
- **Data Quality**: Validation tests for external API data

## 📚 Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Leptos Documentation](https://leptos.dev/)
- [Axum Guide](https://docs.rs/axum/)
- [WebAssembly Concepts](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [Plotters Documentation](https://docs.rs/plotters/)

## 🤝 Contributing

This is a personal learning project, but suggestions and improvements are welcome!

## 📄 License

MIT License - feel free to use this as a learning resource or template for your own projects.

## 🔮 Future Ideas

- Add more assets: real estate ETFs, bonds, international markets
- Machine learning predictions using Rust ML libraries
- Social features: share and compare portfolios
- Mobile app using Tauri
- Desktop app for offline analysis

---

**Status**: 🚧 In Development | **Current Phase**: Project Setup & Architecture
