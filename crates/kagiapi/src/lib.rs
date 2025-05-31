//! Rust client library for Kagi Search and Universal Summarizer APIs
//!
//! This crate provides a simple, async client for interacting with Kagi's APIs:
//! - Search API for web search results
//! - Universal Summarizer API for content summarization
//!
//! References:
//! - https://help.kagi.com/kagi/api/search.html
//! - https://help.kagi.com/kagi/api/summarizer.html
//!
//! # Example
//!
//! ```no_run
//! use kagiapi::{KagiClient, SummaryType, SummarizerEngine};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), kagiapi::Error> {
//!     let client = KagiClient::new("your-api-key");
//!
//!     // Search the web
//!     let results = client.search("rust programming", Some(10)).await?;
//!     for result in results.data {
//!         if result.result_type == 0 {
//!             println!("{}: {}", result.title, result.url);
//!         }
//!     }
//!
//!     // Summarize content
//!     let summary = client.summarize(
//!         "https://example.com/article",
//!         Some(SummarizerEngine::Cecil),
//!         Some(SummaryType::Summary),
//!         None
//!     ).await?;
//!     println!("Summary: {}", summary.output);
//!
//!     Ok(())
//! }
//! ```

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

pub const API_BASE_URL: &str = "https://kagi.com/api/v0";

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid API key")]
    InvalidApiKey,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct KagiClient {
    client: Client,
    api_key: String,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResponse {
    pub meta: SearchMeta,
    pub data: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchMeta {
    pub id: String,
    pub node: String,
    pub ms: u64,
    #[serde(default)]
    pub api_balance: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    #[serde(rename = "t")]
    pub result_type: i32, // 0 = search result, 1 = related searches
    #[serde(default)]
    pub rank: Option<i32>,
    pub url: String,
    pub title: String,
    pub snippet: String,
    #[serde(default)]
    pub published: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thumbnail {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SummaryResponse {
    pub meta: SummaryMeta,
    pub data: SummaryData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SummaryMeta {
    pub id: String,
    pub node: String,
    pub ms: u64,
    pub api_balance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SummaryData {
    pub output: String,
    #[serde(default)]
    pub tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SummarizerEngine {
    Cecil,
    Agnes,
    Daphne,
    Muriel,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SummaryType {
    Summary,
    Takeaway,
}

impl KagiClient {
    /// Create a new Kagi API client with the given API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: API_BASE_URL.to_string(),
        }
    }

    /// Create a new client with a custom base URL (useful for testing)
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: base_url.into(),
        }
    }

    /// Search the web using Kagi's Search API
    ///
    /// # Arguments
    /// * `query` - The search query
    /// * `limit` - Maximum number of results (optional, defaults to 10)
    pub async fn search(&self, query: &str, limit: Option<u32>) -> Result<SearchResponse> {
        let mut params = HashMap::new();
        params.insert("q", query.to_string());
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }

        let url = format!("{}/search", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bot {}", self.api_key))
            .json(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::Api {
                status,
                message: text,
            });
        }

        let search_response: SearchResponse = response.json().await?;
        Ok(search_response)
    }

    /// Summarize content using Kagi's Universal Summarizer API
    ///
    /// # Arguments
    /// * `url` - URL of the content to summarize
    /// * `engine` - Summarization engine to use (optional, defaults to Cecil)
    /// * `summary_type` - Type of summary (optional, defaults to Summary)
    /// * `target_language` - Target language code (optional)
    pub async fn summarize(
        &self,
        url: &str,
        engine: Option<SummarizerEngine>,
        summary_type: Option<SummaryType>,
        target_language: Option<&str>,
    ) -> Result<SummaryData> {
        let mut params = HashMap::new();
        params.insert("url", url.to_string());

        if let Some(engine) = engine {
            params.insert(
                "engine",
                serde_json::to_string(&engine)?
                    .trim_matches('"')
                    .to_string(),
            );
        }

        if let Some(summary_type) = summary_type {
            params.insert(
                "summary_type",
                serde_json::to_string(&summary_type)?
                    .trim_matches('"')
                    .to_string(),
            );
        }

        if let Some(target_language) = target_language {
            params.insert("target_language", target_language.to_string());
        }

        let url = format!("{}/summarize", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bot {}", self.api_key))
            .json(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::Api {
                status,
                message: text,
            });
        }

        let summary_response: SummaryResponse = response.json().await?;
        Ok(summary_response.data)
    }

    /// Summarize text content directly (not from URL)
    ///
    /// # Arguments
    /// * `text` - The text content to summarize
    /// * `engine` - Summarization engine to use (optional, defaults to Cecil)
    /// * `summary_type` - Type of summary (optional, defaults to Summary)
    /// * `target_language` - Target language code (optional)
    pub async fn summarize_text(
        &self,
        text: &str,
        engine: Option<SummarizerEngine>,
        summary_type: Option<SummaryType>,
        target_language: Option<&str>,
    ) -> Result<SummaryData> {
        let mut params = HashMap::new();
        params.insert("text", text.to_string());

        if let Some(engine) = engine {
            params.insert(
                "engine",
                serde_json::to_string(&engine)?
                    .trim_matches('"')
                    .to_string(),
            );
        }

        if let Some(summary_type) = summary_type {
            params.insert(
                "summary_type",
                serde_json::to_string(&summary_type)?
                    .trim_matches('"')
                    .to_string(),
            );
        }

        if let Some(target_language) = target_language {
            params.insert("target_language", target_language.to_string());
        }

        let url = format!("{}/summarize", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bot {}", self.api_key))
            .json(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::Api {
                status,
                message: text,
            });
        }

        let summary_response: SummaryResponse = response.json().await?;
        Ok(summary_response.data)
    }

    /// Get the current API balance
    pub async fn get_balance(&self) -> Result<f64> {
        // This would require a separate API endpoint
        // For now, we can extract it from other API responses
        let response = self.search("test", Some(1)).await?;
        response.meta.api_balance.ok_or(Error::Api {
            status: 404,
            message: "Balance not available".to_string(),
        })
    }
}

impl Default for SummarizerEngine {
    fn default() -> Self {
        Self::Cecil
    }
}

impl Default for SummaryType {
    fn default() -> Self {
        Self::Summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = KagiClient::new("test-key");
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.base_url, API_BASE_URL);
    }

    #[test]
    fn test_client_with_custom_url() {
        let client = KagiClient::with_base_url("test-key", "https://custom.api.com");
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.base_url, "https://custom.api.com");
    }

    #[test]
    fn test_serialization() {
        let engine = SummarizerEngine::Cecil;
        let json = serde_json::to_string(&engine).unwrap();
        assert_eq!(json, "\"cecil\"");

        let summary_type = SummaryType::Takeaway;
        let json = serde_json::to_string(&summary_type).unwrap();
        assert_eq!(json, "\"takeaway\"");
    }
}
