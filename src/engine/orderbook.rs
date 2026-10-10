// src/engine/orderbook.rs

use crate::protocol::zero_copy::MarketTick;

/// A bounded array size covering expected price ticks
/// for a real system, I'd centre this arounf the daily open price
/// or use a fixed offset (e.g., index = price - base_price).
const MAX_PRICE_LEVELS: usize = 1_000_000;

pub struct OrderBook {
    bids: Box<[u32]>,
    asks: Box<[u32]>,
}

impl OrderBook {
    pub fn new() -> Self {
        // Allocate the 4 MB buffers on the heap once during initialization.
        // During trading, indexing directly into the slice performs no allocations.
        Self {
            bids: vec![0u32; MAX_PRICE_LEVELS].into_boxed_slice(),
            asks: vec![0u32; MAX_PRICE_LEVELS].into_boxed_slice(),
        }
    }

    /// inline this func aggresively so the compiler
    /// merges directly into the hot-loop
    #[inline(always)]
    pub fn apply_tick(&mut self, tick: &MarketTick) {
        // direct mapped caching: use modulo to ensure we never panic on out-of-bounds
        // in PROD: this would be an exact offset calculation
        let level = (tick.price_ticks as usize) % MAX_PRICE_LEVELS;

        if tick.is_bid {
            self.bids[level] = tick.size;
        } else {
            self.asks[level] = tick.size;
        }
    }
}


