# kagiapi

A Rust client library for Kagi's Search, Enrich, FastGPT, and Universal Summarizer APIs.

## Features

- **Search API** - Web search using Kagi's high-quality, privacy-focused results
- **Universal Summarizer** - Summarize content from URLs or raw text (webpages, PDFs, videos, audio)
- **FastGPT** - AI-powered answers with automatic web search and citations
- **Enrichment API** - Discover non-commercial "small web" content and non-mainstream news
- **Async/await** - Built on `tokio` and `reqwest`
- **Type-safe** - Strongly typed request/response structures
- **Error handling** - Comprehensive error types with detailed messages

## Installation

```toml
[dependencies]
kagiapi = "0.0.30"
```

## Usage

### Basic Setup

```rust
use kagiapi::KagiClient;

let client = KagiClient::new("your-api-key");
```

### Search

```rust
use kagiapi::KagiClient;

#[tokio::main]
async fn main() -> Result<(), kagiapi::Error> {
    let client = KagiClient::new("your-api-key");

    let results = client.search("rust programming", Some(10)).await?;
    for result in results.data {
        if result.result_type == 0 {
            let title = result.title.as_deref().unwrap_or("No title");
            let url = result.url.as_deref().unwrap_or("No URL");
            println!("{title}: {url}");
        }
    }

    Ok(())
}
```

### Summarize a URL

```rust
use kagiapi::{KagiClient, SummarizerEngine, SummaryType};

#[tokio::main]
async fn main() -> Result<(), kagiapi::Error> {
    let client = KagiClient::new("your-api-key");

    let summary = client.summarize(
        "https://example.com/article",
        Some(SummarizerEngine::Cecil),
        Some(SummaryType::Summary),
        None,
    ).await?;
    println!("Summary: {}", summary.output);

    Ok(())
}
```

### Summarize Text Directly

```rust
use kagiapi::{KagiClient, SummarizerEngine, SummaryType};

#[tokio::main]
async fn main() -> Result<(), kagiapi::Error> {
    let client = KagiClient::new("your-api-key");

    let summary = client.summarize_text(
        "Your long text content here...",
        Some(SummarizerEngine::Agnes),
        Some(SummaryType::Takeaway),
        Some("EN"),
    ).await?;
    println!("Key points: {}", summary.output);

    Ok(())
}
```

### FastGPT

```rust
use kagiapi::KagiClient;

#[tokio::main]
async fn main() -> Result<(), kagiapi::Error> {
    let client = KagiClient::new("your-api-key");

    let answer = client.fastgpt("What is Rust's ownership model?", None, None).await?;
    println!("Answer: {}", answer.output);

    for reference in &answer.references {
        println!("  - {} ({})", reference.title, reference.url);
    }

    Ok(())
}
```

### Enrichment

```rust
use kagiapi::{KagiClient, EnrichType};

#[tokio::main]
async fn main() -> Result<(), kagiapi::Error> {
    let client = KagiClient::new("your-api-key");

    // Find small web / non-commercial content
    let web_results = client.enrich("rust async runtime", EnrichType::Web).await?;

    // Find non-mainstream news
    let news_results = client.enrich("open source AI", EnrichType::News).await?;

    Ok(())
}
```

### Custom API Versions

```rust
use kagiapi::KagiClient;

let client = KagiClient::with_api_versions(
    "your-api-key",
    "v0",  // search
    "v0",  // summarizer
    "v0",  // fastgpt
    "v0",  // enrich
);
```

## API Reference

### KagiClient Methods

| Method | Description |
|--------|-------------|
| `new(api_key)` | Create a client with default settings |
| `with_base_url_prefix(api_key, url)` | Create with custom base URL (for testing) |
| `with_api_versions(api_key, ...)` | Create with specific API versions per endpoint |
| `search(query, limit)` | Search the web |
| `summarize(url, engine, type, lang)` | Summarize content from a URL |
| `summarize_text(text, engine, type, lang)` | Summarize raw text |
| `fastgpt(query, cache, web_search)` | AI-powered answers with citations |
| `enrich(query, type)` | Non-commercial content discovery |

### Enums

**SummarizerEngine**: `Cecil` (default), `Agnes`, `Daphne`, `Muriel`

**SummaryType**: `Summary` (paragraph prose), `Takeaway` (bullet points)

**EnrichType**: `Web`, `News`

## Error Handling

```rust
use kagiapi::Error;

match client.search("query", None).await {
    Ok(results) => println!("Found {} results", results.data.len()),
    Err(Error::Api { status, message }) => eprintln!("API error {status}: {message}"),
    Err(Error::Request(e)) => eprintln!("Request failed: {e}"),
    Err(e) => eprintln!("Other error: {e}"),
}
```

## Requirements

- **API Key**: Get your API key from [Kagi Settings](https://kagi.com/settings?p=api)
- **API Access**: The Search API is currently in closed beta. Request access by emailing support@kagi.com

## License

MIT License - see [LICENSE](../../LICENSE) for details.
