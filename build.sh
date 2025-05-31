#!/bin/bash

set -e

echo "Building Kagi MCP Extension for Zed..."
echo "Note: Consider using 'make build' for a more comprehensive build process"
echo ""

# Use make if available, otherwise fallback to cargo
if command -v make >/dev/null 2>&1; then
    echo "Using Makefile..."
    make build
    make sizes
else
    echo "Make not available, using cargo directly..."
    
    # Check if rust target is installed
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        echo "Installing wasm32-unknown-unknown target..."
        rustup target add wasm32-unknown-unknown
    fi

    # Build the MCP server binary
    echo "Building Kagi MCP server..."
    cargo build --package kagi-mcp-server --release

    # Build the extension
    echo "Building Zed extension..."
    cargo build --target wasm32-unknown-unknown --release

    echo ""
    echo "Build complete!"
    echo "  Extension: target/wasm32-unknown-unknown/release/kagimcp_zed.wasm"
    echo "  MCP Server: target/release/kagi-mcp-server"

    echo ""
    echo "Extension size:"
    ls -lh target/wasm32-unknown-unknown/release/kagimcp_zed.wasm

    echo ""
    echo "MCP Server size:"
    ls -lh target/release/kagi-mcp-server
fi

echo ""
echo "To install extension in Zed:"
echo "  make install-extension"
echo ""
echo "To test the MCP server:"
echo "  KAGI_API_KEY=your_key make run-mcp-server"
echo ""
echo "To create a release:"
echo "  make snapshot    # local testing"
echo "  git tag v0.1.0 && git push origin v0.1.0  # automated release"