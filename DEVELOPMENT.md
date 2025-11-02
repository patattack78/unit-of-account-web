# Development Guide

## Prerequisites

Make sure you have the following installed:

```bash
# Rust (latest stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Wasm target for Rust
rustup target add wasm32-unknown-unknown

# Trunk (Wasm build tool)
cargo install trunk

# SQLx CLI (for database migrations)
cargo install sqlx-cli --no-default-features --features sqlite
```

## Project Structure

```
portfolio-tracker/
├── crates/
│   ├── shared/          # Shared types between frontend & backend
│   │   ├── models.rs    # Core data models (Asset, PricePoint, etc.)
│   │   └── api_types.rs # Request/Response DTOs
│   │
│   ├── backend/         # Axum REST API
│   │   ├── src/
│   │   │   ├── main.rs      # Server entry point
│   │   │   ├── routes/      # API endpoints
│   │   │   ├── services/    # Business logic
│   │   │   ├── db/          # Database connection
│   │   │   └── clients/     # External API clients
│   │   └── migrations/      # SQL migrations
│   │
│   └── frontend/        # Leptos Wasm app
│       ├── src/
│       │   ├── lib.rs       # App entry point
│       │   ├── pages/       # Page components
│       │   ├── components/  # Reusable UI components
│       │   └── api/         # Backend API client
│       ├── index.html
│       ├── Trunk.toml
│       └── static/          # CSS and assets
```

## Setup

### 1. Clone and Configure

```bash
# Copy environment variables
cp .env.example .env

# Edit .env if needed (database path, API keys, etc.)
```

### 2. Build the Workspace

```bash
# From project root, build all crates
cargo build

# This compiles:
# - shared library
# - backend server
# - frontend (as lib)
```

### 3. Set Up Database

```bash
cd crates/backend

# Create database and run migrations
sqlx database create
sqlx migrate run

# This creates:
# - assets table
# - price_points table
# - default assets (QQQ, SPY, BTC, Gold, etc.)
```

## Running the Application

### Terminal 1: Backend Server

```bash
cd crates/backend
cargo run

# Server runs on http://localhost:3000
# API endpoints:
#   GET  /            - API info
#   GET  /health      - Health check
#   GET  /api/compare - Compare assets (WIP)
```

### Terminal 2: Frontend Dev Server

```bash
cd crates/frontend
trunk serve

# App runs on http://localhost:8080
# Auto-reloads on file changes
# Compiles Rust to Wasm
```

## Development Workflow

### Adding a New Type (End-to-End Type Safety)

1. **Define in `shared/src/models.rs`**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyNewType {
    pub field: String,
}
```

2. **Use in Backend** (`backend/src/routes/mod.rs`):
```rust
use shared::MyNewType;

async fn handler() -> Json<MyNewType> {
    Json(MyNewType { 
        field: "test".to_string() 
    })
}
```

3. **Use in Frontend** (`frontend/src/api/mod.rs`):
```rust
use shared::MyNewType;

pub async fn fetch_data() -> Result<MyNewType, Error> {
    // Type-safe request/response
    let response = Request::get("/api/endpoint")
        .send()
        .await?
        .json::<MyNewType>()
        .await?;
    Ok(response)
}
```

### Adding a New API Endpoint

1. **Backend**: Add route in `backend/src/routes/mod.rs`
2. **Frontend**: Add API call in `frontend/src/api/mod.rs`
3. **Types**: Ensure request/response types in `shared/src/api_types.rs`

### Adding a New Component

Create in `frontend/src/components/`:
```rust
use leptos::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <div>"My Component"</div>
    }
}
```

## Testing

```bash
# Run tests for all crates
cargo test

# Run specific crate tests
cargo test -p backend
cargo test -p shared
```

## Building for Production

```bash
# Backend (optimized binary)
cd crates/backend
cargo build --release

# Frontend (optimized Wasm)
cd crates/frontend
trunk build --release

# Output in crates/frontend/dist/
```

## Common Issues

### "error: database doesn't exist"
```bash
cd crates/backend
sqlx database create
sqlx migrate run
```

### "failed to fetch wasm"
Make sure Trunk is serving on port 8080 and you're accessing `http://localhost:8080`

### CORS errors
Backend CORS is configured to allow all origins in development. Check backend logs.

### Type mismatch between frontend/backend
Ensure both use the same `shared` crate version. Run `cargo clean` and rebuild.

## Next Steps

- [ ] Implement CoinGecko API client
- [ ] Add Alpha Vantage client for stocks
- [ ] Implement data fetching service
- [ ] Build comparison logic
- [ ] Create chart visualization
- [ ] Add time range selector
- [ ] Add portfolio builder UI

## Resources

- [Leptos Docs](https://leptos.dev/)
- [Axum Docs](https://docs.rs/axum/latest/axum/)
- [Plotters Docs](https://docs.rs/plotters/latest/plotters/)
- [SQLx Docs](https://docs.rs/sqlx/latest/sqlx/)
