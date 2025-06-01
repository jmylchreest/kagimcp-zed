//! Rust client library for Kagi Search and Universal Summarizer APIs
//!
//! This crate provides a simple, async client for interacting with Kagi's APIs:
//! - Search API for web search results
//! - Universal Summarizer API for content summarization
//!
//! References:
//! - https://help.kagi.com/kagi/api/search.html
//! - https://help.kagi.com/kagi/api/summarizer.html
//! - https://help.kagi.com/kagi/api/fastgpt.html
//! - https://help.kagi.com/kagi/api/enrich.html
//!
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

pub const API_BASE_URL_PREFIX: &str = "https://kagi.com/api";

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
    search_api_version: String,
    summarizer_api_version: String,
    fastgpt_api_version: String,
    enrich_api_version: String,
    base_url_prefix: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum EnrichType {
    Web,
    News,
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
    #[serde(default)]
    pub url: Option<String>, // Required for type=0, not present for type=1
    #[serde(default)]
    pub title: Option<String>, // Required for type=0, not present for type=1
    #[serde(default)]
    pub snippet: Option<String>, // Optional for type=0, not present for type=1
    #[serde(default)]
    pub published: Option<String>, // Optional for type=0
    #[serde(default)]
    pub thumbnail: Option<Thumbnail>, // Optional for type=0
    #[serde(default)]
    pub list: Option<Vec<String>>, // Present only for type=1 (related searches)
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FastGptResponse {
    pub meta: FastGptMeta,
    pub data: FastGptData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FastGptMeta {
    pub id: String,
    pub node: String,
    pub ms: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FastGptData {
    pub output: String,
    pub tokens: u32,
    #[serde(default)]
    pub references: Vec<FastGptReference>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FastGptReference {
    pub title: String,
    pub snippet: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnrichResponse {
    pub meta: SearchMeta,
    pub data: Vec<SearchResult>,
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
            search_api_version: "v0".to_string(),
            summarizer_api_version: "v0".to_string(),
            fastgpt_api_version: "v0".to_string(),
            enrich_api_version: "v0".to_string(),
            base_url_prefix: API_BASE_URL_PREFIX.to_string(),
        }
    }

    /// Create a new client with a custom base URL prefix (useful for testing)
    pub fn with_base_url_prefix(
        api_key: impl Into<String>,
        base_url_prefix: impl Into<String>,
    ) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            search_api_version: "v0".to_string(),
            summarizer_api_version: "v0".to_string(),
            fastgpt_api_version: "v0".to_string(),
            enrich_api_version: "v0".to_string(),
            base_url_prefix: base_url_prefix.into(),
        }
    }

    /// Create a new client with specific API versions for each endpoint
    pub fn with_api_versions(
        api_key: impl Into<String>,
        search_version: impl Into<String>,
        summarizer_version: impl Into<String>,
        fastgpt_version: impl Into<String>,
        enrich_version: impl Into<String>,
    ) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            search_api_version: search_version.into(),
            summarizer_api_version: summarizer_version.into(),
            fastgpt_api_version: fastgpt_version.into(),
            enrich_api_version: enrich_version.into(),
            base_url_prefix: API_BASE_URL_PREFIX.to_string(),
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

        // Use URL parameters instead of JSON body for search API
        let mut url = url::Url::parse(&format!(
            "{}/{}/search",
            self.base_url_prefix, self.search_api_version
        ))
        .map_err(|_| Error::Api {
            status: 400,
            message: "Invalid URL".to_string(),
        })?;

        // Add query parameters to URL
        url.query_pairs_mut().append_pair("q", query);
        if let Some(limit) = limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bot {}", self.api_key))
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
        let mut params = serde_json::Map::new();
        params.insert("url".to_string(), serde_json::Value::String(url.to_string()));

        if let Some(engine) = engine {
            let engine_str = serde_json::to_string(&engine)?
                .trim_matches('"')
                .to_string();
            params.insert("engine".to_string(), serde_json::Value::String(engine_str));
        }

        if let Some(summary_type) = summary_type {
            let summary_type_str = serde_json::to_string(&summary_type)?
                .trim_matches('"')
                .to_string();
            params.insert("summary_type".to_string(), serde_json::Value::String(summary_type_str));
        }

        if let Some(target_language) = target_language {
            params.insert("target_language".to_string(), serde_json::Value::String(target_language.to_string()));
        }

        let url = format!(
            "{}/{}/summarize",
            self.base_url_prefix, self.summarizer_api_version
        );
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bot {}", self.api_key))
            .json(&serde_json::Value::Object(params))
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
        let mut params = serde_json::Map::new();
        params.insert("text".to_string(), serde_json::Value::String(text.to_string()));

        if let Some(engine) = engine {
            let engine_str = serde_json::to_string(&engine)?
                .trim_matches('"')
                .to_string();
            params.insert("engine".to_string(), serde_json::Value::String(engine_str));
        }

        if let Some(summary_type) = summary_type {
            let summary_type_str = serde_json::to_string(&summary_type)?
                .trim_matches('"')
                .to_string();
            params.insert("summary_type".to_string(), serde_json::Value::String(summary_type_str));
        }

        if let Some(target_language) = target_language {
            params.insert("target_language".to_string(), serde_json::Value::String(target_language.to_string()));
        }

        let url = format!(
            "{}/{}/summarize",
            self.base_url_prefix, self.summarizer_api_version
        );
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bot {}", self.api_key))
            .json(&serde_json::Value::Object(params))
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

    /// Use FastGPT to answer a query
    ///
    /// # Arguments
    /// * `query` - The query to be answered
    /// * `cache` - Whether to allow cached requests & responses (optional, defaults to true)
    /// * `web_search` - Whether to perform web searches to enrich answers (optional, defaults to true)
    pub async fn fastgpt(
        &self,
        query: &str,
        cache: Option<bool>,
        web_search: Option<bool>,
    ) -> Result<FastGptData> {
        let mut params = serde_json::Map::new();
        params.insert("query".to_string(), serde_json::Value::String(query.to_string()));

        if let Some(cache) = cache {
            params.insert("cache".to_string(), serde_json::Value::Bool(cache));
        }

        if let Some(web_search) = web_search {
            params.insert("web_search".to_string(), serde_json::Value::Bool(web_search));
        }

        let url = format!(
            "{}/{}/fastgpt",
            self.base_url_prefix, self.fastgpt_api_version
        );
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bot {}", self.api_key))
            .header("Content-Type", "application/json")
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

        let fastgpt_response: FastGptResponse = response.json().await?;
        Ok(fastgpt_response.data)
    }

    /// Use Kagi's Enrichment API to get non-commercial content
    ///
    /// # Arguments
    /// * `query` - The search query
    /// * `enrich_type` - The type of enrichment (web or news)
    pub async fn enrich(&self, query: &str, enrich_type: EnrichType) -> Result<Vec<SearchResult>> {
        // Build the URL with query parameters
        let endpoint = match enrich_type {
            EnrichType::Web => "web",
            EnrichType::News => "news",
        };

        // Construct the URL with parameters
        let mut url = url::Url::parse(&format!(
            "{}/{}/enrich/{}",
            self.base_url_prefix, self.enrich_api_version, endpoint
        ))
        .map_err(|_| Error::Api {
            status: 400,
            message: "Invalid URL".to_string(),
        })?;

        url.query_pairs_mut().append_pair("q", query);

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bot {}", self.api_key))
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

        let enrich_response: EnrichResponse = response.json().await?;
        Ok(enrich_response.data)
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
        assert_eq!(client.base_url_prefix, API_BASE_URL_PREFIX);
        assert_eq!(client.search_api_version, "v0");
        assert_eq!(client.summarizer_api_version, "v0");
        assert_eq!(client.fastgpt_api_version, "v0");
        assert_eq!(client.enrich_api_version, "v0");
    }

    #[test]
    fn test_client_with_custom_url() {
        let client = KagiClient::with_base_url_prefix("test-key", "https://custom.api.com");
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.base_url_prefix, "https://custom.api.com");
    }

    #[test]
    fn test_client_with_api_versions() {
        let client = KagiClient::with_api_versions("test-key", "v1", "v2", "v3", "v4");
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.search_api_version, "v1");
        assert_eq!(client.summarizer_api_version, "v2");
        assert_eq!(client.fastgpt_api_version, "v3");
        assert_eq!(client.enrich_api_version, "v4");
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

    #[test]
    fn test_fastgpt_params_serialization() {
        // Test that boolean parameters are serialized as JSON booleans, not strings
        let mut params = serde_json::Map::new();
        params.insert("query".to_string(), serde_json::Value::String("test query".to_string()));
        params.insert("web_search".to_string(), serde_json::Value::Bool(true));
        params.insert("cache".to_string(), serde_json::Value::Bool(false));

        let json = serde_json::to_string(&serde_json::Value::Object(params)).unwrap();
        
        // Verify that booleans are not quoted in the JSON
        assert!(json.contains("\"web_search\":true"));
        assert!(json.contains("\"cache\":false"));
        assert!(!json.contains("\"web_search\":\"true\""));
        assert!(!json.contains("\"cache\":\"false\""));
    }
}
