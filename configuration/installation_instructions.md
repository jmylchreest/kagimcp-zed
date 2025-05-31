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

- **Summarizer Engine**: You can optionally specify which Kagi summarizer engine to use (defaults to "cecil"). Available engines include "cecil", "agnes", "daphne", and "muriel". See [Kagi's documentation](https://help.kagi.com/kagi/api/summarizer.html) for more details.

### Usage

Once configured, you can use Kagi search and summarization tools directly in Zed's AI assistant. Try queries like:
- "Search for the latest news about AI"
- "Summarize this video: https://www.youtube.com/watch?v=example"

**Note**: The MCP server binary is automatically downloaded and managed by the extension. No manual installation of Python, uv, or other dependencies is required.