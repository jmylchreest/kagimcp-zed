//! Kagi MCP Server - Provides Kagi search and summarization tools for AI assistants
//!
//! This server implements the Model Context Protocol (MCP) to provide AI assistants
//! with access to Kagi's search and Universal Summarizer APIs.

use clap::Parser;
use kagiapi::{KagiClient, SummarizerEngine, SummaryType};
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router,
    transport::stdio,
    ErrorData as McpError, ServerHandler, ServiceExt,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use std::fmt::Write;

#[derive(Parser)]
#[command(name = "kagi-mcp-server")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Kagi MCP Server for AI assistants")]
struct Args {
    /// Kagi API key (can also be set via `KAGI_API_KEY` environment variable)
    #[arg(long, env = "KAGI_API_KEY")]
    api_key: Option<String>,

    /// Default summarizer engine
    #[arg(long, env = "KAGI_SUMMARIZER_ENGINE", default_value = "cecil")]
    summarizer_engine: String,

    /// API version for search endpoint
    #[arg(long, env = "KAGI_SEARCH_API_VERSION", default_value = "v0")]
    search_api_version: String,

    /// API version for summarizer endpoint
    #[arg(long, env = "KAGI_SUMMARIZER_API_VERSION", default_value = "v0")]
    summarizer_api_version: String,

    /// API version for `FastGPT` endpoint
    #[arg(long, env = "KAGI_FASTGPT_API_VERSION", default_value = "v0")]
    fastgpt_api_version: String,

    /// API version for enrichment endpoint
    #[arg(long, env = "KAGI_ENRICH_API_VERSION", default_value = "v0")]
    enrich_api_version: String,
}

// ----- Tool parameter types -----

