#!/bin/bash

set -e

echo "Building Kagi MCP Extension for Zed..."

# Check if rust target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Run clippy for linting
echo "Running clippy..."
cargo clippy --target wasm32-unknown-unknown -- -D warnings

# Build the extension
echo "Building extension..."
cargo build --target wasm32-unknown-unknown --release

echo "Build complete! Extension built to:"
echo "  target/wasm32-unknown-unknown/release/kagimcp.wasm"

# Check if extension builds properly
echo "Extension size:"
ls -lh target/wasm32-unknown-unknown/release/kagimcp.wasm

echo ""
echo "To test the extension:"
echo "1. Open Zed"
echo "2. Go to Extensions (Cmd+Shift+X)"
echo "3. Click 'Install Dev Extension'"
echo "4. Select this directory: $(pwd)"