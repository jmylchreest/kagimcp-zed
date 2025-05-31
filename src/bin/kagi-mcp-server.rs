use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, Write};


#[derive(Debug, Serialize, Deserialize)]
struct McpRequest {
    jsonrpc: String,
    id: Value,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct McpResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<McpError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct McpError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tool {
    name: String,
    description: String,
    #[serde(rename = "inputSchema")]
    input_schema: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct KagiSearchResponse {
    meta: Value,
    data: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct KagiSummaryResponse {
    meta: Value,
    data: KagiSummaryData,
}

#[derive(Debug, Serialize, Deserialize)]
struct KagiSummaryData {
    output: String,
}

struct KagiMcpServer {
    api_key: String,
    client: reqwest::Client,
}

impl KagiMcpServer {
    fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    async fn search(&self, query: &str) -> Result<String, String> {
        let mut params = HashMap::new();
        params.insert("q", query);
        params.insert("limit", "10");

        let response = self
            .client
            .post("https://kagi.com/api/v0/search")
            .header("Authorization", format!("Bot {}", self.api_key))
            .json(&params)
            .send()
            .await
            .map_err(|e| format!("Search request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Search failed with status: {}", response.status()));
        }

        let search_result: KagiSearchResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse search response: {}", e))?;

        self.format_search_results(query, &search_result)
    }

    fn format_search_results(&self, query: &str, result: &KagiSearchResponse) -> Result<String, String> {
        let mut output = format!("-----\nResults for search query \"{}\":\n-----\n", query);
        let mut result_number = 1;

        for item in &result.data {
            if let Some(t) = item.get("t").and_then(|v| v.as_i64()) {
                if t == 0 {
                    let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("No title");
                    let url = item.get("url").and_then(|v| v.as_str()).unwrap_or("No URL");
                    let snippet = item.get("snippet").and_then(|v| v.as_str()).unwrap_or("No snippet");
                    let published = item.get("published").and_then(|v| v.as_str()).unwrap_or("Not Available");

                    output.push_str(&format!(
                        "{}: {}\n{}\nPublished Date: {}\n{}\n\n",
                        result_number, title, url, published, snippet
                    ));
                    result_number += 1;
                }
            }
        }

        Ok(output)
    }

    async fn summarize(&self, url: &str, engine: Option<&str>, summary_type: Option<&str>) -> Result<String, String> {
        let mut params = HashMap::new();
        params.insert("url", url);
        params.insert("engine", engine.unwrap_or("cecil"));
        params.insert("summary_type", summary_type.unwrap_or("summary"));

        let response = self
            .client
            .post("https://kagi.com/api/v0/summarize")
            .header("Authorization", format!("Bot {}", self.api_key))
            .json(&params)
            .send()
            .await
            .map_err(|e| format!("Summarize request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Summarize failed with status: {}", response.status()));
        }

        let summary_result: KagiSummaryResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse summary response: {}", e))?;

        Ok(summary_result.data.output)
    }

    fn get_tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "kagi_search_fetch".to_string(),
                description: "Fetch web results based on one or more queries using the Kagi Search API. Use for general search and when the user explicitly tells you to 'fetch' results/information.".to_string(),
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

    async fn handle_request(&self, request: McpRequest) -> McpResponse {
        match request.method.as_str() {
            "initialize" => McpResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "kagi-mcp-server",
                        "version": "0.1.0"
                    }
                })),
                error: None,
            },
            "tools/list" => McpResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(json!({
                    "tools": self.get_tools()
                })),
                error: None,
            },
            "tools/call" => {
                if let Some(params) = request.params {
                    if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
                        match name {
                            "kagi_search_fetch" => {
                                if let Some(args) = params.get("arguments") {
                                    if let Some(queries) = args.get("queries").and_then(|v| v.as_array()) {
                                        let mut all_results = String::new();
                                        for query in queries {
                                            if let Some(query_str) = query.as_str() {
                                                match self.search(query_str).await {
                                                    Ok(result) => {
                                                        all_results.push_str(&result);
                                                        all_results.push_str("\n");
                                                    }
                                                    Err(e) => {
                                                        return McpResponse {
                                                            jsonrpc: "2.0".to_string(),
                                                            id: request.id,
                                                            result: None,
                                                            error: Some(McpError {
                                                                code: -1,
                                                                message: format!("Error: {}", e),
                                                                data: None,
                                                            }),
                                                        };
                                                    }
                                                }
                                            }
                                        }
                                        McpResponse {
                                            jsonrpc: "2.0".to_string(),
                                            id: request.id,
                                            result: Some(json!({
                                                "content": [{
                                                    "type": "text",
                                                    "text": all_results
                                                }]
                                            })),
                                            error: None,
                                        }
                                    } else {
                                        McpResponse {
                                            jsonrpc: "2.0".to_string(),
                                            id: request.id,
                                            result: None,
                                            error: Some(McpError {
                                                code: -32602,
                                                message: "Missing or invalid queries parameter".to_string(),
                                                data: None,
                                            }),
                                        }
                                    }
                                } else {
                                    McpResponse {
                                        jsonrpc: "2.0".to_string(),
                                        id: request.id,
                                        result: None,
                                        error: Some(McpError {
                                            code: -32602,
                                            message: "Missing arguments".to_string(),
                                            data: None,
                                        }),
                                    }
                                }
                            }
                            "kagi_summarizer" => {
                                if let Some(args) = params.get("arguments") {
                                    if let Some(url) = args.get("url").and_then(|v| v.as_str()) {
                                        let summary_type = args.get("summary_type").and_then(|v| v.as_str());
                                        let engine = env::var("KAGI_SUMMARIZER_ENGINE").ok();
                                        let engine_ref = engine.as_deref();

                                        match self.summarize(url, engine_ref, summary_type).await {
                                            Ok(result) => McpResponse {
                                                jsonrpc: "2.0".to_string(),
                                                id: request.id,
                                                result: Some(json!({
                                                    "content": [{
                                                        "type": "text",
                                                        "text": result
                                                    }]
                                                })),
                                                error: None,
                                            },
                                            Err(e) => McpResponse {
                                                jsonrpc: "2.0".to_string(),
                                                id: request.id,
                                                result: None,
                                                error: Some(McpError {
                                                    code: -1,
                                                    message: format!("Error: {}", e),
                                                    data: None,
                                                }),
                                            },
                                        }
                                    } else {
                                        McpResponse {
                                            jsonrpc: "2.0".to_string(),
                                            id: request.id,
                                            result: None,
                                            error: Some(McpError {
                                                code: -32602,
                                                message: "Missing url parameter".to_string(),
                                                data: None,
                                            }),
                                        }
                                    }
                                } else {
                                    McpResponse {
                                        jsonrpc: "2.0".to_string(),
                                        id: request.id,
                                        result: None,
                                        error: Some(McpError {
                                            code: -32602,
                                            message: "Missing arguments".to_string(),
                                            data: None,
                                        }),
                                    }
                                }
                            }
                            _ => McpResponse {
                                jsonrpc: "2.0".to_string(),
                                id: request.id,
                                result: None,
                                error: Some(McpError {
                                    code: -32601,
                                    message: format!("Unknown tool: {}", name),
                                    data: None,
                                }),
                            },
                        }
                    } else {
                        McpResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(McpError {
                                code: -32602,
                                message: "Missing tool name".to_string(),
                                data: None,
                            }),
                        }
                    }
                } else {
                    McpResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(McpError {
                            code: -32602,
                            message: "Missing parameters".to_string(),
                            data: None,
                        }),
                    }
                }
            }
            _ => McpResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(McpError {
                    code: -32601,
                    message: format!("Unknown method: {}", request.method),
                    data: None,
                }),
            },
        }
    }

    async fn run(&self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            match serde_json::from_str::<McpRequest>(&line) {
                Ok(request) => {
                    let response = self.handle_request(request).await;
                    let response_json = serde_json::to_string(&response).unwrap();
                    writeln!(stdout, "{}", response_json)?;
                    stdout.flush()?;
                }
                Err(e) => {
                    let error_response = McpResponse {
                        jsonrpc: "2.0".to_string(),
                        id: json!(null),
                        result: None,
                        error: Some(McpError {
                            code: -32700,
                            message: format!("Parse error: {}", e),
                            data: None,
                        }),
                    };
                    let response_json = serde_json::to_string(&error_response).unwrap();
                    writeln!(stdout, "{}", response_json)?;
                    stdout.flush()?;
                }
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let api_key = env::var("KAGI_API_KEY").unwrap_or_else(|_| {
        eprintln!("Error: KAGI_API_KEY environment variable is required");
        std::process::exit(1);
    });

    let server = KagiMcpServer::new(api_key);
    server.run().await
}