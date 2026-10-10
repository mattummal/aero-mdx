// src/protocol/binance.rs

use serde::Deserialize;

/// Binance Partial Book Depth Stream Payload
/// We borrow strings directly from the network buffer (`&'a str`) to avoid allocations.

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BinanceDepthUpdate<'a> {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    #[serde(borrow)]
    pub bids: Vec<(&'a str, &'a str)>,
    #[serde(borrow)]
    pub asks: Vec<(&'a str, &'a str)>,
}

impl<'a> BinanceDepthUpdate<'a> {
    /// Fast string-to-fixed-point integer conversion.
    /// In a real HFT system, you would avoid `parse::<f64>()` entirely and write a 
    /// custom ASCII-to-integer parser to prevent floating-point unit (FPU) overhead.
    #[inline(always)]
    pub fn parse_price(price_str: &str) -> u64 {
        (price_str.parse::<f64>().unwrap_or(0.0) * 10_000.0) as u64
    }

    #[inline(always)]
    pub fn parse_size(size_str: &str) -> u32 {
        size_str.parse::<f32>().unwrap_or(0.0) as u32
    }
}
