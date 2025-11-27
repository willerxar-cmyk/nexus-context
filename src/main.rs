use anyhow::Result;
use nexus_context::db::VectorDB;
use nexus_context::embeddings::Embedder;
use nexus_context::watcher;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead};
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

struct AppState {
    db: VectorDB,
    embedder: Mutex<Embedder>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logs
    tracing_subscriber::fmt::init();

    // Load Model (Blocking)
    // eprintln to avoid messing up stdout JSON-RPC
    eprintln!("Loading Embedding Model (BGE-Base-v1.5)...");
    let embedder = Embedder::new()?;
    eprintln!("Model Loaded.");

    // Init DB
    let db = VectorDB::new("data/vectors.json").await?;
    let app_state = Arc::new(AppState {
        db,
        embedder: Mutex::new(embedder),
    });

    // MCP Loop (Stdio)
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        // Ignore empty lines
        if line.trim().is_empty() { continue; }
        
        if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(&line) {
            handle_request(req, app_state.clone()).await?;
        }
    }

    Ok(())
}

async fn handle_request(req: JsonRpcRequest, state: Arc<AppState>) -> Result<()> {
    let response = match req.method.as_str() {
        "initialize" => {
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "nexus-context",
                    "version": "0.1.0"
                }
            })
        }
        "tools/list" => {
            json!({
                "tools": [
                    {
                        "name": "search_context",
                        "description": "Search the local vector database for relevant code or conversations",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "query": { "type": "string" }
                            },
                            "required": ["query"]
                        }
                    },
                    {
                        "name": "add_memory",
                        "description": "Add a new memory to the vector database",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "text": { "type": "string" },
                                "metadata": { "type": "string" }
                            },
                            "required": ["text"]
                        }
                    }
                ]
            })
        }
        "tools/call" => {
            if let Some(params) = req.params {
                let name = params["name"].as_str().unwrap_or("");
                let args = &params["arguments"];
                
                match name {
                    "search_context" => {
                        let query = args["query"].as_str().unwrap_or("");
                        // Generate embedding
                        let mut embedder = state.embedder.lock().await;
                        let vector = embedder.embed(query)?;
                        drop(embedder); // Release lock
                        
                        // Search DB
                        let results = state.db.search(vector, 5).await?;
                        // Format results for display
                        let result_str = format!("{:?}", results); 
                        json!({ "content": [{ "type": "text", "text": result_str }] })
                    }
                    "add_memory" => {
                        let text = args["text"].as_str().unwrap_or("");
                        let meta = args["metadata"].as_str().unwrap_or("{}");
                        
                        let mut embedder = state.embedder.lock().await;
                        let vector = embedder.embed(text)?;
                        drop(embedder);
                        
                        state.db.add("manual_id", text, vector, meta).await?;
                        json!({ "content": [{ "type": "text", "text": "Memory added." }] })
                    }
                    _ => json!({ "error": "Tool not found", "isError": true })
                }
            } else {
                json!({ "error": "Missing params", "isError": true })
            }
        }
        _ => json!({ "method": req.method }) // Echo for debugging
    };

    let resp_msg = json!({
        "jsonrpc": "2.0",
        "id": req.id,
        "result": response
    });

    println!("{}", serde_json::to_string(&resp_msg)?);
    Ok(())
}
