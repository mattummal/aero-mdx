// src/engine/orderbook.rs

use crate::protocol::zero_copy::MarketTick;

/// A bounded array size covering expected price ticks
/// for a real system, I'd centre this arounf the daily open price
/// or use a fixed offset (e.g., index = price - base_price).
const MAX_PRICE_LEVELS: usize = 1_000_000;

pub struct OrderBook {
    bids: [u32; MAX_PRICE_LEVELS],
    asks: [u32; MAX_PRICE_LEVELS],
}

impl OrderBook {
    pub fn new() -> Self {
        // use a Box here purely for initilisation so we don't
        // overflow the stack on startup. Once initialised, the
        // memory box remains static
        Self {
            bids: vec![0; MAX_PRICE_LEVELS].try_into().unwrap(),
            asks: vec![0; MAX_PRICE_LEVELS].try_into().unwrap(),
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


