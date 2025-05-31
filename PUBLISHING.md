# Publishing Guide for Kagi MCP Extension

This guide covers how to publish the Kagi MCP Server extension to the Zed marketplace.

## Prerequisites

1. **Fork the zed-industries/extensions repository** to your personal GitHub account (not an organization)
2. **Test the extension locally** to ensure it works properly
3. **Ensure all tests pass** in CI/CD

## Publishing Steps

### 1. Prepare the Extension

Ensure your extension is ready:

```bash
# Build and test the extension
./build.sh

# Verify the extension works locally in Zed:
# 1. Open Zed
# 2. Go to Extensions (Cmd+Shift+X or Ctrl+Shift+X)
# 3. Click "Install Dev Extension"  
# 4. Select this directory
# 5. Test with a Kagi API key
```

### 2. Fork the Extensions Repository

1. Go to https://github.com/zed-industries/extensions
2. Click "Fork" and fork to your personal GitHub account
3. Clone your fork locally:

```bash
git clone https://github.com/YOUR_USERNAME/extensions.git
cd extensions
```

### 3. Add Your Extension as a Submodule

```bash
# Add your extension as a git submodule
git submodule add https://github.com/jmylchreest/kagimcp-zed.git extensions/kagimcp

# Add the submodule to git
git add extensions/kagimcp
```

### 4. Update extensions.toml

Add your extension entry to the top of `extensions.toml`:

```toml
[kagimcp]
submodule = "extensions/kagimcp"
version = "0.1.0"
```

### 5. Sort Extensions

```bash
# Install pnpm if you don't have it
npm install -g pnpm

# Sort the extensions file
pnpm sort-extensions
```

### 6. Create Pull Request

```bash
# Commit your changes
git add extensions.toml .gitmodules
git commit -m "Add Kagi MCP Server extension

- Integrates Kagi search and summarization via MCP
- Requires Kagi API key and uv/uvx
- Supports configurable summarizer engines"

# Push to your fork
git push origin main
```

Then create a pull request from your fork to `zed-industries/extensions`.

## Pull Request Guidelines

### PR Title
```
Add Kagi MCP Server extension
```

### PR Description
```markdown
## Extension Details

- **Name**: Kagi MCP Server
- **ID**: kagimcp
- **Version**: 0.1.0
- **Description**: Model Context Protocol Server for Kagi Search

## Features

- Kagi web search integration
- Content summarization (web pages, videos, documents)
- Configurable summarizer engines
- Privacy-focused search via Kagi

## Requirements

- Kagi API key (currently in closed beta - request access at support@kagi.com)
- uv package manager installed
- kagimcp Python package (installed via uvx)

## Testing

- [x] Extension builds successfully
- [x] CI/CD passes
- [x] Tested locally with dev extension installation
- [x] Confirmed MCP server integration works

## Repository

https://github.com/jmylchreest/kagimcp-zed
```

## After Publishing

Once your PR is merged:

1. **The extension will be automatically packaged** and published to the Zed extension registry
2. **Users can install it** via Zed's Extensions panel
3. **Monitor for issues** and respond to user feedback

## Updating the Extension

To update the extension later:

1. **Make changes** to your extension repository
2. **Update the version** in `extension.toml`
3. **Create a new PR** to `zed-industries/extensions`:
   - Update the submodule to point to the new commit
   - Update the version in `extensions.toml`
   - Ensure versions match between your `extension.toml` and `extensions.toml`

## Tips

- **Use descriptive commit messages** when updating
- **Test thoroughly** before submitting PRs
- **Keep extension.toml version in sync** with extensions.toml version
- **Respond promptly** to feedback from Zed maintainers
- **Consider using the GitHub Action** for automated updates (see community actions)

## Troubleshooting

### Common Issues

1. **Submodule not found**: Ensure the repository URL is correct and accessible
2. **Version mismatch**: Double-check versions in both `extension.toml` and `extensions.toml`
3. **Build failures**: Ensure the extension builds on the target platform
4. **Sort errors**: Run `pnpm sort-extensions` to fix ordering

### Getting Help

- Check the [Zed extensions documentation](https://zed.dev/docs/extensions/developing-extensions)
- Look at other extensions in the repository for examples
- Ask questions in the Zed community Discord or GitHub discussions