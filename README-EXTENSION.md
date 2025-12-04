# Kagi MCP Extension for Zed

## Prerequisites

Get your API key from [Kagi Settings](https://kagi.com/settings?p=api).

## Setup

1. **Install the extension**: The binary downloads automatically on first use.

2. **Configure your API key**: Add your Kagi API key to Zed's context server settings.

## Available Tools

This extension provides five tools for accessing Kagi's APIs:

1. **Search (kagi_search_fetch)**: Access Kagi's premium search results
2. **Summarizer (kagi_summarizer)**: Summarize content from any URL (web pages, videos, etc.)
3. **FastGPT (kagi_fastgpt)**: Generate AI-powered answers with web search and references
4. **Web Enrichment (kagi_enrich_web)**: Discover non-commercial, "small web" content
5. **News Enrichment (kagi_enrich_news)**: Find alternative news sources and discussions

## Configuration Options

### Summarizer Engine

Default engine is "cecil". Consumer engines (cecil, agnes) are cheaper than enterprise (muriel). See [summarization engines](https://help.kagi.com/kagi/api/summarizer.html#summarization-engines) for details.

### FastGPT Settings

- `kagi_fastgpt_cache`: Enable/disable caching of responses (default: true)
- `kagi_fastgpt_web_search`: Enable/disable web search enrichment (default: true)

## Pricing

Usage may incur charges on your Kagi account:
- [Search API](https://help.kagi.com/kagi/api/search.html#pricing)
- [Summarizer](https://help.kagi.com/kagi/api/summarizer.html#pricing)
- [FastGPT](https://help.kagi.com/kagi/api/fastgpt.html#pricing)
- [Enrichment APIs](https://help.kagi.com/kagi/api/enrich.html#pricing)
