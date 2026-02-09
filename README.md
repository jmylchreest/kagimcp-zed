# Kagi MCP Server

Kagi Search and Universal Summarizer integration through the Model Context Protocol (MCP). Works with Claude Code, Claude Desktop, Cursor, Zed, and any MCP-compatible client.

This repository provides:

- **[`kagiapi`](crates/kagiapi/)** - Standalone Rust client library for Kagi's APIs ([crates.io](https://crates.io/crates/kagiapi))
- **[`kagi-mcp-server`](crates/kagi-mcp-server/)** - MCP server binary exposing Kagi tools to AI assistants ([crates.io](https://crates.io/crates/kagi-mcp-server))
- **Zed extension** - Direct integration with the Zed editor via the extension marketplace

## Prerequisites

Get Kagi API access:
1. Request API access by emailing support@kagi.com (currently in closed beta)
2. Get your API key from [Kagi Settings](https://kagi.com/settings?p=api)

## Quick Start

### Install via cargo

```bash
cargo install kagi-mcp-server
```

### Install via npm (pre-built binaries, no Rust toolchain needed)

```bash
npx -y kagi-mcp-server@latest
```

### Zed Editor

1. Install the extension: Extensions > Search "Kagi MCP Server" > Install
2. Add to your Zed settings:

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

### Claude Code (CLI)

```bash
claude mcp add -s user kagi -e KAGI_API_KEY="<YOUR-KEY-HERE>" -- kagi-mcp-server
```

### Claude Desktop / Cursor / Other MCP Clients

See the [kagi-mcp-server documentation](crates/kagi-mcp-server/README.md) for detailed configuration examples for all supported clients.

## Available Tools

| Tool | Description |
|------|-------------|
| `kagi_search_fetch` | Web search using Kagi's privacy-focused search API |
| `kagi_fastgpt` | Quick AI-powered answers with automatic web search |
| `kagi_enrich_web` | Find non-commercial "small web" content and discussions |
| `kagi_enrich_news` | Find non-mainstream news sources and alternative perspectives |
| `kagi_summarizer` | Summarize web pages, documents, PDFs, and videos |

## Configuration Options

| Option | Required | Description |
|--------|----------|-------------|
| `KAGI_API_KEY` | Yes | Your Kagi API key |
| `--summarizer-engine` | No | Summarizer engine: `cecil` (default), `agnes`, `daphne`, or `muriel` |

## Usage Examples

Ask your AI assistant:

- "Search for the latest AI safety research"
- "Find recent climate change news"
- "Summarize this article: https://example.com/article"
- "Summarize this YouTube video: https://youtube.com/watch?v=..."

## Using the Kagi API Library

If you want to use Kagi's APIs in your own Rust project:

```toml
[dependencies]
kagiapi = "0.0.30"
```

```rust
use kagiapi::KagiClient;

#[tokio::main]
async fn main() -> Result<(), kagiapi::Error> {
    let client = KagiClient::new("your-api-key");
    let results = client.search("rust programming", Some(10)).await?;
    Ok(())
}
```

See the [kagiapi documentation](crates/kagiapi/README.md) for full API reference.

## Troubleshooting

**API errors?**
- Ensure you have Kagi Search API access (closed beta)
- Double-check your API key

**Server not responding?**
- Test manually: `KAGI_API_KEY=your-key kagi-mcp-server --help`
- Check your client's logs for error messages

## Links

- [kagi-mcp-server on crates.io](https://crates.io/crates/kagi-mcp-server)
- [kagiapi on crates.io](https://crates.io/crates/kagiapi)
- [NPM Package & Client Configuration](contrib/npm/README.md)
- [Kagi Search](https://kagi.com)
- [Kagi API Docs](https://help.kagi.com/kagi/api/)

## License

MIT License - see [LICENSE](LICENSE)
