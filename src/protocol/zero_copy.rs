// src/protocol/zero_copy.rs

/// A compact, heap-free representation of a market update.
/// Using fixed-point integers for prices to avoid floating-point math overhead 
/// and precision loss in the hot path.

#[derive(Debug, Clone, Copy, Default)]
pub struct MarketTick {
    pub instrument_id: u16,
    pub price_ticks: u64, // e.g., $50,000.50 becomes 500005000 (scaled by 10^4)
    pub size: u32,
    pub is_bid: bool,
    pub timestamp_ns: u64,
}

