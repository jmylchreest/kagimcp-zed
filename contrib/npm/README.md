# Kagi MCP Server - NPX Wrapper

NPX wrapper for the [Kagi MCP Server](https://github.com/jmylchreest/kagimcp-zed) - run the Kagi Search and Summarizer MCP server without manual binary downloads.

## Features

- 🚀 **Zero Config**: Just run `npx kagi-mcp-server` - no installation needed
- 📦 **Auto-Download**: Automatically downloads the correct binary for your platform
- 💾 **Cached**: Binary is cached after first download for fast subsequent runs
- 🔄 **Cross-Platform**: Works on macOS (Intel & Apple Silicon), Linux, and Windows
- 🔒 **Privacy-First**: Built on Kagi's privacy-focused search infrastructure

## Prerequisites

- **Node.js 16+**: Required for NPX execution
- **Kagi API Key**: Get one from [Kagi Settings](https://kagi.com/settings?p=api) (requires API access - currently in closed beta, email support@kagi.com to request access)

## Quick Start

```bash
# Run with environment variable
KAGI_API_KEY=your-key npx -y kagi-mcp-server

# Or with command line argument
npx -y kagi-mcp-server --api-key your-key

# Show help
npx -y kagi-mcp-server --help
```

> **Note**: The `-y` flag auto-confirms the npx install prompt, which is required for non-interactive use (like MCP clients).

## MCP Client Configuration

### Claude Desktop

Add this to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
**Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "kagi": {
      "command": "npx",
      "args": ["-y", "kagi-mcp-server"],
      "env": {
        "KAGI_API_KEY": "your-api-key-here"
      }
    }
  }
}
```

#### With Custom Summarizer Engine

```json
{
  "mcpServers": {
    "kagi": {
      "command": "npx",
      "args": [
        "-y",
        "kagi-mcp-server",
        "--summarizer-engine",
        "muriel"
      ],
      "env": {
        "KAGI_API_KEY": "your-api-key-here"
      }
    }
  }
}
```

### Cursor

Add to your Cursor MCP settings (Settings → MCP):

```json
{
  "mcpServers": {
    "kagi": {
      "command": "npx",
      "args": ["-y", "kagi-mcp-server"],
      "env": {
        "KAGI_API_KEY": "your-api-key-here"
      }
    }
  }
}
```

### VS Code with Continue.dev

In your Continue configuration (`~/.continue/config.json`):

```json
{
  "experimental": {
    "modelContextProtocolServers": [
      {
        "transport": {
          "type": "stdio",
          "command": "npx",
          "args": ["-y", "kagi-mcp-server"],
          "env": {
            "KAGI_API_KEY": "your-api-key-here"
          }
        }
      }
    ]
  }
}
```

### Zed Editor

For Zed, we recommend using the native Zed extension instead of this npx wrapper. Install "Kagi MCP Server" from the Zed extension marketplace.

However, if you prefer the npx approach, add to your Zed settings:

```json
{
  "context_servers": {
    "kagi": {
      "command": {
        "path": "npx",
        "args": ["-y", "kagi-mcp-server"],
        "env": {
          "KAGI_API_KEY": "your-api-key-here"
        }
      }
    }
  }
}
```

### Generic MCP Client Configuration

For any MCP-compatible client, use these settings:

| Setting | Value |
|---------|-------|
| Transport | `stdio` |
| Command | `npx` |
| Arguments | `-y`, `kagi-mcp-server` |
| Environment | `KAGI_API_KEY=your-api-key` |

## Available Tools

Once connected, the Kagi MCP Server provides these tools to AI assistants:

### 🔍 Search Tools

| Tool | Description |
|------|-------------|
| `kagi_search` | Web search using Kagi's privacy-focused search API |
| `kagi_fastgpt` | Quick AI-powered answers with automatic web search |
| `kagi_enrich_web` | Find non-commercial "small web" content and discussions |
| `kagi_enrich_news` | Find non-mainstream news sources and alternative perspectives |

### 📄 Summarization

| Tool | Description |
|------|-------------|
| `kagi_summarize` | Summarize web pages, documents, PDFs, and videos |

## Command Line Options

```
Options:
  --api-key KEY              Kagi API key (or set KAGI_API_KEY env var)
  --summarizer-engine ENGINE Summarizer engine to use (default: cecil)
  --help                     Show help message
  --wrapper-help             Show npx wrapper help

Summarizer Engines:
  cecil   - Friendly, conversational summaries (default)
  agnes   - Formal, professional tone
  daphne  - Detailed, comprehensive summaries
  muriel  - Concise, brief summaries
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `KAGI_API_KEY` | Yes | Your Kagi API key |

### Security Best Practices

- **Never commit your API key** to version control
- Use environment variables or secure secrets management
- For CI/CD, use encrypted secrets (GitHub Secrets, etc.)
- Consider using a `.env` file locally (and add it to `.gitignore`)

## How It Works

