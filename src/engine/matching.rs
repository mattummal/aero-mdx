// src/engine/matching.rs

#[allow(dead_code)]
pub struct ExecutionEngine {
    pub trades_executed: u64,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        Self { trades_executed: 0 }
    }
    
    #[allow(dead_code)]
    #[inline(always)]
    pub fn try_match(&mut self, _price: u64, _size: u32, _is_buy: bool) -> bool {
        // placeholder: look at the OrderBook bids/asks at the requested price.
        // if volume exists, decrement the OrderBook size and increment trades_executed.
        self.trades_executed += 1;
        true
    }
}
