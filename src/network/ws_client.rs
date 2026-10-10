// src/network/ws_client.rs

use crate::ipc::ring_buffer::EngineMetrics;
use crate::protocol::binance::BinanceDepthUpdate;
use crate::protocol::zero_copy::MarketTick;

use futures_util::StreamExt;
use rtrb::Producer;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

const BINANCE_WS_URL: &str = "wss://stream.binance.com:9443/ws/btcusdt@depth20@100ms";

pub async fn run_ingestion(
    mut producer: Producer<MarketTick>,
    metrics: Arc<EngineMetrics>,
) {
    let url = Url::parse(BINANCE_WS_URL).expect("Invalid WebSocket URL");
    
    println!("Connecting to Binance WebSocket: {}", BINANCE_WS_URL);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket connected successfully.");

    let (_, mut read) = ws_stream.split();

    // The asynchronous read loop
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(payload)) => {
                let timestamp_ns = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64;

                // Deserialize zero-copy directly from the payload string
                if let Ok(update) = serde_json::from_str::<BinanceDepthUpdate>(&payload) {
                    
                    // Process Bids
                    for (price_str, size_str) in update.bids {
                        let tick = MarketTick {
                            instrument_id: 1, // BTCUSDT
                            price_ticks: BinanceDepthUpdate::parse_price(price_str),
                            size: BinanceDepthUpdate::parse_size(size_str),
                            is_bid: true,
                            timestamp_ns,
                        };
                        push_to_ring(&mut producer, tick, &metrics);
                    }

                    // Process Asks
                    for (price_str, size_str) in update.asks {
                        let tick = MarketTick {
                            instrument_id: 1,
                            price_ticks: BinanceDepthUpdate::parse_price(price_str),
                            size: BinanceDepthUpdate::parse_size(size_str),
                            is_bid: false,
                            timestamp_ns,
                        };
                        push_to_ring(&mut producer, tick, &metrics);
                    }
                }
            }
            Ok(Message::Ping(_)) => {
                // Tokio-tungstenite handles Pong automatically
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

/// Helper function to push to the lock-free queue and track drops
#[inline(always)]
fn push_to_ring(
    producer: &mut Producer<MarketTick>,
    tick: MarketTick,
    metrics: &Arc<EngineMetrics>,
) {
    if producer.is_full() {
        metrics.dropped_ticks.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    } else {
        // Safe to unwrap because we just checked if it was full
        producer.push(tick).unwrap();
    }
}
