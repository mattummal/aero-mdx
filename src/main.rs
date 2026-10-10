mod network;
mod protocol;
mod engine;
mod ipc;
mod telemetry;

use std::sync::Arc;
use ipc::ring_buffer::{create_tick_channel, EngineMetrics};

fn main() {
    // 1. Initialize Shared Lock-Free State
    let (producer, consumer) = create_tick_channel();
    let metrics = Arc::new(EngineMetrics::new());
    
    // 2. Spawn the Engine Thread (Pinned to a specific core)
    // Note: If you are on a Mac (M1/M2), core_affinity might not pin correctly. 
    // It works flawlessly on Linux/AWS EC2.
    let engine_metrics = Arc::clone(&metrics);
    std::thread::spawn(move || {
        engine::run_hot_path(1, consumer, engine_metrics);
    });

    // 3. Build Tokio Runtime for Network & Agent I/O
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async move {
        // Spawn the Agentic MCP Server
        let mcp_metrics = Arc::clone(&metrics);
        tokio::spawn(async move {
            telemetry::mcp_server::run_mcp_server(mcp_metrics).await;
        });

        // Start Live WebSocket Ingestion
        let ingestion_metrics = Arc::clone(&metrics);
        network::ws_client::run_ingestion(producer, ingestion_metrics).await;
    });
}
