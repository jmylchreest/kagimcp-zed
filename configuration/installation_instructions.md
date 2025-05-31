## Installation Instructions

### Prerequisites

1. **Install uv** (Python package manager):
   - **macOS/Linux**: `curl -LsSf https://astral.sh/uv/install.sh | sh`
   - **Windows**: `powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"`

2. **Get Kagi API Access**: 
   - The Kagi Search API is currently in closed beta
   - Request access by emailing support@kagi.com
   - Once approved, get your API key from [Kagi Settings](https://kagi.com/settings?p=api)

### Setup

1. Install the Kagi MCP server globally:
   ```bash
   uvx kagimcp
   ```

2. Configure your Kagi API key in Zed's settings by adding it to the context server configuration.

### Optional Configuration

- **Summarizer Engine**: You can optionally specify which Kagi summarizer engine to use (defaults to "cecil"). Available engines include "cecil", "daphne", and others. See [Kagi's documentation](https://help.kagi.com/kagi/api/summarizer.html) for more details.

### Usage

Once configured, you can use Kagi search and summarization tools directly in Zed's AI assistant. Try queries like:
- "Search for the latest news about AI"
- "Summarize this video: https://www.youtube.com/watch?v=example"