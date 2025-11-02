-- Initial schema for portfolio tracker

-- Assets table
CREATE TABLE IF NOT EXISTS assets (
    symbol TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    asset_type TEXT NOT NULL CHECK(asset_type IN ('STOCK', 'CRYPTO', 'COMMODITY')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Price points table
CREATE TABLE IF NOT EXISTS price_points (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    asset_symbol TEXT NOT NULL,
    timestamp DATETIME NOT NULL,
    price REAL NOT NULL,
    volume REAL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (asset_symbol) REFERENCES assets(symbol),
    UNIQUE(asset_symbol, timestamp)
);

-- Create indices for performance
CREATE INDEX IF NOT EXISTS idx_price_points_symbol_timestamp 
ON price_points(asset_symbol, timestamp);

CREATE INDEX IF NOT EXISTS idx_price_points_timestamp 
ON price_points(timestamp);

-- Insert default assets
INSERT OR IGNORE INTO assets (symbol, name, asset_type) VALUES
    ('QQQ', 'Invesco QQQ Trust', 'STOCK'),
    ('SPY', 'S&P 500 ETF', 'STOCK'),
    ('^IXIC', 'NASDAQ Composite', 'STOCK'),
    ('BTC', 'Bitcoin', 'CRYPTO'),
    ('XAU', 'Gold', 'COMMODITY');
