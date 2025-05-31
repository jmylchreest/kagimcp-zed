# Kagi MCP Extension for Zed

A pure Rust implementation providing Kagi Search and Universal Summarizer integration for Zed's AI Assistant through the Model Context Protocol (MCP).

## Features

- **🔍 Kagi Search**: Access Kagi's privacy-focused search engine
- **📄 Content Summarization**: Summarize web pages, documents, and videos  
- **⚙️ Configurable Engines**: Choose from multiple Kagi summarization engines
- **🔒 Privacy-First**: Built on Kagi's privacy-focused infrastructure
- **🦀 Pure Rust**: No Python dependencies, native performance
- **📦 Modular**: Reusable libraries for the broader ecosystem

## Architecture

This project is structured as a Cargo workspace with focused, lightweight crates:

### 📚 **Components**
- **`kagiapi`** - Pure Rust client for Kagi's APIs (search, summarizer)
- **`kagi-mcp-server`** - Lightweight MCP server implementation (400 LOC)
- **`kagimcp-zed`** - Zed extension (WebAssembly)

### 🎯 **Benefits**
- **Simple**: Custom MCP implementation that's easy to understand and modify
- **Type-safe**: Strongly typed APIs with comprehensive error handling
- **Performance**: Native Rust performance, no Python overhead
- **Maintainable**: Focused codebase without complex external dependencies
</edits>

## Quick Start

### 1. Prerequisites

**Get Kagi API Access**:
1. Request API access by emailing support@kagi.com (currently in closed beta)
2. Get your API key from [Kagi Settings](https://kagi.com/settings?p=api)

### 2. Install & Configure Extension

1. **Install in Zed**: Extensions → Search "Kagi MCP Server" → Install
2. **Automatic Setup**: The MCP server binary is automatically downloaded on first use
3. **Configure**: Add to your Zed settings:

```json
{
  "context_servers": {
    "kagimcp": {
      "settings": {
        "kagi_api_key": "YOUR_KAGI_API_KEY_HERE",
        "kagi_summarizer_engine": "cecil"
      }
    }
  }
}
```

## Usage Examples

Ask Zed's AI Assistant:

**🔍 Search:**
- "Search for the latest AI safety research"
- "Find recent climate change news"
- "Search for Rust async programming guides"

**📄 Summarize:**
- "Summarize this article: https://example.com/article"
- "Summarize this YouTube video: https://youtube.com/watch?v=..."
- "Give me a summary of this paper: https://arxiv.org/abs/..."

## Configuration

### Required
- `kagi_api_key`: Your Kagi API key

### Optional
- `kagi_summarizer_engine`: Engine to use (default: "cecil")
  - Options: "cecil", "daphne", etc.
  - See [Kagi docs](https://help.kagi.com/kagi/api/summarizer.html)

### Example Configuration
```json
{
  "context_servers": {
    "kagimcp": {
      "settings": {
        "kagi_api_key": "your-api-key-here",
        "kagi_summarizer_engine": "cecil"
      }
    }
  }
}
```

## Troubleshooting

**Extension not working?**
1. Check that the MCP server binary downloaded successfully (check Zed logs)
2. Confirm API key is valid and has permissions
3. Ensure you have internet access for initial binary download

**API errors?**
- Ensure you have Kagi Search API access (closed beta)
- Double-check your API key in settings

**Binary download issues?**
- The extension automatically downloads the MCP server binary from GitHub releases
- If download fails, check your internet connection and GitHub access
- Binaries are cached locally for offline use after first download

## Development

### Prerequisites

```bash
# Install dependencies
make install-deps
# or manually:
rustup default stable
rustup target add wasm32-unknown-unknown
cargo install --locked cargo-zigbuild
go install github.com/goreleaser/goreleaser@latest
```

### Local Development

```bash
# Clone and build
git clone https://github.com/jmylchreest/kagimcp-zed.git
cd kagimcp-zed

# Build all components
make build

# Development build (faster)
make dev

# Run tests
make test

# Check component sizes
make sizes
```

### Release Management

```bash
# Create snapshot (local testing)
make snapshot

# Create tagged release
git tag v0.1.0
git push origin v0.1.0
# GitHub Actions will automatically build and release

# Test MCP server locally
KAGI_API_KEY=your_key make run-mcp-server

# Install extension for development
make install-extension
```

## Crate Documentation

### 🔧 **kagiapi** 
Pure Rust client for Kagi's APIs with async/await support.

```rust
use kagiapi::{KagiClient, SummarizerEngine, SummaryType};

let client = KagiClient::new("your-api-key");
let results = client.search("rust programming", Some(10)).await?;
let summary = client.summarize("https://example.com", None, None, None).await?;
```

### 🔧 **kagi-mcp-server**
Lightweight MCP server specifically for Kagi integration.

```rust  
use kagi_mcp_server::KagiMcpServer;

let server = KagiMcpServer::new(api_key, engine);
server.run().await?;
```

### 📦 **Binary Usage**
Command-line MCP server with flexible configuration options.

```bash
# Run with environment variables
KAGI_API_KEY=your_key kagi-mcp-server

# Or with command line args
kagi-mcp-server --api-key your_key --summarizer-engine muriel
```

## Release Process

This project uses [GoReleaser](https://goreleaser.com/) for automated builds and releases:

### Automated Releases
1. **Tag a version**: `git tag v0.1.0 && git push origin v0.1.0`
2. **GitHub Actions** automatically builds binaries for all platforms
3. **Release created** with assets named: `kagi-mcp-server_{OS}_{ARCH}.{ext}`

### Local Snapshots
```bash
make snapshot  # Creates local test builds in dist/
```

### Cross-Platform Builds
GoReleaser automatically builds for:
- **Linux**: x86_64, ARM64
- **macOS**: x86_64 (Intel), ARM64 (Apple Silicon)  
- **Windows**: x86_64

## Links

- **Original MCP Server**: [kagimcp](https://github.com/kagisearch/kagimcp)
- **Kagi Search**: [kagi.com](https://kagi.com)
- **Zed Extensions**: [zed.dev/docs/extensions](https://zed.dev/docs/extensions)
- **GoReleaser**: [goreleaser.com](https://goreleaser.com/)

## License

MIT License - see [LICENSE](LICENSE)