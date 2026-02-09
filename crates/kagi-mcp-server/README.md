# kagi-mcp-server

A Model Context Protocol (MCP) server that provides Kagi search, summarization, FastGPT, and enrichment tools to AI assistants.

Built with the official [rmcp](https://github.com/modelcontextprotocol/rust-sdk) Rust MCP SDK and the [kagiapi](https://crates.io/crates/kagiapi) client library.

## Installation

### From crates.io

```bash
cargo install kagi-mcp-server
```

### From npm (downloads pre-built binary)

```bash
npx -y kagi-mcp-server@latest
```

### From source

```bash
git clone https://github.com/jmylchreest/kagimcp-zed
cd kagimcp-zed
cargo install --path crates/kagi-mcp-server
```

## Available Tools

| Tool | Description |
|------|-------------|
| `kagi_search_fetch` | Web search using Kagi's privacy-focused search API |
| `kagi_fastgpt` | AI-powered answers with automatic web search and citations |
| `kagi_enrich_web` | Find non-commercial "small web" content and discussions |
| `kagi_enrich_news` | Find non-mainstream news sources and alternative perspectives |
| `kagi_summarizer` | Summarize web pages, documents, PDFs, videos, and audio |

## MCP Client Configuration

### Claude Code

```bash
claude mcp add -s user kagi -e KAGI_API_KEY="YOUR_KEY" -- kagi-mcp-server
```

Or using npx (no Rust toolchain needed):

```bash
claude mcp add -s user kagi -e KAGI_API_KEY="YOUR_KEY" -- npx -y kagi-mcp-server@latest
```

### OpenCode

Add to your `opencode.json`:

```json
{
  "mcp": {
    "kagi": {
      "command": "kagi-mcp-server",
      "env": {
        "KAGI_API_KEY": "YOUR_KEY"
      }
    }
  }
}
```

### Claude Desktop

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "kagi": {
      "command": "kagi-mcp-server",
      "env": {
        "KAGI_API_KEY": "YOUR_KEY"
      }
    }
  }
}
```

Or with npx:

```json
{
  "mcpServers": {
    "kagi": {
      "command": "npx",
      "args": ["-y", "kagi-mcp-server@latest"],
      "env": {
        "KAGI_API_KEY": "YOUR_KEY"
      }
    }
  }
}
```

### Cursor

Add to Cursor MCP settings:

```json
{
  "mcpServers": {
    "kagi": {
      "command": "kagi-mcp-server",
      "env": {
        "KAGI_API_KEY": "YOUR_KEY"
      }
    }
  }
}
```

### Zed

Install the "Kagi MCP Server" extension from the Zed extension marketplace, then add to your Zed settings:

```json
{
  "context_servers": {
    "kagimcp": {
      "settings": {
        "kagi_api_key": "YOUR_KEY",
        "kagi_summarizer_engine": "cecil"
      }
    }
  }
}
```

## CLI Options

```
kagi-mcp-server [OPTIONS]

Options:
      --api-key <API_KEY>                        Kagi API key [env: KAGI_API_KEY]
      --summarizer-engine <SUMMARIZER_ENGINE>     Default summarizer engine [env: KAGI_SUMMARIZER_ENGINE] [default: cecil]
      --search-api-version <VERSION>              API version for search [env: KAGI_SEARCH_API_VERSION] [default: v0]
      --summarizer-api-version <VERSION>          API version for summarizer [env: KAGI_SUMMARIZER_API_VERSION] [default: v0]
      --fastgpt-api-version <VERSION>             API version for FastGPT [env: KAGI_FASTGPT_API_VERSION] [default: v0]
      --enrich-api-version <VERSION>              API version for enrichment [env: KAGI_ENRICH_API_VERSION] [default: v0]
  -h, --help                                      Print help
  -V, --version                                   Print version
```

All options can also be set via environment variables.

## Prerequisites

- **API Key**: Get your API key from [Kagi Settings](https://kagi.com/settings?p=api)
- **API Access**: The Search API is currently in closed beta. Request access by emailing support@kagi.com

## License

MIT License - see [LICENSE](../../LICENSE) for details.
