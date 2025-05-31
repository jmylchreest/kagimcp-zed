# Kagi MCP Server Extension for Zed

A Zed extension that integrates [Kagi's MCP Server](https://github.com/kagisearch/kagimcp) to provide AI-powered search and summarization capabilities directly in Zed's AI Assistant.

## Features

- **🔍 Kagi Search**: Access Kagi's privacy-focused search engine
- **📄 Content Summarization**: Summarize web pages, documents, and videos  
- **⚙️ Configurable Engines**: Choose from multiple Kagi summarization engines
- **🔒 Privacy-First**: Built on Kagi's privacy-focused infrastructure

## Quick Start

### 1. Install Dependencies

**Install uv (Python package manager):**
- **macOS/Linux**: `curl -LsSf https://astral.sh/uv/install.sh | sh`
- **Windows**: `powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"`

**Install Kagi MCP server:**
```bash
uvx kagimcp
```

### 2. Get Kagi API Access

1. **Request API access**: Email support@kagi.com (currently in closed beta)
2. **Get your API key**: Visit [Kagi Settings](https://kagi.com/settings?p=api)

### 3. Install & Configure Extension

1. **Install in Zed**: Extensions → Search "Kagi MCP Server" → Install
2. **Configure**: Add to your Zed settings:

```json
{
  "context_servers": {
    "kagimcp": {
      "settings": {
        "kagi_api_key": "YOUR_KAGI_API_KEY_HERE"
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
1. Check `uv` is installed: `uv --version`
2. Verify MCP server: `uvx kagimcp --help`  
3. Confirm API key is valid and has permissions

**API errors?**
- Ensure you have Kagi Search API access (closed beta)
- Double-check your API key in settings

## Development

```bash
# Clone and build
git clone https://github.com/jmylchreest/kagimcp-zed.git
cd kagimcp-zed
./build.sh

# Test locally in Zed
# Extensions → Install Dev Extension → Select this directory
```

## Links

- **Original MCP Server**: [kagimcp](https://github.com/kagisearch/kagimcp)
- **Kagi Search**: [kagi.com](https://kagi.com)
- **Zed Extensions**: [zed.dev/docs/extensions](https://zed.dev/docs/extensions)

## License

MIT License - see [LICENSE](LICENSE)