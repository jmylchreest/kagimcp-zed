## Installation Instructions

### Prerequisites

**Get Kagi API Access**:
- The Kagi Search API is currently in closed beta
- Request access by emailing support@kagi.com
- Once approved, get your API key from [Kagi Settings](https://kagi.com/settings?p=api)

### Setup

1. **Install the extension**: The Kagi MCP server binary will be automatically downloaded when you first use the extension.

2. **Configure your API key**: Add your Kagi API key to Zed's settings by configuring the context server.

### Optional Configuration

- **Summarizer Engine**: You can optionally specify which Kagi summarizer engine to use (defaults to "cecil"). Available engines include "cecil", "agnes", "daphne", "muriel". See [Kagi's documentation](https://help.kagi.com/kagi/api/summarizer.html) for more details.

- **FastGPT Settings**: You can configure FastGPT behavior:
  - `kagi_fastgpt_cache`: Enable/disable caching of responses (defaults to true)
  - `kagi_fastgpt_web_search`: Enable/disable web search enrichment (defaults to true)

### Available Tools

This extension provides five tools for accessing Kagi's powerful APIs:

1. **Search (kagi_search_fetch)**: Access Kagi's premium search results
2. **Summarizer (kagi_summarizer)**: Summarize content from any URL (web pages, videos, etc.)
3. **FastGPT (kagi_fastgpt)**: Generate AI-powered answers with web search capabilities and references
4. **Web Enrichment (kagi_enrich_web)**: Discover non-commercial, "small web" content
5. **News Enrichment (kagi_enrich_news)**: Find alternative news sources and discussions

### Usage

Once configured, you can use Kagi's tools directly in Zed's AI assistant. Try queries like:
- "Search for the latest news about AI"
- "Summarize this video: https://www.youtube.com/watch?v=example"
- "Use FastGPT to explain the implications of quantum computing"
- "Find unique non-commercial websites about sustainable gardening using the web enrichment tool"
- "Discover alternative news sources covering space exploration using the news enrichment tool"

**Note**: The MCP server binary is automatically downloaded and managed by the extension. No manual installation of Python, uv, or other dependencies is required.

**Pricing**: Usage of these APIs may incur charges on your Kagi account. For current pricing information, visit:
- [Search API pricing](https://help.kagi.com/kagi/api/search.html#pricing)
- [Summarizer pricing](https://help.kagi.com/kagi/api/summarizer.html#pricing)
- [FastGPT pricing](https://help.kagi.com/kagi/api/fastgpt.html#pricing)
- [Enrichment APIs pricing](https://help.kagi.com/kagi/api/enrich.html#pricing)