#[derive(Debug, Deserialize, JsonSchema)]
struct SearchParams {
    /// One or more concise, keyword-focused search queries. Include essential
    /// context within each query for standalone use.
    queries: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct SummarizerParams {
    /// A URL to a document to summarize.
    url: String,

    /// Type of summary to produce. Options are 'summary' for paragraph prose
    /// and 'takeaway' for a bulleted list of key points.
    #[serde(default)]
    summary_type: Option<String>,

    /// Summarization engine to use. Defaults to configured engine.
    #[serde(default)]
    engine: Option<String>,

    /// Desired output language using language codes (e.g., 'EN' for English).
    /// If not specified, the document's original language influences the output.
    #[serde(default)]
    target_language: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FastGptParams {
    /// The question or query to be answered by the AI.
    query: String,

    /// Whether to allow cached requests & responses. Defaults to true.
    #[serde(default)]
    cache: Option<bool>,

    /// Whether to perform web searches to enrich answers. Currently, must be
    /// set to true.
    #[serde(default)]
    web_search: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct EnrichParams {
    /// The search query to find content.
    query: String,
}

// ----- Server implementation -----

#[derive(Clone)]
struct KagiMcpServer {
    client: KagiClient,
    default_engine: SummarizerEngine,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl KagiMcpServer {
    fn new(
        api_key: String,
        default_engine: SummarizerEngine,
        search_version: String,
        summarizer_version: String,
        fastgpt_version: String,
        enrich_version: String,
    ) -> Self {
        Self {
            client: KagiClient::with_api_versions(
                api_key,
                search_version,
                summarizer_version,
                fastgpt_version,
                enrich_version,
            ),
            default_engine,
            tool_router: Self::tool_router(),
        }
    }

    fn parse_engine(&self, engine_str: Option<&str>) -> SummarizerEngine {
        match engine_str {
            Some("cecil") => SummarizerEngine::Cecil,
            Some("agnes") => SummarizerEngine::Agnes,
            Some("daphne") => SummarizerEngine::Daphne,
            Some("muriel") => SummarizerEngine::Muriel,
            _ => self.default_engine,
        }
    }

    fn parse_summary_type(type_str: Option<&str>) -> SummaryType {
        match type_str {
            Some("takeaway") => SummaryType::Takeaway,
            _ => SummaryType::Summary,
        }
    }

    fn format_search_results(query: &str, response: &kagiapi::SearchResponse) -> String {
        let mut output = format!("-----\nResults for search query \"{query}\":\n-----\n");
        let mut result_number = 1;

        for result in &response.data {
            match result.result_type {
                0 => {
                    if let (Some(title), Some(url)) = (&result.title, &result.url) {
                        let _ = writeln!(output, "{result_number}: {title}\n{url}");
                        let _ = writeln!(
                            output,
                            "Published Date: {}",
                            result.published.as_deref().unwrap_or("Not Available")
                        );
                        if let Some(snippet) = &result.snippet {
                            let _ = writeln!(output, "{snippet}");
                        }
                        output.push('\n');
                        result_number += 1;
                    }
                }
                1 => {
                    if let Some(list) = &result.list {
                        output.push_str("Related searches:\n");
                        for item in list {
                            let _ = writeln!(output, "- {item}");
                        }
                        output.push('\n');
                    }
                }
                _ => {
                    if let Some(title) = &result.title {
                        let _ = writeln!(output, "{result_number}: {title}");
                        if let Some(url) = &result.url {
                            let _ = writeln!(output, "{url}");
                        }
                        if let Some(snippet) = &result.snippet {
                            let _ = writeln!(output, "{snippet}");
                        }
                        output.push('\n');
                        result_number += 1;
                    }
                }
            }
        }

        output
    }

    /// Fetch web results based on one or more queries using the Kagi Search API.
    /// Use for general search and when the user explicitly tells you to 'fetch'
    /// results/information. Results are from all queries given. They are numbered
    /// continuously, so that a user may be able to refer to a result by a
    /// specific number.
    #[tool(name = "kagi_search_fetch")]
    async fn search(
        &self,
        Parameters(params): Parameters<SearchParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut all_results = String::new();

        for (index, query) in params.queries.iter().enumerate() {
            match self.client.search(query, Some(10)).await {
                Ok(response) => {
                    if index > 0 {
                        all_results.push('\n');
                    }
                    all_results.push_str(&Self::format_search_results(query, &response));
                }
                Err(e) => {
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "Search failed for query '{query}': {e}"
                    ))]));
                }
            }
        }

        Ok(CallToolResult::success(vec![Content::text(all_results)]))
    }

    /// Summarize content from a URL using the Kagi Summarizer API. The
    /// Summarizer can summarize any document type (text webpage, video,
    /// audio, etc.)
    #[tool(name = "kagi_summarizer")]
    async fn summarize(
        &self,
        Parameters(params): Parameters<SummarizerParams>,
    ) -> Result<CallToolResult, McpError> {
        let engine = self.parse_engine(params.engine.as_deref());
        let summary_type = Self::parse_summary_type(params.summary_type.as_deref());

        match self
            .client
            .summarize(
                &params.url,
                Some(engine),
                Some(summary_type),
                params.target_language.as_deref(),
            )
            .await
        {
            Ok(summary_data) => Ok(CallToolResult::success(vec![Content::text(
                summary_data.output,
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Summarization failed: {e}"
            ))])),
        }
    }

    /// Generate AI-powered answers to questions using the Kagi FastGPT API.
    /// This tool performs web searches automatically to provide well-referenced,
    /// up-to-date responses. Use for direct questions that need AI-generated
    /// answers with citations.
    #[tool(name = "kagi_fastgpt")]
    async fn fastgpt(
        &self,
        Parameters(params): Parameters<FastGptParams>,
    ) -> Result<CallToolResult, McpError> {
        match self
            .client
            .fastgpt(&params.query, params.cache, params.web_search)
            .await
        {
            Ok(response) => {
                let mut result = response.output.clone();

                if !response.references.is_empty() {
                    result.push_str("\n\nReferences:\n");
                    for (i, reference) in response.references.iter().enumerate() {
                        let _ = writeln!(result, "{}. {}", i + 1, reference.title);
                        let _ = writeln!(result, "   {}", reference.url);
                    }
                }

                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "FastGPT failed for query '{}': {e}",
                params.query
            ))])),
        }
    }

    /// Find non-commercial, 'small web' content and discussions using Kagi's
    /// Web Enrichment API. Great for discovering unique websites and content
    /// that might not appear in regular search results.
    #[tool(name = "kagi_enrich_web")]
    async fn enrich_web(
        &self,
        Parameters(params): Parameters<EnrichParams>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_enrich(&params.query, kagiapi::EnrichType::Web)
            .await
    }

    /// Find non-mainstream news sources and discussions using Kagi's News
    /// Enrichment API. Useful for discovering alternative perspectives and
    /// news coverage.
    #[tool(name = "kagi_enrich_news")]
    async fn enrich_news(
        &self,
        Parameters(params): Parameters<EnrichParams>,
    ) -> Result<CallToolResult, McpError> {
        self.handle_enrich(&params.query, kagiapi::EnrichType::News)
            .await
    }
}

