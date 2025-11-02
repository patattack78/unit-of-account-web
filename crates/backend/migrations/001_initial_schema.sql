-- Create assets table
CREATE TABLE IF NOT EXISTS assets (
    id TEXT PRIMARY KEY,
    symbol TEXT NOT NULL,
    name TEXT NOT NULL,
    asset_type TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create price_points table
CREATE TABLE IF NOT EXISTS price_points (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    asset_id TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    price REAL NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (asset_id) REFERENCES assets(id),
    UNIQUE(asset_id, timestamp)
);

-- Create index for efficient queries
CREATE INDEX IF NOT EXISTS idx_price_points_asset_timestamp 
ON price_points(asset_id, timestamp);

-- Create portfolios table
CREATE TABLE IF NOT EXISTS portfolios (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create portfolio_assets table for many-to-many relationship
CREATE TABLE IF NOT EXISTS portfolio_assets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    portfolio_id TEXT NOT NULL,
    asset_id TEXT NOT NULL,
    weight REAL NOT NULL,
    FOREIGN KEY (portfolio_id) REFERENCES portfolios(id),
    FOREIGN KEY (asset_id) REFERENCES assets(id),
    UNIQUE(portfolio_id, asset_id)
);