1. On first run, the wrapper downloads the correct pre-built binary for your platform from GitHub releases
2. The binary is cached in `~/.cache/kagi-mcp-server/`
3. Subsequent runs use the cached binary directly
4. The wrapper passes all arguments and environment variables through to the native binary
5. All MCP communication happens over stdio (stdin/stdout)

## Supported Platforms

| OS | Architecture | Binary |
|----|--------------|--------|
| macOS | x86_64 (Intel) | ✅ |
| macOS | arm64 (Apple Silicon) | ✅ |
| Linux | x86_64 | ✅ |
| Linux | arm64 | ✅ |
| Windows | x86_64 | ✅ |

## Troubleshooting

### Binary Download Fails

```bash
# Clear the cache and try again
rm -rf ~/.cache/kagi-mcp-server
npx -y kagi-mcp-server
```

### Permission Denied (Unix)

```bash
chmod -R u+w ~/.cache/kagi-mcp-server
```

### Server Not Responding in Claude/Cursor

1. Check that your API key is valid at [Kagi Settings](https://kagi.com/settings?p=api)
2. Verify you have Kagi Search API access (closed beta)
3. Test the server manually first:
   ```bash
   KAGI_API_KEY=your-key npx -y kagi-mcp-server --help
   ```
4. Check the client's logs for error messages

### NPX Cache Issues

```bash
# Clear npx cache
npx clear-npx-cache

# Or force a fresh download
npx -y kagi-mcp-server@latest
```

### Windows-Specific Issues

- Ensure Node.js is in your PATH
- Try running from PowerShell instead of CMD
- Check Windows Defender isn't blocking the binary

## Publishing to npm (For Maintainers)

This package is automatically published to npm when a new release is tagged in the repository. The GitHub Actions workflow handles:

1. Building platform-specific binaries
2. Creating a GitHub release with the binaries
3. Publishing the npm package with the matching version

### Initial Setup (One-Time)

Follow these steps to enable automatic npm publishing:

#### Step 1: Create an npm Account

If you don't have one, create an account at [npmjs.com/signup](https://www.npmjs.com/signup)

#### Step 2: Check Package Name Availability

```bash
npm view kagi-mcp-server
```

If it returns a 404, the name is available. If it's taken, you'll need to either:
- Use a scoped name: change `"name"` in `package.json` to `"@yourusername/kagi-mcp-server"`
- Pick a different name like `kagimcp-server`

#### Step 3: Generate an npm Automation Token

1. Go to [npmjs.com](https://www.npmjs.com) and sign in
2. Click your avatar (top right) → **Access Tokens**
3. Click **Generate New Token** → select **Automation**
4. Copy the token (it starts with `npm_...`) - you won't see it again!

#### Step 4: Add the Token to GitHub Secrets

1. Go to your GitHub repository: `https://github.com/jmylchreest/kagimcp-zed`
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Name: `NPM_TOKEN`
5. Value: Paste your npm automation token
6. Click **Add secret**

#### Step 5: Publish

**Option A: Automatic (recommended)**

Just tag a new release - the workflow handles everything:

```bash
git tag v0.0.31
git push origin v0.0.31
```

The workflow will:
1. Build binaries for all platforms
2. Create a GitHub release
3. Publish the Zed extension
4. Publish to npm with the matching version

**Option B: Manual (first time or testing)**

```bash
cd contrib/npm

# Login to npm (first time only)
npm login

# Update version to match the release
npm version 0.0.31 --no-git-tag-version

# Publish publicly
npm publish --access public
```

### Package Naming

The package is published as `kagi-mcp-server`. If you fork this project:
- Change the `name` field in `package.json`
- Consider using a scoped name: `@yourusername/kagi-mcp-server`

## Development

This wrapper is part of the [kagimcp-zed](https://github.com/jmylchreest/kagimcp-zed) project.

### Building the Native Binary from Source

```bash
# Clone the repository
git clone https://github.com/jmylchreest/kagimcp-zed.git
cd kagimcp-zed

# Build the native binary
cargo build --package kagi-mcp-server --release

# Run directly (without npx wrapper)
KAGI_API_KEY=your-key ./target/release/kagi-mcp-server
```

### Testing the NPX Wrapper Locally

```bash
cd contrib/npm

# Run tests
npm test

# Test the wrapper locally
node bin/kagi-mcp-server.js --help
```

### Version Synchronization

The npm package version should match the Cargo workspace version. The release workflow handles this automatically, but for manual releases:

1. Update `Cargo.toml` version
2. Update `contrib/npm/package.json` version
3. Tag and push: `git tag v0.0.31 && git push origin v0.0.31`

## Links

- **Repository**: [github.com/jmylchreest/kagimcp-zed](https://github.com/jmylchreest/kagimcp-zed)
- **npm Package**: [npmjs.com/package/kagi-mcp-server](https://www.npmjs.com/package/kagi-mcp-server)
- **Kagi Search**: [kagi.com](https://kagi.com)
- **Kagi API Docs**: [help.kagi.com/kagi/api](https://help.kagi.com/kagi/api/)
- **MCP Protocol**: [modelcontextprotocol.io](https://modelcontextprotocol.io)

## License

MIT License - see [LICENSE](../../LICENSE)