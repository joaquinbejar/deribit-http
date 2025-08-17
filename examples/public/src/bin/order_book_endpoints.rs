//! Order Book Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public endpoints:
//! - `/public/get_order_book` - Order book by instrument name
//! - `/public/get_order_book_by_instrument_id` - Order book by instrument ID
//!
//! Usage: cargo run --bin order_book_endpoints

use deribit_http::DeribitHttpClient;
use std::env;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    info!("🚀 Deribit HTTP Client - Order Book Endpoints Example");
    info!("=====================================================");
    println!();

    // Determine if we should use testnet or production
    let use_testnet = env::var("DERIBIT_TESTNET")
        .map(|val| val.to_lowercase() == "true")
        .unwrap_or(true); // Default to testnet for safety

    info!(
        "🌐 Environment: {}",
        if use_testnet { "Testnet" } else { "Production" }
    );

    // Create HTTP client
    let client = DeribitHttpClient::new(use_testnet);
    info!(
        "✅ HTTP client created for {}: {}",
        if use_testnet { "testnet" } else { "production" },
        client.base_url()
    );
    println!();

    // =================================================================
    // 1. GET ORDER BOOK BY INSTRUMENT NAME (/public/get_order_book)
    // =================================================================
    info!("📖 1. GET ORDER BOOK BY INSTRUMENT NAME");
    info!("----------------------------------------");

    // Test with BTC-PERPETUAL (default depth)
    match client.get_order_book("BTC-PERPETUAL", None).await {
        Ok(order_book) => {
            info!("✅ Order book for {} retrieved successfully", order_book.instrument_name);
            info!("📊 Found {} bids and {} asks", order_book.bids.len(), order_book.asks.len());
            
            info!("📈 Top 5 Bids (Price, Amount):");
            for (i, bid) in order_book.bids.iter().take(5).enumerate() {
                info!("   {}. ${:.2} - {:.6} BTC", i + 1, bid[0], bid[1]);
            }
            
            info!("📉 Top 5 Asks (Price, Amount):");
            for (i, ask) in order_book.asks.iter().take(5).enumerate() {
                info!("   {}. ${:.2} - {:.6} BTC", i + 1, ask[0], ask[1]);
            }

            if !order_book.bids.is_empty() && !order_book.asks.is_empty() {
                let best_bid = order_book.bids[0][0];
                let best_ask = order_book.asks[0][0];
                let spread = best_ask - best_bid;
                let spread_percentage = (spread / best_bid) * 100.0;
                info!("💰 Best Bid: ${:.2} | Best Ask: ${:.2}", best_bid, best_ask);
                info!("📏 Spread: ${:.2} ({:.4}%)", spread, spread_percentage);
            }
        }
        Err(e) => {
            warn!("⚠️ Get order book for BTC-PERPETUAL error: {}", e);
            info!("💡 This may be expected if the instrument is not available");
        }
    }

    // Test with BTC-PERPETUAL (custom depth)
    match client.get_order_book("BTC-PERPETUAL", Some(10)).await {
        Ok(order_book) => {
            info!("✅ Order book for {} with depth 10 retrieved successfully", order_book.instrument_name);
            info!("📊 Found {} bids and {} asks with depth 10", order_book.bids.len(), order_book.asks.len());
            
            if order_book.bids.len() >= 3 {
                info!("📈 Bid depth analysis:");
                let total_bid_volume: f64 = order_book.bids.iter().map(|bid| bid[1]).sum();
                info!("   Total bid volume: {:.6} BTC", total_bid_volume);
                info!("   Price range: ${:.2} - ${:.2}", 
                     order_book.bids.last().unwrap()[0], 
                     order_book.bids.first().unwrap()[0]);
            }
            
            if order_book.asks.len() >= 3 {
                info!("📉 Ask depth analysis:");
                let total_ask_volume: f64 = order_book.asks.iter().map(|ask| ask[1]).sum();
                info!("   Total ask volume: {:.6} BTC", total_ask_volume);
                info!("   Price range: ${:.2} - ${:.2}", 
                     order_book.asks.first().unwrap()[0], 
                     order_book.asks.last().unwrap()[0]);
            }
        }
        Err(e) => {
            warn!("⚠️ Get order book for BTC-PERPETUAL with depth error: {}", e);
            info!("💡 This may be expected if the instrument is not available");
        }
    }

    // Test with ETH-PERPETUAL
    match client.get_order_book("ETH-PERPETUAL", Some(5)).await {
        Ok(order_book) => {
            info!("✅ Order book for {} retrieved successfully", order_book.instrument_name);
            info!("📊 Found {} bids and {} asks", order_book.bids.len(), order_book.asks.len());
            
            if !order_book.bids.is_empty() {
                info!("📈 Best bid: ${:.2} - {:.6} ETH", order_book.bids[0][0], order_book.bids[0][1]);
            }
            
            if !order_book.asks.is_empty() {
                info!("📉 Best ask: ${:.2} - {:.6} ETH", order_book.asks[0][0], order_book.asks[0][1]);
            }
        }
        Err(e) => {
            warn!("⚠️ Get order book for ETH-PERPETUAL error: {}", e);
            info!("💡 This may be expected if ETH-PERPETUAL is not available on testnet");
        }
    }

    // Test with a BTC future (if available)
    match client.get_order_book("BTC-29MAR24", Some(3)).await {
        Ok(order_book) => {
            info!("✅ Order book for BTC future retrieved successfully");
            info!("📊 Future order book: {} bids, {} asks", order_book.bids.len(), order_book.asks.len());
            
            if !order_book.bids.is_empty() && !order_book.asks.is_empty() {
                info!("💰 Future market - Best bid: ${:.2} | Best ask: ${:.2}", 
                     order_book.bids[0][0], order_book.asks[0][0]);
            }
        }
        Err(e) => {
            warn!("⚠️ Get order book for BTC future error: {}", e);
            info!("💡 This is expected if the specific future is not available");
        }
    }

    // Test with invalid instrument to demonstrate error handling
    match client.get_order_book("INVALID-INSTRUMENT", Some(1)).await {
        Ok(order_book) => {
            if order_book.bids.is_empty() && order_book.asks.is_empty() {
                info!("✅ Empty order book for invalid instrument (expected behavior)");
            } else {
                warn!("⚠️ Unexpected order book data found for invalid instrument");
            }
        }
        Err(e) => {
            info!("✅ Expected error for invalid instrument: {}", e);
            info!("💡 This demonstrates proper error handling for invalid instrument names");
        }
    }
    println!();

    // =================================================================
    // 2. GET ORDER BOOK BY INSTRUMENT ID (/public/get_order_book_by_instrument_id)
    // =================================================================
    info!("🆔 2. GET ORDER BOOK BY INSTRUMENT ID");
    info!("------------------------------------");

    // First, we need to get some instrument IDs by fetching instruments
    info!("🔍 Fetching available BTC instruments to get their IDs...");
    match client.get_instruments("BTC", None, Some(false)).await {
        Ok(instruments) => {
            if !instruments.is_empty() {
                info!("✅ Found {} BTC instruments", instruments.len());
                
                // Extract actual instrument IDs from the response
                let mut valid_ids = Vec::new();
                for instrument in instruments.iter().take(10) { // Check first 10 instruments
                    if let Some(id) = instrument.instrument_id {
                        valid_ids.push(id as u32);
                        if valid_ids.len() >= 5 {
                            break; // We only need a few for testing
                        }
                    }
                }
                
                if !valid_ids.is_empty() {
                    info!("🧪 Testing with actual instrument IDs: {:?}...", &valid_ids[..valid_ids.len().min(3)]);
                    
                    // Test with actual instrument IDs
                    for &test_id in valid_ids.iter().take(3) {
                        match client.get_order_book_by_instrument_id(test_id, Some(5)).await {
                            Ok(order_book) => {
                                info!("✅ Order book for instrument ID {} retrieved successfully", test_id);
                                info!("📊 Instrument: {} - {} bids, {} asks", 
                                     order_book.instrument_name,
                                     order_book.bids.len(), 
                                     order_book.asks.len());
                                
                                if !order_book.bids.is_empty() && !order_book.asks.is_empty() {
                                    info!("💰 ID {} - Best bid: ${:.2} | Best ask: ${:.2}", 
                                         test_id, order_book.bids[0][0], order_book.asks[0][0]);
                                }
                                
                                // Found a valid ID, no need to test others
                                break;
                            }
                            Err(e) => {
                                warn!("⚠️ Get order book for instrument ID {} error: {}", test_id, e);
                            }
                        }
                    }
                } else {
                    info!("💡 No instruments with IDs found for testing");
                    info!("🧪 Testing with fallback instrument IDs...");
                    
                    // Fallback to some realistic test IDs if no IDs found in instruments
                    let fallback_ids = [1, 2, 3, 4]; // More realistic starting IDs
                    
                    for &test_id in &fallback_ids {
                        match client.get_order_book_by_instrument_id(test_id, Some(5)).await {
                            Ok(order_book) => {
                                info!("✅ Order book for instrument ID {} retrieved successfully", test_id);
                                info!("📊 Instrument: {} - {} bids, {} asks", 
                                     order_book.instrument_name,
                                     order_book.bids.len(), 
                                     order_book.asks.len());
                                
                                if !order_book.bids.is_empty() && !order_book.asks.is_empty() {
                                    info!("💰 ID {} - Best bid: ${:.2} | Best ask: ${:.2}", 
                                         test_id, order_book.bids[0][0], order_book.asks[0][0]);
                                }
                                
                                // Found a valid ID, no need to test others
                                break;
                            }
                            Err(e) => {
                                info!("⚠️ Instrument ID {} not found: {}", test_id, e);
                            }
                        }
                    }
                }
            } else {
                info!("💡 No BTC instruments found for ID testing");
            }
        }
        Err(e) => {
            warn!("⚠️ Could not fetch instruments for ID testing: {}", e);
            info!("💡 This may indicate API issues or rate limiting");
        }
    }

    // Test with different depths using discovered instrument IDs (if available)
    // First try to get a valid instrument ID again for depth testing
    match client.get_instruments("BTC", None, Some(false)).await {
        Ok(instruments) => {
            // Find the first instrument with an ID
            if let Some(instrument) = instruments.iter().find(|inst| inst.instrument_id.is_some()) {
                if let Some(test_id) = instrument.instrument_id {
                    let test_id = test_id as u32;
                    
                    // Test with custom depth
                    match client.get_order_book_by_instrument_id(test_id, Some(15)).await {
                        Ok(order_book) => {
                            info!("✅ Order book by ID {} with depth 15 retrieved successfully", test_id);
                            info!("📊 Instrument: {} - {} bids, {} asks", 
                                 order_book.instrument_name,
                                 order_book.bids.len(), 
                                 order_book.asks.len());
                            
                            if order_book.bids.len() > 10 && order_book.asks.len() > 0 {
                                info!("📈 Deep market analysis:");
                                let mid_market = (order_book.bids[0][0] + order_book.asks[0][0]) / 2.0;
                                info!("   Mid-market price: ${:.2}", mid_market);
                                
                                let total_bid_volume: f64 = order_book.bids.iter().map(|bid| bid[1]).sum();
                                let total_ask_volume: f64 = order_book.asks.iter().map(|ask| ask[1]).sum();
                                info!("   Total volume - Bids: {:.6}, Asks: {:.6}", total_bid_volume, total_ask_volume);
                                
                                let imbalance = (total_bid_volume - total_ask_volume) / (total_bid_volume + total_ask_volume) * 100.0;
                                info!("   Order book imbalance: {:.2}% (positive = more bids)", imbalance);
                            }
                        }
                        Err(e) => {
                            info!("⚠️ Get order book by ID {} with depth error: {}", test_id, e);
                            info!("💡 This may be expected if the instrument is not currently tradeable");
                        }
                    }
                    
                    // Test with minimal depth (1)
                    match client.get_order_book_by_instrument_id(test_id, Some(1)).await {
                        Ok(order_book) => {
                            info!("✅ Order book by ID {} with minimal depth retrieved successfully", test_id);
                            info!("📊 Instrument: {} - Top level only", order_book.instrument_name);
                            
                            if !order_book.bids.is_empty() && !order_book.asks.is_empty() {
                                info!("💰 Top of book - Bid: ${:.2} | Ask: ${:.2}", 
                                     order_book.bids[0][0], order_book.asks[0][0]);
                            }
                        }
                        Err(e) => {
                            info!("⚠️ Get order book by ID {} error: {}", test_id, e);
                            info!("💡 This may be expected if the instrument is not currently tradeable");
                        }
                    }
                } else {
                    info!("💡 No instrument IDs available for additional depth testing");
                }
            } else {
                info!("💡 No instruments with IDs found for depth testing");
            }
        }
        Err(e) => {
            info!("⚠️ Could not fetch instruments for depth testing: {}", e);
            info!("💡 Skipping additional depth tests due to API issues");
        }
    }

    // Test with invalid instrument ID to demonstrate error handling
    match client.get_order_book_by_instrument_id(999999, Some(5)).await {
        Ok(order_book) => {
            if order_book.bids.is_empty() && order_book.asks.is_empty() {
                info!("✅ Empty order book for invalid instrument ID (expected behavior)");
            } else {
                warn!("⚠️ Unexpected order book data found for invalid instrument ID");
            }
        }
        Err(e) => {
            info!("✅ Expected error for invalid instrument ID: {}", e);
            info!("💡 This demonstrates proper error handling for invalid instrument IDs");
        }
    }
    println!();

    // =================================================================
    // ORDER BOOK DATA EXPLANATION
    // =================================================================
    info!("📚 ORDER BOOK DATA EXPLANATION");
    info!("==============================");
    info!("📈 Bids: Buy orders sorted by price (highest first)");
    info!("📉 Asks: Sell orders sorted by price (lowest first)");
    info!("💰 Price: The price level in USD");
    info!("📦 Amount: The quantity available at that price level");
    info!("📏 Spread: Difference between best ask and best bid prices");
    info!("🎯 Depth: Number of price levels to include (default: 5)");
    info!("⚖️ Liquidity: Total volume available at different price levels");
    info!("🔄 Real-time: Order book data reflects current market conditions");
    println!();

    // =================================================================
    // TRADING INSIGHTS
    // =================================================================
    info!("🧠 TRADING INSIGHTS FROM ORDER BOOK DATA");
    info!("==========================================");
    info!("💡 Tight spreads indicate liquid markets with active trading");
    info!("📊 Large order book depth suggests good liquidity for larger trades");
    info!("⚖️ Order book imbalance can indicate short-term price direction");
    info!("🎯 Best bid/ask prices show the current market for immediate execution");
    info!("📈 Higher bid volumes may indicate buying pressure");
    info!("📉 Higher ask volumes may indicate selling pressure");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED ORDER BOOK ENDPOINTS");
    info!("===========================================");
    info!("📖 /public/get_order_book - Order book data by instrument name");
    info!("🆔 /public/get_order_book_by_instrument_id - Order book data by instrument ID");
    println!();

    info!("🎉 Order book endpoints example completed successfully!");
    info!("💡 Tip: Use order book data for market analysis and optimal trade execution");
    info!("🔗 Order book depth is crucial for understanding market liquidity");
    info!("📊 Monitor spread and volume for trading opportunities and market health");
    info!("⚡ Consider using instrument IDs for faster lookups when available");

    Ok(())
}