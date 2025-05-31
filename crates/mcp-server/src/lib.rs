//! Generic Model Context Protocol (MCP) server library
//!
//! This crate provides a framework for building MCP servers that can be used
//! with AI assistants like Claude, GPT, and others.
//!
//! # Example
//!
//! ```no_run
//! use mcp_server::{McpServer, Tool, ToolHandler, ToolResult};
//! use serde_json::{json, Value};
//! use async_trait::async_trait;
//!
//! struct MyToolHandler;
//!
//! #[async_trait]
//! impl ToolHandler for MyToolHandler {
//!     async fn handle_tool(&self, name: &str, args: Value) -> ToolResult {
//!         match name {
//!             "echo" => {
//!                 let message = args.get("message").and_then(|v| v.as_str()).unwrap_or("");
//!                 Ok(vec![json!({"type": "text", "text": message})])
//!             }
//!             _ => Err(format!("Unknown tool: {}", name)),
//!         }
//!     }
//!
//!     fn get_tools(&self) -> Vec<Tool> {
//!         vec![Tool {
//!             name: "echo".to_string(),
//!             description: "Echo a message".to_string(),
//!             input_schema: json!({
//!                 "type": "object",
//!                 "properties": {
//!                     "message": {"type": "string"}
//!                 },
//!                 "required": ["message"]
//!             }),
//!         }]
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let handler = MyToolHandler;
//!     let server = McpServer::new("my-server", "1.0.0", handler);
//!     server.run().await?;
//!     Ok(())
//! }
//! ```

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub const MCP_VERSION: &str = "2024-11-05";

#[derive(Error, Debug)]
pub enum McpError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Protocol error: {0}")]
    Protocol(String),
    #[error("Tool error: {0}")]
    Tool(String),
}

pub type McpResult<T> = Result<T, McpError>;
pub type ToolResult = Result<Vec<Value>, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: Value,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpErrorResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpErrorResponse {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    pub tools: Option<Value>,
    pub resources: Option<Value>,
    pub prompts: Option<Value>,
}

#[async_trait]
pub trait ToolHandler: Send + Sync {
    async fn handle_tool(&self, name: &str, args: Value) -> ToolResult;
    fn get_tools(&self) -> Vec<Tool>;
}

pub struct McpServer<T: ToolHandler> {
    server_info: ServerInfo,
    tool_handler: T,
}

impl<T: ToolHandler> McpServer<T> {
    pub fn new(name: impl Into<String>, version: impl Into<String>, tool_handler: T) -> Self {
        Self {
            server_info: ServerInfo {
                name: name.into(),
                version: version.into(),
            },
            tool_handler,
        }
    }

    pub async fn run(&self) -> McpResult<()> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await?;
            
