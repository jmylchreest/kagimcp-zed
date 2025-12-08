# Kagi MCP Server

Kagi Search and Universal Summarizer integration through the Model Context Protocol (MCP). Works with Claude Code, Claude Desktop, Cursor, Zed, and any MCP-compatible client.

## Prerequisites

Get Kagi API access:
1. Request API access by emailing support@kagi.com (currently in closed beta)
2. Get your API key from [Kagi Settings](https://kagi.com/settings?p=api)

## Quick Start

### Zed Editor

1. Install the extension: Extensions → Search "Kagi MCP Server" → Install
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
claude mcp add -s user kagi -e KAGI_API_KEY="<YOUR-KEY-HERE>" -- npx -y kagi-mcp-server@latest
```

### Claude Desktop / Cursor / Other MCP Clients

See the [NPM package documentation](contrib/npm/README.md) for detailed configuration examples for all supported clients.

## Available Tools

| Tool | Description |
|------|-------------|
| `kagi_search` | Web search using Kagi's privacy-focused search API |
| `kagi_fastgpt` | Quick AI-powered answers with automatic web search |
| `kagi_enrich_web` | Find non-commercial "small web" content and discussions |
| `kagi_enrich_news` | Find non-mainstream news sources and alternative perspectives |
| `kagi_summarize` | Summarize web pages, documents, PDFs, and videos |

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

## Troubleshooting

**API errors?**
- Ensure you have Kagi Search API access (closed beta)
- Double-check your API key

**Server not responding?**
- Test manually: `KAGI_API_KEY=your-key npx -y kagi-mcp-server@latest --help`
- Check your client's logs for error messages

## Links

- [NPM Package & Client Configuration](contrib/npm/README.md)
- [Kagi Search](https://kagi.com)
- [Kagi API Docs](https://help.kagi.com/kagi/api/)
- [Original Python MCP Server](https://github.com/kagisearch/kagimcp)

## License

MIT License - see [LICENSE](LICENSE)