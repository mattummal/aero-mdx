// src/engine/mod.rs

pub mod orderbook;
pub mod matching;
// pub mod arena; // Uncomment when building out custom memory allocators

use crate::protocol::zero_copy::MarketTick;
use crate::ipc::ring_buffer::EngineMetrics;
use orderbook::OrderBook;
use matching::ExecutionEngine;

use rtrb::Consumer;
use std::sync::Arc;
use std::time::Instant;

pub fn run_hot_path(
    core_id: usize,
    mut consumer: Consumer<MarketTick>,
    metrics: Arc<EngineMetrics>,
) {
    // 1. Thread Pinning
    // Binds this OS thread to a specific physical core, bypassing the OS scheduler.
    if let Some(core_ids) = core_affinity::get_core_ids() {
        if let Some(core) = core_ids.into_iter().find(|c| c.id == core_id) {
            core_affinity::set_for_current(core);
            println!("Engine Hot-Path successfully pinned to CPU core: {}", core.id);
        }
    }

    let mut book = OrderBook::new();
    let mut _matcher = ExecutionEngine::new();
    let mut batch_count = 0;

    // 2. The Spin Loop
    loop {
        // consumer.slots() returns the exact number of available elements,
        // avoiding the error of asking for a fixed chunk size that isn't ready.
        let available = consumer.slots();
        
        if available > 0 {
            if let Ok(chunk) = consumer.read_chunk(available) {
                let start = Instant::now();
                let (first, second) = chunk.as_slices();

                // Process the contiguous slices sequentially
                for tick in first.iter().chain(second.iter()) {
                    book.apply_tick(tick);
                }
                
                let elapsed = start.elapsed().as_nanos() as usize;
                
                // Advance the ring buffer's internal read pointer
                chunk.commit_all();

                // 3. Telemetry Updates
                // We batch atomic updates to prevent cache-line ping-pong with the MCP thread
                batch_count += available;
                if batch_count >= 1024 {
                    metrics.add_processed(batch_count);
                    metrics.record_latency(elapsed);
                    batch_count = 0;
                }
            }
        } else {
            // CPU pause instruction (x86 PAUSE or ARM YIELD). 
            // Reduces power consumption and prevents pipeline flushing without yielding to the OS.
            std::hint::spin_loop();
        }
    }
}
