.PHONY: help build test clean release snapshot install-deps

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install-deps: ## Install development dependencies
	rustup default stable
	rustup target add wasm32-unknown-unknown
	@echo "Note: cargo-zigbuild is only needed for GoReleaser cross-compilation"
	@echo "For local development, regular cargo is sufficient"

build: ## Build all components
	cargo build --package kagiapi
	cargo build --package kagi-mcp-server --release
	cargo build --target wasm32-unknown-unknown --release

test: ## Run tests and checks
	cargo fmt -- --check
	cargo clippy --workspace -- -D warnings
	cargo clippy --target wasm32-unknown-unknown -- -D warnings
	cargo test --workspace

clean: ## Clean build artifacts
	cargo clean
	rm -rf dist/

dev: ## Build for development (faster builds)
	cargo build --package kagi-mcp-server
	cargo build --target wasm32-unknown-unknown

snapshot: ## Create a snapshot release (local testing) - requires cargo-zigbuild and zig
	@echo "Creating snapshot with GoReleaser..."
	@echo "Note: This requires cargo-zigbuild and zig for cross-compilation"
	goreleaser release --snapshot --clean --skip=publish

release: ## Create a tagged release (requires git tag) - for CI use
	@echo "This should typically be run by CI/CD on tagged releases"
	goreleaser release --clean

check-goreleaser: ## Validate GoReleaser configuration
	goreleaser check

install-goreleaser: ## Install GoReleaser
	go install github.com/goreleaser/goreleaser@latest

install-cross-tools: ## Install cross-compilation tools (for GoReleaser)
	cargo install --locked cargo-zigbuild
	@echo "Also install Zig from: https://ziglang.org/download/"
	@echo "Or use the setup-zig action in CI"

extension-size: ## Show extension size
	@echo "Extension size:"
	@ls -lh target/wasm32-unknown-unknown/release/kagimcp_zed.wasm 2>/dev/null || echo "Extension not built yet - run 'make build' first"

mcp-server-size: ## Show MCP server size
	@echo "MCP server size:"
	@ls -lh target/release/kagi-mcp-server 2>/dev/null || echo "MCP server not built yet - run 'make build' first"

sizes: extension-size mcp-server-size ## Show sizes of both components

run-mcp-server: ## Run MCP server locally (requires KAGI_API_KEY)
	@if [ -z "$(KAGI_API_KEY)" ]; then echo "Error: KAGI_API_KEY environment variable required"; exit 1; fi
	./target/release/kagi-mcp-server

install-extension: build ## Install extension in Zed for development
	@echo "To install the extension in Zed:"
	@echo "1. Open Zed"
	@echo "2. Go to Extensions (Cmd+Shift+X)"
	@echo "3. Click 'Install Dev Extension'"
	@echo "4. Select this directory: $(PWD)"