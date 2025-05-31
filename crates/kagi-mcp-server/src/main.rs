//! Kagi MCP Server - Provides Kagi search and summarization tools for AI assistants
//!
//! This server implements the Model Context Protocol (MCP) to provide AI assistants
//! with access to Kagi's search and Universal Summarizer APIs.

use async_trait::async_trait;
use clap::Parser;
use kagiapi::{KagiClient, SummarizerEngine, SummaryType};
use mcp_server::{McpServer, Tool, ToolHandler, ToolResult};
use serde_json::{json, Value};
use std::env;

#[derive(Parser)]
#[command(name = "kagi-mcp-server")]
#[command(about = "Kagi MCP Server for AI assistants")]
struct Args {
    /// Kagi API key (can also be set via KAGI_API_KEY environment variable)
    #[arg(long, env = "KAGI_API_KEY")]
    api_key: Option<String>,
    
    /// Default summarizer engine
    #[arg(long, env = "KAGI_SUMMARIZER_ENGINE", default_value = "cecil")]
    summarizer_engine: String,
}

struct KagiToolHandler {
    client: KagiClient,
    default_engine: SummarizerEngine,
}

impl KagiToolHandler {
    fn new(api_key: String, default_engine: SummarizerEngine) -> Self {
        Self {
            client: KagiClient::new(api_key),
            default_engine,
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

    fn parse_summary_type(&self, type_str: Option<&str>) -> SummaryType {
        match type_str {
            Some("takeaway") => SummaryType::Takeaway,
            _ => SummaryType::Summary,
        }
    }

    async fn handle_search(&self, queries: &[Value]) -> ToolResult {
        let mut all_results = String::new();
        
        for (index, query_value) in queries.iter().enumerate() {
            if let Some(query) = query_value.as_str() {
                match self.client.search(query, Some(10)).await {
                    Ok(response) => {
                        if index > 0 {
                            all_results.push('\n');
                        }
                        all_results.push_str(&self.format_search_results(query, &response));
                    }
                    Err(e) => {
                        return Err(format!("Search failed for query '{}': {}", query, e));
                    }
                }
            } else {
                return Err("Invalid query format - expected string".to_string());
            }
        }

        Ok(vec![json!({
            "type": "text",
            "text": all_results
        })])
    }

    fn format_search_results(&self, query: &str, response: &kagiapi::SearchResponse) -> String {
        let mut output = format!("-----\nResults for search query \"{}\":\n-----\n", query);
        let mut result_number = 1;

        for result in &response.data {
            if result.result_type == 0 {
                output.push_str(&format!(
                    "{}: {}\n{}\nPublished Date: {}\n{}\n\n",
                    result_number,
                    result.title,
                    result.url,
                    result.published.as_deref().unwrap_or("Not Available"),
                    result.snippet
                ));
                result_number += 1;
            }
        }

        output
    }

    async fn handle_summarize(&self, url: &str, engine: Option<&str>, summary_type: Option<&str>, target_language: Option<&str>) -> ToolResult {
        let engine = self.parse_engine(engine);
        let summary_type = self.parse_summary_type(summary_type);

        match self.client.summarize(url, Some(engine), Some(summary_type), target_language).await {
            Ok(summary_data) => {
                Ok(vec![json!({
                    "type": "text",
                    "text": summary_data.output
                })])
            }
            Err(e) => Err(format!("Summarization failed: {}", e)),
        }
    }
}

#[async_trait]
impl ToolHandler for KagiToolHandler {
    async fn handle_tool(&self, name: &str, args: Value) -> ToolResult {
        match name {
            "kagi_search_fetch" => {
                if let Some(queries) = args.get("queries").and_then(|v| v.as_array()) {
                    self.handle_search(queries).await
                } else {
                    Err("Missing or invalid 'queries' parameter".to_string())
                }
            }
            "kagi_summarizer" => {
                if let Some(url) = args.get("url").and_then(|v| v.as_str()) {
                    let engine = args.get("engine").and_then(|v| v.as_str());
                    let summary_type = args.get("summary_type").and_then(|v| v.as_str());
                    let target_language = args.get("target_language").and_then(|v| v.as_str());
                    
                    self.handle_summarize(url, engine, summary_type, target_language).await
                } else {
                    Err("Missing 'url' parameter".to_string())
                }
            }
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }

    fn get_tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "kagi_search_fetch".to_string(),
                description: "Fetch web results based on one or more queries using the Kagi Search API. Use for general search and when the user explicitly tells you to 'fetch' results/information. Results are from all queries given. They are numbered continuously, so that a user may be able to refer to a result by a specific number.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "queries": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            },
                            "description": "One or more concise, keyword-focused search queries. Include essential context within each query for standalone use."
                        }
                    },
                    "required": ["queries"]
                }),
            },
            Tool {
                name: "kagi_summarizer".to_string(),
                description: "Summarize content from a URL using the Kagi Summarizer API. The Summarizer can summarize any document type (text webpage, video, audio, etc.)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "url": {
                            "type": "string",
                            "description": "A URL to a document to summarize."
                        },
                        "summary_type": {
                            "type": "string",
                            "enum": ["summary", "takeaway"],
                            "default": "summary",
                            "description": "Type of summary to produce. Options are 'summary' for paragraph prose and 'takeaway' for a bulleted list of key points."
                        },
                        "engine": {
                            "type": "string",
                            "enum": ["cecil", "agnes", "daphne", "muriel"],
                            "description": "Summarization engine to use. Defaults to configured engine."
                        },
                        "target_language": {
                            "type": "string",
                            "description": "Desired output language using language codes (e.g., 'EN' for English). If not specified, the document's original language influences the output."
                        }
                    },
                    "required": ["url"]
                }),
            },
        ]
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let api_key = args.api_key.or_else(|| env::var("KAGI_API_KEY").ok())
        .ok_or("KAGI_API_KEY must be provided via --api-key or environment variable")?;

    let default_engine = match args.summarizer_engine.as_str() {
        "cecil" => SummarizerEngine::Cecil,
        "agnes" => SummarizerEngine::Agnes,
        "daphne" => SummarizerEngine::Daphne,
        "muriel" => SummarizerEngine::Muriel,
        _ => {
            eprintln!("Warning: Unknown engine '{}', defaulting to 'cecil'", args.summarizer_engine);
            SummarizerEngine::Cecil
        }
    };

    let handler = KagiToolHandler::new(api_key, default_engine);
    let server = McpServer::new("kagi-mcp-server", "0.1.0", handler);
    
    server.run().await?;
    Ok(())
}