            if bytes_read == 0 {
                break; // EOF
            }

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match self.handle_line(line).await {
                Ok(response) => {
                    let response_json = serde_json::to_string(&response)?;
                    stdout.write_all(response_json.as_bytes()).await?;
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                }
                Err(e) => {
                    let error_response = McpResponse {
                        jsonrpc: "2.0".to_string(),
                        id: json!(null),
                        result: None,
                        error: Some(McpErrorResponse {
                            code: -32603,
                            message: format!("Internal error: {}", e),
                            data: None,
                        }),
                    };
                    let response_json = serde_json::to_string(&error_response)?;
                    stdout.write_all(response_json.as_bytes()).await?;
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                }
            }
        }

        Ok(())
    }

    pub fn run_sync(&self) -> McpResult<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line?;
            let line = line.trim();
            
            if line.is_empty() {
                continue;
            }

            let rt = tokio::runtime::Runtime::new()?;
            
            match rt.block_on(self.handle_line(line)) {
                Ok(response) => {
                    let response_json = serde_json::to_string(&response)?;
                    writeln!(stdout, "{}", response_json)?;
                    stdout.flush()?;
                }
                Err(e) => {
                    let error_response = McpResponse {
                        jsonrpc: "2.0".to_string(),
                        id: json!(null),
                        result: None,
                        error: Some(McpErrorResponse {
                            code: -32603,
                            message: format!("Internal error: {}", e),
                            data: None,
                        }),
                    };
                    let response_json = serde_json::to_string(&error_response)?;
                    writeln!(stdout, "{}", response_json)?;
                    stdout.flush()?;
                }
            }
        }

        Ok(())
    }

    async fn handle_line(&self, line: &str) -> McpResult<McpResponse> {
        let request: McpRequest = serde_json::from_str(line)?;
        self.handle_request(request).await
    }

    async fn handle_request(&self, request: McpRequest) -> McpResult<McpResponse> {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request).await,
            "tools/list" => self.handle_tools_list(request).await,
            "tools/call" => self.handle_tools_call(request).await,
            _ => Ok(McpResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(McpErrorResponse {
                    code: -32601,
                    message: format!("Method not found: {}", request.method),
                    data: None,
                }),
            }),
        }
    }

    async fn handle_initialize(&self, request: McpRequest) -> McpResult<McpResponse> {
        Ok(McpResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(json!({
                "protocolVersion": MCP_VERSION,
                "capabilities": Capabilities {
                    tools: Some(json!({})),
                    resources: None,
                    prompts: None,
                },
                "serverInfo": self.server_info
            })),
            error: None,
        })
    }

    async fn handle_tools_list(&self, request: McpRequest) -> McpResult<McpResponse> {
        let tools = self.tool_handler.get_tools();
        Ok(McpResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(json!({
                "tools": tools
            })),
            error: None,
        })
    }

    async fn handle_tools_call(&self, request: McpRequest) -> McpResult<McpResponse> {
        if let Some(params) = request.params {
            if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
                if let Some(args) = params.get("arguments") {
                    match self.tool_handler.handle_tool(name, args.clone()).await {
                        Ok(content) => Ok(McpResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(json!({
                                "content": content
                            })),
                            error: None,
                        }),
                        Err(e) => Ok(McpResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(McpErrorResponse {
                                code: -1,
                                message: e,
                                data: None,
                            }),
                        }),
                    }
                } else {
                    Ok(McpResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(McpErrorResponse {
                            code: -32602,
                            message: "Missing arguments parameter".to_string(),
                            data: None,
                        }),
                    })
                }
            } else {
                Ok(McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(McpErrorResponse {
                        code: -32602,
                        message: "Missing name parameter".to_string(),
                        data: None,
                    }),
                })
            }
        } else {
            Ok(McpResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(McpErrorResponse {
                    code: -32602,
                    message: "Missing parameters".to_string(),
                    data: None,
                }),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler;

    #[async_trait]
    impl ToolHandler for TestHandler {
        async fn handle_tool(&self, name: &str, _args: Value) -> ToolResult {
            match name {
                "test" => Ok(vec![json!({"type": "text", "text": "test result"})]),
                _ => Err(format!("Unknown tool: {}", name)),
            }
        }

        fn get_tools(&self) -> Vec<Tool> {
            vec![Tool {
                name: "test".to_string(),
                description: "A test tool".to_string(),
                input_schema: json!({"type": "object"}),
            }]
        }
    }

    #[tokio::test]
    async fn test_initialize() {
        let handler = TestHandler;
        let server = McpServer::new("test-server", "1.0.0", handler);
        
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "initialize".to_string(),
            params: None,
        };

        let response = server.handle_request(request).await.unwrap();
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_tools_list() {
        let handler = TestHandler;
        let server = McpServer::new("test-server", "1.0.0", handler);
        
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "tools/list".to_string(),
            params: None,
        };

        let response = server.handle_request(request).await.unwrap();
        assert!(response.result.is_some());
        
        if let Some(result) = response.result {
            if let Some(tools) = result.get("tools").and_then(|v| v.as_array()) {
                assert_eq!(tools.len(), 1);
                assert_eq!(tools[0]["name"], "test");
            }
        }
    }
}