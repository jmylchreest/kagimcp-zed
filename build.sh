#!/bin/bash

set -e

echo "Building Kagi MCP Extension for Zed..."

# Check if rust target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Run clippy for linting
echo "Running clippy on workspace..."
cargo clippy --workspace -- -D warnings

# Run clippy for extension
echo "Running clippy on extension..."
cargo clippy --target wasm32-unknown-unknown -- -D warnings

# Build the MCP server binary
echo "Building Kagi MCP server..."
cargo build --package kagi-mcp-server --release

# Build the extension
echo "Building Zed extension..."
cargo build --target wasm32-unknown-unknown --release

echo "Build complete!"
echo "  Extension: target/wasm32-unknown-unknown/release/kagimcp_zed.wasm"
echo "  MCP Server: target/release/kagi-mcp-server"

# Check if extension builds properly
echo ""
echo "Extension size:"
ls -lh target/wasm32-unknown-unknown/release/kagimcp_zed.wasm

echo ""
echo "MCP Server size:"
ls -lh target/release/kagi-mcp-server

echo ""
echo "To test the extension:"
echo "1. Open Zed"
echo "2. Go to Extensions (Cmd+Shift+X)"
echo "3. Click 'Install Dev Extension'"
echo "4. Select this directory: $(pwd)"
echo ""
echo "To test the MCP server:"
echo "  KAGI_API_KEY=your_key ./target/release/kagi-mcp-server"