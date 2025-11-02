#!/bin/bash

# Portfolio Tracker Setup Verification Script

set -e

echo "🔍 Portfolio Tracker Setup Verification"
echo "========================================"
echo ""

# Check Rust
echo "✓ Checking Rust installation..."
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Install from https://rustup.rs/"
    exit 1
fi
RUST_VERSION=$(rustc --version)
echo "  Found: $RUST_VERSION"
echo ""

# Check Wasm target
echo "✓ Checking wasm32-unknown-unknown target..."
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "❌ Wasm target not installed. Run: rustup target add wasm32-unknown-unknown"
    exit 1
fi
echo "  Target installed ✓"
echo ""

# Check Trunk
echo "✓ Checking Trunk..."
if ! command -v trunk &> /dev/null; then
    echo "❌ Trunk not found. Install with: cargo install trunk"
    exit 1
fi
TRUNK_VERSION=$(trunk --version)
echo "  Found: $TRUNK_VERSION"
echo ""

# Check project structure
echo "✓ Checking project structure..."
REQUIRED_DIRS=(
    "crates/shared/src"
    "crates/backend/src"
    "crates/frontend/src"
    "crates/charting/src"
)

for dir in "${REQUIRED_DIRS[@]}"; do
    if [ ! -d "$dir" ]; then
        echo "❌ Missing directory: $dir"
        exit 1
    fi
done
echo "  All required directories present ✓"
echo ""

# Check key files
echo "✓ Checking key files..."
REQUIRED_FILES=(
    "Cargo.toml"
    "crates/shared/Cargo.toml"
    "crates/backend/Cargo.toml"
    "crates/frontend/Cargo.toml"
    "crates/charting/Cargo.toml"
    ".env.example"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ Missing file: $file"
        exit 1
    fi
done
echo "  All required files present ✓"
echo ""

# Try to build the workspace
echo "✓ Attempting to build workspace..."
if cargo check --workspace --quiet; then
    echo "  Workspace builds successfully ✓"
else
    echo "❌ Workspace failed to build. Run 'cargo check' for details."
    exit 1
fi
echo ""

# Check .env file
echo "✓ Checking environment configuration..."
if [ ! -f ".env" ]; then
    echo "⚠️  .env file not found. Creating from .env.example..."
    cp .env.example .env
    echo "  Created .env file ✓"
else
    echo "  .env file exists ✓"
fi
echo ""

echo "========================================"
echo "✅ Setup verification complete!"
echo ""
echo "Next steps:"
echo "  1. Start backend:  cd crates/backend && cargo run"
echo "  2. Start frontend: cd crates/frontend && trunk serve"
echo ""
