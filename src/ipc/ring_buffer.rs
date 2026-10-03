// src/ipc/ring_buffer.rs

/// 2. The Inter-Process Communication (IPC) Layer
/// Create src/ipc/ring_buffer.rs. This file sets up the Single-Producer Single-Consumer (SPSC) queue
/// that bridges async Tokio network thread and
/// pinned engine thread, along with the lock-free telemetry.

use crate::protocol::zero_copy::MarketTick;
use rtrb::{Consumer, Producer, RingBuffer};
use std::sync::atomic::{AtomicUsize, Ordering};


/// Must be power of 2 for optimal bitwise masking by the CPU
/// 131,072 slots * 25 bytes per tick = ~3.2MB (Fits nicely in L3 cache)

pub const RING_BUFFER_SIZE: usize = 131_072;

pub fn create_tick_channel() -> (Producer<MarketTick>, Consumer<MarketTick>) {
    RingBuffer::new(RING_BUFFER_SIZE)
}


/// Global metrics updated by the Hot Path & read by the MCP server
/// Use Atomics to ensure the telemetry thread never blocks the trading engine
pub struct EngineMetrics {
    pub messages_processed: AtomicUsize,
    pub current_latency_ns: AtomicUsize,
    pub dropped_ticks: AtomicUsize,
}

impl EngineMetrics {
    pub fn new() -> Self {
        Self {
            messages_processed: AtomicUsize::new(0),
            current_latency_ns: AtomicUsize::new(0),
            dropped_ticks: AtomicUsize::new(0)
        }
    }

    #[inline(always)]
    pub fn record_latency(&self, latency_ns: usize) {
        self.current_latency_ns.store(latency_ns, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn add_processed(&self, count: usize) {
        self.messages_processed.fetch_add(count, Ordering::Relaxed);
    }

}
