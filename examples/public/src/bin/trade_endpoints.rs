//! Trade Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public endpoints:
//! - `/public/get_last_trades_by_currency` - Recent trades by currency
//! - `/public/get_last_trades_by_currency_and_time` - Recent trades by currency and time
//! - `/public/get_last_trades_by_instrument` - Recent trades by instrument
//! - `/public/get_last_trades_by_instrument_and_time` - Recent trades by instrument and time
//!
//! Usage: cargo run --bin trade_endpoints

use deribit_base::prelude::setup_logger;
use deribit_http::DeribitHttpClient;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_logger();
    info!("🚀 Deribit HTTP Client - Trade Endpoints Example");
    info!("=================================================");
    println!();

    // Create HTTP client
    let client = DeribitHttpClient::new();

    // =================================================================
    // 1. GET LAST TRADES BY CURRENCY (/public/get_last_trades_by_currency)
    // =================================================================
    info!("💱 1. GET LAST TRADES BY CURRENCY");
    info!("----------------------------------");

    // Test with BTC trades (all instruments)
    match client
        .get_last_trades_by_currency("BTC", None, Some(10), Some(false), Some("default"))
        .await
    {
        Ok(trades_response) => {
            info!("✅ Trades for BTC retrieved successfully");
            info!("📊 Found {} trade records:", trades_response.trades.len());

            for (i, trade) in trades_response.trades.iter().take(5).enumerate() {
                info!(
                    "   {}. {} - {} BTC at ${:.2}",
                    i + 1,
                    trade.instrument_name,
                    trade.amount,
                    trade.price
                );

                info!(
                    "      Direction: {} - Timestamp: {}",
                    trade.direction, trade.timestamp
                );

                info!("      Trade ID: {}", trade.trade_id);

                info!("      Index Price: ${:.2}", trade.index_price);
            }

            if trades_response.trades.len() > 5 {
                info!(
                    "💡 Showing first 5 of {} BTC trades",
                    trades_response.trades.len()
                );
            }

            if trades_response.has_more {
                info!("🔗 More trades available for pagination");
            }

            if trades_response.trades.is_empty() {
                info!("💡 No trade data available for BTC");
            }
        }
        Err(e) => {
            warn!("⚠️ Get trades for BTC error: {}", e);
            info!("💡 This may be expected if no recent trades are available");
        }
    }

    // Test with BTC futures only
    match client
        .get_last_trades_by_currency("BTC", Some("future"), Some(5), Some(false), Some("default"))
        .await
    {
        Ok(trades_response) => {
            info!("✅ BTC future trades retrieved successfully");
            info!(
                "📊 Found {} future trade records",
                trades_response.trades.len()
            );

            for (i, trade) in trades_response.trades.iter().enumerate() {
                info!(
                    "   {}. Future trade: {} - {} BTC at ${:.2}",
                    i + 1,
                    trade.instrument_name,
                    trade.amount,
                    trade.price
                );

                info!(
                    "      Direction: {} - Timestamp: {}",
                    trade.direction, trade.timestamp
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC future trades error: {}", e);
            info!("💡 This may be expected if no future trades are available");
        }
    }

    // Test with ETH trades
    match client
        .get_last_trades_by_currency("ETH", None, Some(3), Some(false), Some("default"))
        .await
    {
        Ok(trades_response) => {
            info!("✅ ETH trades retrieved successfully");
            info!(
                "📊 Found {} ETH trade records",
                trades_response.trades.len()
            );

            for trade in &trades_response.trades {
                info!(
                    "   ETH trade: {} - {} ETH at ${:.2} ({})",
                    trade.instrument_name, trade.amount, trade.price, trade.direction
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get ETH trades error: {}", e);
            info!("💡 This may be expected if ETH trades are limited on testnet");
        }
    }
    println!();

    // =================================================================
    // 2. GET LAST TRADES BY CURRENCY AND TIME (/public/get_last_trades_by_currency_and_time)
    // =================================================================
    info!("⏰ 2. GET LAST TRADES BY CURRENCY AND TIME");
    info!("------------------------------------------");

    // Get current timestamp and set a time range (last hour)
    let end_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let start_timestamp = end_timestamp - (60 * 60 * 1000); // 1 hour ago

    info!(
        "🕐 Time range: {} to {} (last hour)",
        start_timestamp, end_timestamp
    );

    match client
        .get_last_trades_by_currency_and_time(
            "BTC",
            start_timestamp,
            end_timestamp,
            None,
            Some(10),
            Some(false),
            Some("default"),
        )
        .await
    {
        Ok(trades_response) => {
            info!("✅ BTC trades in time range retrieved successfully");
            info!(
                "📊 Found {} trade records in the last hour:",
                trades_response.trades.len()
            );

            for (i, trade) in trades_response.trades.iter().take(3).enumerate() {
                info!(
                    "   {}. {} - {} BTC at ${:.2}",
                    i + 1,
                    trade.instrument_name,
                    trade.amount,
                    trade.price
                );

                info!(
                    "      Direction: {} - Timestamp: {}",
                    trade.direction, trade.timestamp
                );
            }

            if trades_response.trades.len() > 3 {
                info!(
                    "💡 Showing first 3 of {} BTC trades in time range",
                    trades_response.trades.len()
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC trades by time error: {}", e);
            info!("💡 This may be expected if no trades occurred in the specified time range");
        }
    }

    // Test with ETH and futures in time range
    match client
        .get_last_trades_by_currency_and_time(
            "ETH",
            start_timestamp,
            end_timestamp,
            Some("future"),
            Some(5),
            Some(false),
            Some("default"),
        )
        .await
    {
        Ok(trades_response) => {
            info!("✅ ETH future trades in time range retrieved successfully");
            info!(
                "📊 Found {} ETH future trade records",
                trades_response.trades.len()
            );

            for trade in &trades_response.trades {
                info!(
                    "   ETH future trade: {} - {} ETH at ${:.2}",
                    trade.instrument_name, trade.amount, trade.price
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get ETH future trades by time error: {}", e);
            info!("💡 This may be expected if no ETH future trades occurred in the time range");
        }
    }
    println!();

    // =================================================================
    // 3. GET LAST TRADES BY INSTRUMENT (/public/get_last_trades_by_instrument)
    // =================================================================
    info!("🎯 3. GET LAST TRADES BY INSTRUMENT");
    info!("-----------------------------------");

    // Test with BTC-PERPETUAL
    match client
        .get_last_trades("BTC-PERPETUAL", Some(10), Some(false))
        .await
    {
        Ok(trades) => {
            info!("✅ Trades for BTC-PERPETUAL retrieved successfully");
            info!("📊 Found {} trade records for BTC-PERPETUAL:", trades.len());

            for (i, trade) in trades.iter().take(5).enumerate() {
                info!(
                    "   {}. Trade at {} - {} BTC at ${:.2}",
                    i + 1,
                    trade.timestamp,
                    trade.amount,
                    trade.price
                );

                info!("      Direction: {}", trade.direction);
            }

            if trades.len() > 5 {
                info!(
                    "💡 Showing first 5 of {} BTC-PERPETUAL trades",
                    trades.len()
                );
            }

            if trades.is_empty() {
                info!("💡 No trade data available for BTC-PERPETUAL");
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC-PERPETUAL trades error: {}", e);
            info!("💡 This may be expected if no recent trades are available for this instrument");
        }
    }

    // Test with ETH-PERPETUAL
    match client
        .get_last_trades("ETH-PERPETUAL", Some(5), Some(false))
        .await
    {
        Ok(trades) => {
            info!("✅ Trades for ETH-PERPETUAL retrieved successfully");
            info!("📊 Found {} trade records for ETH-PERPETUAL", trades.len());

            for (i, trade) in trades.iter().enumerate() {
                info!(
                    "   {}. {} ETH at ${:.2} ({})",
                    i + 1,
                    trade.amount,
                    trade.price,
                    trade.direction
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get ETH-PERPETUAL trades error: {}", e);
            info!("💡 This may be expected if ETH-PERPETUAL trades are not available");
        }
    }

    // Test with a BTC future (if available)
    match client
        .get_last_trades("BTC-29MAR24", Some(3), Some(false))
        .await
    {
        Ok(trades) => {
            info!("✅ Trades for BTC future retrieved successfully");
            info!("📊 Found {} trade records for BTC future", trades.len());

            for trade in &trades {
                info!(
                    "   Future trade: {} BTC at ${:.2} - {}",
                    trade.amount, trade.price, trade.timestamp
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC future trades error: {}", e);
            info!("💡 This is expected if the specific future is not available or has no trades");
        }
    }

    // Test with invalid instrument to demonstrate error handling
    match client
        .get_last_trades("INVALID-INSTRUMENT", Some(1), Some(false))
        .await
    {
        Ok(trades) => {
            if trades.is_empty() {
                info!("✅ Empty result for invalid instrument (expected behavior)");
            } else {
                warn!(
                    "⚠️ Unexpected trades found for invalid instrument: {}",
                    trades.len()
                );
            }
        }
        Err(e) => {
            info!("✅ Expected error for invalid instrument: {}", e);
            info!("💡 This demonstrates proper error handling for invalid instrument names");
        }
    }
    println!();

    // =================================================================
    // 4. GET LAST TRADES BY INSTRUMENT AND TIME (/public/get_last_trades_by_instrument_and_time)
    // =================================================================
    info!("🎯⏰ 4. GET LAST TRADES BY INSTRUMENT AND TIME");
    info!("----------------------------------------------");

    // Test with BTC-PERPETUAL in time range
    match client
        .get_last_trades_by_instrument_and_time(
            "BTC-PERPETUAL",
            start_timestamp,
            end_timestamp,
            Some(10),
            Some(false),
            Some("default"),
        )
        .await
    {
        Ok(trades_response) => {
            info!("✅ BTC-PERPETUAL trades in time range retrieved successfully");
            info!(
                "📊 Found {} trade records for BTC-PERPETUAL in the last hour:",
                trades_response.trades.len()
            );

            for (i, trade) in trades_response.trades.iter().take(3).enumerate() {
                info!(
                    "   {}. Trade at {} - {} BTC at ${:.2}",
                    i + 1,
                    trade.timestamp,
                    trade.amount,
                    trade.price
                );

                info!(
                    "      Direction: {} - Trade seq: {}",
                    trade.direction, trade.trade_seq
                );
            }

            if trades_response.trades.len() > 3 {
                info!(
                    "💡 Showing first 3 of {} BTC-PERPETUAL trades in time range",
                    trades_response.trades.len()
                );
            }

            if trades_response.has_more {
                info!("🔗 More trades available for this time range");
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC-PERPETUAL trades by time error: {}", e);
            info!(
                "💡 This may be expected if no trades occurred for this instrument in the time range"
            );
        }
    }

    // Test with ETH-PERPETUAL in time range
    match client
        .get_last_trades_by_instrument_and_time(
            "ETH-PERPETUAL",
            start_timestamp,
            end_timestamp,
            Some(5),
            Some(false),
            Some("default"),
        )
        .await
    {
        Ok(trades_response) => {
            info!("✅ ETH-PERPETUAL trades in time range retrieved successfully");
            info!(
                "📊 Found {} trade records for ETH-PERPETUAL",
                trades_response.trades.len()
            );

            for trade in &trades_response.trades {
                info!(
                    "   ETH-PERPETUAL trade: {} ETH at ${:.2} ({})",
                    trade.amount, trade.price, trade.direction
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get ETH-PERPETUAL trades by time error: {}", e);
            info!("💡 This may be expected if no ETH-PERPETUAL trades occurred in the time range");
        }
    }

    // Test with wider time range (last 24 hours)
    let start_timestamp_24h = end_timestamp - (24 * 60 * 60 * 1000); // 24 hours ago
    info!(
        "🕐 Extended time range: {} to {} (last 24 hours)",
        start_timestamp_24h, end_timestamp
    );

    match client
        .get_last_trades_by_instrument_and_time(
            "BTC-PERPETUAL",
            start_timestamp_24h,
            end_timestamp,
            Some(5),
            Some(false),
            Some("default"),
        )
        .await
    {
        Ok(trades_response) => {
            info!("✅ BTC-PERPETUAL trades in 24h range retrieved successfully");
            info!(
                "📊 Found {} trade records in the last 24 hours",
                trades_response.trades.len()
            );

            for (i, trade) in trades_response.trades.iter().take(2).enumerate() {
                info!(
                    "   {}. Recent trade: {} BTC at ${:.2} ({})",
                    i + 1,
                    trade.amount,
                    trade.price,
                    trade.direction
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get BTC-PERPETUAL trades in 24h range error: {}", e);
            info!("💡 This may indicate API rate limits or no data in the extended range");
        }
    }
    println!();

    // =================================================================
    // TRADE DATA EXPLANATION
    // =================================================================
    info!("📚 TRADE DATA EXPLANATION");
    info!("==========================");
    info!("💱 Trade Direction: 'buy' or 'sell' indicating the taker's side");
    info!("💰 Amount: The trade size in the base currency (BTC, ETH, etc.)");
    info!("💲 Price: The execution price in USD");
    info!("🔢 Trade Sequence: Unique sequence number for ordering trades");
    info!("⚡ Liquidation: Indicates if the trade was part of a liquidation event");
    info!("📦 Block Trade: Large trades executed off the order book");
    info!("💡 Index Price: Reference price used for mark-to-market calculations");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED TRADE ENDPOINTS");
    info!("=====================================");
    info!("💱 /public/get_last_trades_by_currency - Recent trades filtered by currency");
    info!(
        "⏰ /public/get_last_trades_by_currency_and_time - Recent trades by currency with time range"
    );
    info!("🎯 /public/get_last_trades_by_instrument - Recent trades filtered by instrument");
    info!(
        "🎯⏰ /public/get_last_trades_by_instrument_and_time - Recent trades by instrument with time range"
    );
    println!();

    info!("🎉 Trade endpoints example completed successfully!");
    info!("💡 Tip: Use these endpoints to analyze recent market activity and trading patterns");
    info!("🔗 Trade data is essential for market analysis, backtesting, and monitoring execution");
    info!(
        "📊 Filter by instrument type (future/option/perpetual) and time ranges for specific analysis"
    );

    Ok(())
}
