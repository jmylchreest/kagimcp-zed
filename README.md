# Kagi MCP Server Extension for Zed

This extension integrates the Kagi MCP Server as a context server for Zed's AI Assistant, providing access to Kagi's search and summarization capabilities.

## Features

- **Kagi Search**: Search the web using Kagi's high-quality search API
- **Content Summarization**: Summarize web pages, documents, and videos
- **Multiple Engines**: Choose from different Kagi summarization engines

## Prerequisites

1. **Kagi API Access**: The Kagi Search API is currently in closed beta. Request access by emailing support@kagi.com
2. **uv Package Manager**: Required to run the Kagi MCP server
   - **macOS/Linux**: `curl -LsSf https://astral.sh/uv/install.sh | sh`
   - **Windows**: `powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"`

## Installation

1. Install the extension in Zed:
   - Open **Zed** → **Extensions**
   - Search for "Kagi MCP Server"
   - Click **Install**

2. Install the Kagi MCP server:
   ```bash
   uvx kagimcp
   ```

3. Get your Kagi API key from [Kagi Settings](https://kagi.com/settings?p=api)

4. Configure the extension in Zed's settings:
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

## Usage

Once configured, you can use Kagi's capabilities in Zed's AI assistant:

### Search Examples
- "Search for the latest developments in AI safety"
- "Find recent news about climate change"
- "Search for Python async programming tutorials"

### Summarization Examples
- "Summarize this article: https://example.com/article"
- "Summarize this video: https://www.youtube.com/watch?v=example"
- "Give me a summary of this research paper: https://arxiv.org/abs/example"

## Configuration Options

### Required Settings
- `kagi_api_key`: Your Kagi API key

### Optional Settings
- `kagi_summarizer_engine`: Summarization engine to use (default: "cecil")
  - Available options: "cecil", "daphne", and others
  - See [Kagi's documentation](https://help.kagi.com/kagi/api/summarizer.html) for details

## Troubleshooting

### Extension Not Working
1. Ensure `uv` is installed and in your PATH
2. Verify the Kagi MCP server is installed: `uvx kagimcp --help`
3. Check that your API key is correct and has proper permissions

### API Errors
- Make sure you have access to the Kagi Search API (currently in closed beta)
- Verify your API key is correctly set in the settings

## Related Projects

- [kagimcp](https://github.com/kagisearch/kagimcp) - The original Kagi MCP Server
- [Kagi Search](https://kagi.com) - Privacy-focused search engine

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.