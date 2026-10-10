use crate::ipc::ring_buffer::EngineMetrics;
use std::sync::Arc;
use serde_json::{json, Value};
use tokio::io::{self, AsyncBufReadExt, BufReader};

pub async fn run_mcp_server(metrics: Arc<EngineMetrics>) {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    let init_msg = json!({
        "jsonrpc": "2.0",
        "id": "1",
        "result": {
            "capabilities": { "tools": {} },
            "serverInfo": { "name": "aero-mdx-ops", "version": "0.1.0" }
        }
    });
    println!("{}", init_msg);

    while reader.read_line(&mut line).await.unwrap() > 0 {
        if let Ok(req) = serde_json::from_str::<Value>(&line) {
            if req["method"] == "tools/list" {
                let response = json!({
                    "jsonrpc": "2.0",
                    "id": req["id"],
                    "result": {
                        "tools": [{
                            "name": "get_engine_metrics",
                            "description": "Returns real-time latency and throughput metrics of the Rust trading engine.",
                            "inputSchema": { "type": "object", "properties": {} }
                        }]
                    }
                });
                println!("{}", response);
            } else if req["method"] == "tools/call" {
                if req["params"]["name"] == "get_engine_metrics" {
                    let msgs = metrics.messages_processed.load(std::sync::atomic::Ordering::Relaxed);
                    let latency = metrics.current_latency_ns.load(std::sync::atomic::Ordering::Relaxed);
                    let drops = metrics.dropped_ticks.load(std::sync::atomic::Ordering::Relaxed);

                    let response = json!({
                        "jsonrpc": "2.0",
                        "id": req["id"],
                        "result": {
                            "content": [{
                                "type": "text",
                                "text": format!("Throughput: {} msgs, Batch Latency: {} ns, Drops: {}", msgs, latency, drops)
                            }]
                        }
                    });
                    println!("{}", response);
                }
            }
        }
        line.clear();
    }
}