impl KagiMcpServer {
    async fn handle_enrich(
        &self,
        query: &str,
        enrich_type: kagiapi::EnrichType,
    ) -> Result<CallToolResult, McpError> {
        match self.client.enrich(query, enrich_type).await {
            Ok(results) => {
                let type_name = match enrich_type {
                    kagiapi::EnrichType::Web => "web",
                    kagiapi::EnrichType::News => "news",
                };

                let mut formatted =
                    format!("Kagi {type_name} enrichment results for query: {query}\n\n");

                for (i, result) in results.iter().enumerate() {
                    if result.result_type == 0 {
                        if let Some(title) = &result.title {
                            let _ = writeln!(formatted, "{}. {}", i + 1, title);
                        } else {
                            let _ = writeln!(formatted, "{}. [No Title]", i + 1);
                        }

                        if let Some(url) = &result.url {
                            let _ = writeln!(formatted, "   URL: {url}");
                        }

                        if let Some(snippet) = &result.snippet {
                            if !snippet.is_empty() {
                                let _ = writeln!(formatted, "   {snippet}");
                            }
                        }

                        if let Some(published) = &result.published {
                            if !published.is_empty() {
                                let _ = writeln!(formatted, "   Published: {published}");
                            }
                        }

                        formatted.push('\n');
                    }
                }

                Ok(CallToolResult::success(vec![Content::text(formatted)]))
            }
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Enrichment failed for query '{query}': {e}"
            ))])),
        }
    }
}

#[tool_handler]
impl ServerHandler for KagiMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Kagi MCP Server providing search, summarization, FastGPT, \
                 and enrichment tools powered by Kagi's APIs."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "kagi-mcp-server".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let api_key = args
        .api_key
        .ok_or("KAGI_API_KEY must be provided via --api-key or environment variable")?;

    let default_engine = match args.summarizer_engine.as_str() {
        "cecil" => SummarizerEngine::Cecil,
        "agnes" => SummarizerEngine::Agnes,
        "daphne" => SummarizerEngine::Daphne,
        "muriel" => SummarizerEngine::Muriel,
        _ => {
            eprintln!(
                "Warning: Unknown engine '{}', defaulting to 'cecil'",
                args.summarizer_engine
            );
            SummarizerEngine::Cecil
        }
    };

    let server = KagiMcpServer::new(
        api_key,
        default_engine,
        args.search_api_version,
        args.summarizer_api_version,
        args.fastgpt_api_version,
        args.enrich_api_version,
    );

    let service = server
        .serve(stdio())
        .await
        .inspect_err(|e| eprintln!("Failed to start MCP server: {e}"))?;

    service.waiting().await?;
    Ok(())
}
