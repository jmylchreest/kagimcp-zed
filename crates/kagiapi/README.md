# KagiAPI

A Rust client library for Kagi's Search and Universal Summarizer APIs.

## Features

- **Search API**: Search the web using Kagi's high-quality search results
- **Universal Summarizer API**: Summarize content from URLs or text
- **Async/await support**: Built on tokio and reqwest
- **Type-safe**: Strongly typed request/response structures
- **Error handling**: Comprehensive error types with detailed messages

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
kagiapi = "0.1.0"
```

## Usage

### Basic Setup

```rust
use kagiapi::KagiClient;

let client = KagiClient::new("your-api-key");
```

### Search

```rust
use kagiapi::{KagiClient, SearchResult};

#[tokio::main]
async fn main() -> Result<(), kagiapi::Error> {
    let client = KagiClient::new("your-api-key");
    
    let results = client.search("rust programming", Some(10)).await?;
    
    for result in results.data {
        if result.result_type == 0 { // 0 = search result, 1 = related searches
            println!("{}: {}", result.title, result.url);
            println!("{}\n", result.snippet);
        }
    }
    
    Ok(())
}
```

### Summarization

```rust
use kagiapi::{KagiClient, SummarizerEngine, SummaryType};

#[tokio::main]
async fn main() -> Result<(), kagiapi::Error> {
    let client = KagiClient::new("your-api-key");
    
    // Summarize from URL
    let summary = client.summarize(
        "https://example.com/article",
        Some(SummarizerEngine::Cecil),
        Some(SummaryType::Summary),
        None // target language (optional)
    ).await?;
    
    println!("Summary: {}", summary.output);
    
    // Summarize text directly
    let text_summary = client.summarize_text(
        "Your long text content here...",
        Some(SummarizerEngine::Agnes),
        Some(SummaryType::Takeaway),
        Some("EN")
    ).await?;
    
    println!("Text summary: {}", text_summary.output);
    
    Ok(())
}
```

## API Reference

### KagiClient

#### Methods

- `new(api_key: impl Into<String>) -> Self`
- `with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self`
- `search(query: &str, limit: Option<u32>) -> Result<SearchResponse>`
- `summarize(url: &str, engine: Option<SummarizerEngine>, summary_type: Option<SummaryType>, target_language: Option<&str>) -> Result<SummaryData>`
- `summarize_text(text: &str, engine: Option<SummarizerEngine>, summary_type: Option<SummaryType>, target_language: Option<&str>) -> Result<SummaryData>`

### Enums

#### SummarizerEngine
- `Cecil` - Consumer-grade, fast and efficient
- `Agnes` - Consumer-grade, alternative model
- `Daphne` - High-quality summarization
- `Muriel` - Enterprise-grade, highest quality

#### SummaryType
- `Summary` - Paragraph prose format
- `Takeaway` - Bulleted list of key points

## Error Handling

The library provides comprehensive error handling through the `kagiapi::Error` enum:

```rust
use kagiapi::Error;

match client.search("query", None).await {
    Ok(results) => println!("Found {} results", results.data.len()),
    Err(Error::Api { status, message }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(Error::Request(e)) => {
        eprintln!("Request failed: {}", e);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Requirements

- **API Key**: Get your API key from [Kagi Settings](https://kagi.com/settings?p=api)
- **API Access**: The Search API is currently in closed beta. Request access by emailing support@kagi.com
- **Rust**: This crate requires Rust 1.70 or later

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.