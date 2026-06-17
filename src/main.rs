use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod parser;
mod indexer;

use indexer::Indexer;

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

fn send_response(id: Option<Value>, result: Option<Value>, error: Option<Value>) {
    let response = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result,
        error,
    };
    if let Ok(serialized) = serde_json::to_string(&response) {
        println!("{}", serialized);
        io::stdout().flush().unwrap();
    }
}

fn send_error(id: Option<Value>, code: i32, message: &str) {
    let error_obj = serde_json::json!({
        "code": code,
        "message": message
    });
    send_response(id, None, Some(error_obj));
}

fn handle_request(request: JsonRpcRequest) {
    let method = request.method.as_str();
    let id = request.id;

    eprintln!("[Info] Received request for method: {}", method);

    match method {
        "initialize" => {
            let result = serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "context-squeeze",
                    "version": "0.1.0"
                }
            });
            send_response(id, Some(result), None);
        }
        "notifications/initialized" => {
            // Notification has no ID, no response required
        }
        "tools/list" => {
            let result = serde_json::json!({
                "tools": [
                    {
                        "name": "get_repo_map",
                        "description": "Returns a compressed tree outline of the workspace structure showing folder layouts, file paths, class/interface definitions, and function signatures. Code body blocks are completely omitted to save token budget.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "path": {
                                    "type": "string",
                                    "description": "Target folder path to index (defaults to current working directory if omitted)"
                                }
                            }
                        }
                    },
                    {
                        "name": "get_code_item",
                        "description": "Extracts the complete implementation body of a specific symbol (class, function, interface, struct) inside a file.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "file_path": {
                                    "type": "string",
                                    "description": "Relative path to the source file"
                                },
                                "symbol_name": {
                                    "type": "string",
                                    "description": "Name of the class, function, or struct to retrieve"
                                }
                            },
                            "required": ["file_path", "symbol_name"]
                        }
                    },
                    {
                        "name": "search_code",
                        "description": "Searches for matching symbol names or signatures across the repository code files.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "query": {
                                    "type": "string",
                                    "description": "The search term or symbol name pattern to find"
                                }
                            },
                            "required": ["query"]
                        }
                    }
                ]
            });
            send_response(id, Some(result), None);
        }
        "tools/call" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    send_error(id, -32602, "Invalid params");
                    return;
                }
            };

            let tool_name = params.get("name").and_then(|v| v.as_str());
            let arguments = params.get("arguments").cloned().unwrap_or(Value::Null);

            match tool_name {
                Some("get_repo_map") => {
                    let target_path_str = arguments.get("path").and_then(|v| v.as_str()).unwrap_or(".");
                    let target_path = PathBuf::from(target_path_str);
                    
                    let indexer = Indexer::new(target_path);
                    let repo_map = indexer.build_repo_map();
                    
                    let result = serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": repo_map
                            }
                        ]
                    });
                    send_response(id, Some(result), None);
                }
                Some("get_code_item") => {
                    let file_path = match arguments.get("file_path").and_then(|v| v.as_str()) {
                        Some(f) => f,
                        None => {
                            send_error(id, -32602, "Missing file_path argument");
                            return;
                        }
                    };
                    let symbol_name = match arguments.get("symbol_name").and_then(|v| v.as_str()) {
                        Some(s) => s,
                        None => {
                            send_error(id, -32602, "Missing symbol_name argument");
                            return;
                        }
                    };

                    let indexer = Indexer::new(PathBuf::from("."));
                    let body = indexer.get_symbol_body(file_path, symbol_name);
                    
                    let text_response = match body {
                        Some(code) => format!("Symbol: {} in file {}\n\n```\n{}\n```", symbol_name, file_path, code),
                        None => format!("Symbol '{}' not found in file '{}'.", symbol_name, file_path),
                    };

                    let result = serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": text_response
                            }
                        ]
                    });
                    send_response(id, Some(result), None);
                }
                Some("search_code") => {
                    let query = match arguments.get("query").and_then(|v| v.as_str()) {
                        Some(q) => q,
                        None => {
                            send_error(id, -32602, "Missing query argument");
                            return;
                        }
                    };

                    let indexer = Indexer::new(PathBuf::from("."));
                    let search_results = indexer.search_symbols(query);
                    
                    let result = serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": search_results
                            }
                        ]
                    });
                    send_response(id, Some(result), None);
                }
                _ => {
                    send_error(id, -32601, "Method not found");
                }
            }
        }
        _ => {
            send_error(id, -32601, "Method not found");
        }
    }
}

fn main() {
    eprintln!("[Info] Starting Context-Squeeze MCP Server...");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line_content = match line {
            Ok(content) => content,
            Err(e) => {
                eprintln!("[Error] Failed to read line from stdin: {}", e);
                break;
            }
        };

        if line_content.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<JsonRpcRequest>(&line_content) {
            Ok(request) => {
                handle_request(request);
            }
            Err(e) => {
                eprintln!("[Error] Failed to parse request: {}", e);
                send_error(None, -32700, "Parse error");
            }
        }
    }
}
