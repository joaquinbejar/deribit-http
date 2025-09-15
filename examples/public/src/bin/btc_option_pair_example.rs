//! BTC Option Pair Example
//!
//! This example demonstrates how to download BTC option pairs for a specific expiry date
//! using the get_options_pair method. It shows:
//! - Fetching all BTC option pairs from Deribit grouped by strike price
//! - Displaying call and put options side by side
//! - Analyzing spreads and Greeks for each strike
//! - Calculating combined metrics for option pairs
//!
//! Usage: cargo run --bin btc_option_pair_example

use deribit_http::prelude::*;
use tracing::{info, warn};
use deribit_http::utils::get_tomorrow_deribit_format;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_logger();
    let expiry = get_tomorrow_deribit_format();
    let target_expiry = expiry.as_str();
    
    info!("🚀 Deribit HTTP Client - BTC Option Pair Example");
    info!("===============================================");
    info!("📅 Target Expiry: {target_expiry}");
    println!();

    // Create HTTP client
    let client = DeribitHttpClient::new();

    // =================================================================
    // 1. FETCH BTC OPTION PAIRS BY STRIKE PRICE
    // =================================================================
    info!("📋 1. FETCHING BTC OPTION PAIRS FOR {}", target_expiry);
    info!("-----------------------------------------------");
    // Fetch option pairs for the target expiry
    let option_pairs = client.get_options_pair("BTC", target_expiry).await?;
    info!(
        "✅ Successfully fetched {} option pairs",
        option_pairs.len()
    );

    if option_pairs.is_empty() {
        warn!("⚠️ No option pairs found for expiry date {}", target_expiry);
        return Ok(());
    }

    // =================================================================
    // 2. SORT AND DISPLAY OPTION PAIRS
    // =================================================================
    info!("📊 2. OPTION PAIRS ANALYSIS");
    info!("---------------------------");

    // Sort strikes
    let mut strikes: Vec<u64> = option_pairs.keys().cloned().collect();
    strikes.sort();

    info!("🎯 Found {} unique strikes", strikes.len());
    println!();

    // Display header
    println!(
        "┌──────────┬─────────────────────────────┬─────────────────────────────┬──────────────┐"
    );
    println!(
        "│  STRIKE  │            CALL             │            PUT              │   SPREADS    │"
    );
    println!(
        "├──────────┼─────────────────────────────┼─────────────────────────────┼──────────────┤"
    );
    println!("│          │  Bid   │  Ask   │   IV    │  Bid   │  Ask   │   IV    │ Call │  Put  │");
    println!("├──────────┼────────┼────────┼─────────┼────────┼────────┼─────────┼──────┼───────┤");

    for strike in &strikes {
        if let Some(pair) = option_pairs.get(strike) {
            let call_bid = pair
                .call
                .as_ref()
                .and_then(|c| c.ticker.best_bid_price)
                .unwrap_or(0.0);
            let call_ask = pair
                .call
                .as_ref()
                .and_then(|c| c.ticker.best_ask_price)
                .unwrap_or(0.0);
            let call_iv = pair
                .call
                .as_ref()
                .and_then(|c| c.ticker.mark_iv)
                .unwrap_or(0.0);

            let put_bid = pair
                .put
                .as_ref()
                .and_then(|p| p.ticker.best_bid_price)
                .unwrap_or(0.0);
            let put_ask = pair
                .put
                .as_ref()
                .and_then(|p| p.ticker.best_ask_price)
                .unwrap_or(0.0);
            let put_iv = pair
                .put
                .as_ref()
                .and_then(|p| p.ticker.mark_iv)
                .unwrap_or(0.0);

            let call_spread = if call_ask > call_bid && call_bid > 0.0 {
                call_ask - call_bid
            } else {
                0.0
            };
            let put_spread = if put_ask > put_bid && put_bid > 0.0 {
                put_ask - put_bid
            } else {
                0.0
            };

            println!(
                "│ {:8} │ {:6.4} │ {:6.4} │ {:6.1}% │ {:6.4} │ {:6.4} │ {:6.1}% │ {:4.4} │ {:5.4} │",
                strike,
                call_bid,
                call_ask,
                call_iv,
                put_bid,
                put_ask,
                put_iv,
                call_spread,
                put_spread
            );
        }
    }

    println!("└──────────┴────────┴────────┴─────────┴────────┴────────┴─────────┴──────┴───────┘");
    println!();

    // =================================================================
    // 3. GREEKS ANALYSIS
    // =================================================================
    info!("📈 3. GREEKS ANALYSIS");
    info!("--------------------");

    println!("┌──────────┬─────────────────────────────┬─────────────────────────────┐");
    println!("│  STRIKE  │            CALL             │            PUT              │");
    println!("├──────────┼─────────────────────────────┼─────────────────────────────┤");
    println!("│          │ Delta  │ Gamma  │  Vega   │ Delta  │ Gamma  │  Vega   │");
    println!("├──────────┼────────┼────────┼─────────┼────────┼────────┼─────────┤");

    for strike in &strikes {
        if let Some(pair) = option_pairs.get(strike) {
            let call_delta = pair
                .call
                .as_ref()
                .and_then(|c| c.ticker.greeks.as_ref().and_then(|g| g.delta))
                .unwrap_or(0.0);
            let call_gamma = pair
                .call
                .as_ref()
                .and_then(|c| c.ticker.greeks.as_ref().and_then(|g| g.gamma))
                .unwrap_or(0.0);
            let call_vega = pair
                .call
                .as_ref()
                .and_then(|c| c.ticker.greeks.as_ref().and_then(|g| g.vega))
                .unwrap_or(0.0);

            let put_delta = pair
                .put
                .as_ref()
                .and_then(|p| p.ticker.greeks.as_ref().and_then(|g| g.delta))
                .unwrap_or(0.0);
            let put_gamma = pair
                .put
                .as_ref()
                .and_then(|p| p.ticker.greeks.as_ref().and_then(|g| g.gamma))
                .unwrap_or(0.0);
            let put_vega = pair
                .put
                .as_ref()
                .and_then(|p| p.ticker.greeks.as_ref().and_then(|g| g.vega))
                .unwrap_or(0.0);

            println!(
                "│ {:8} │ {:6.3} │ {:6.4} │ {:7.3} │ {:6.3} │ {:6.4} │ {:7.3} │",
                strike, call_delta, call_gamma, call_vega, put_delta, put_gamma, put_vega
            );
        }
    }

    println!("└──────────┴────────┴────────┴─────────┴────────┴────────┴─────────┘");
    println!();

    // =================================================================
    // 4. VOLUME AND OPEN INTEREST ANALYSIS
    // =================================================================
    info!("📊 4. VOLUME & OPEN INTEREST");
    info!("----------------------------");

    let mut total_call_volume = 0.0;
    let mut total_put_volume = 0.0;
    let mut total_call_oi = 0.0;
    let mut total_put_oi = 0.0;

    println!("┌──────────┬─────────────────────────────┬─────────────────────────────┐");
    println!("│  STRIKE  │            CALL             │            PUT              │");
    println!("├──────────┼─────────────────────────────┼─────────────────────────────┤");
    println!("│          │ Volume │    OI    │ Last  │ Volume │    OI    │ Last  │");
    println!("├──────────┼────────┼──────────┼───────┼────────┼──────────┼───────┤");

    for strike in &strikes {
        if let Some(pair) = option_pairs.get(strike) {
            let call_volume = pair
                .call
                .as_ref()
                .map(|c| c.ticker.stats.volume)
                .unwrap_or(0.0);
            let call_oi = pair
                .call
                .as_ref()
                .and_then(|c| c.ticker.open_interest)
                .unwrap_or(0.0);
            let call_last = pair
                .call
                .as_ref()
                .and_then(|c| c.ticker.last_price)
                .unwrap_or(0.0);

            let put_volume = pair
                .put
                .as_ref()
                .map(|p| p.ticker.stats.volume)
                .unwrap_or(0.0);
            let put_oi = pair
                .put
                .as_ref()
                .and_then(|p| p.ticker.open_interest)
                .unwrap_or(0.0);
            let put_last = pair
                .put
                .as_ref()
                .and_then(|p| p.ticker.last_price)
                .unwrap_or(0.0);

            total_call_volume += call_volume;
            total_put_volume += put_volume;
            total_call_oi += call_oi;
            total_put_oi += put_oi;

            println!(
                "│ {:8} │ {:6.1} │ {:8.1} │ {:5.4} │ {:6.1} │ {:8.1} │ {:5.4} │",
                strike, call_volume, call_oi, call_last, put_volume, put_oi, put_last
            );
        }
    }

    println!("├──────────┼────────┼──────────┼───────┼────────┼──────────┼───────┤");
    println!(
        "│  TOTAL   │ {:6.1} │ {:8.1} │   -   │ {:6.1} │ {:8.1} │   -   │",
        total_call_volume, total_call_oi, total_put_volume, total_put_oi
    );
    println!("└──────────┴────────┴──────────┴───────┴────────┴──────────┴───────┘");
    println!();

    // =================================================================
    // 5. SUMMARY STATISTICS
    // =================================================================
    info!("📈 5. SUMMARY STATISTICS");
    info!("-----------------------");

    let put_call_volume_ratio = if total_call_volume > 0.0 {
        total_put_volume / total_call_volume
    } else {
        0.0
    };

    let put_call_oi_ratio = if total_call_oi > 0.0 {
        total_put_oi / total_call_oi
    } else {
        0.0
    };

    info!("🎯 Total Strikes: {}", strikes.len());
    info!("📞 Total Call Volume: {:.1}", total_call_volume);
    info!("📉 Total Put Volume: {:.1}", total_put_volume);
    info!("📊 Put/Call Volume Ratio: {:.2}", put_call_volume_ratio);
    info!("📞 Total Call Open Interest: {:.1}", total_call_oi);
    info!("📉 Total Put Open Interest: {:.1}", total_put_oi);
    info!("📊 Put/Call OI Ratio: {:.2}", put_call_oi_ratio);
    info!("📅 Expiry Date: {} (2025-09-10)", target_expiry);

    // Get underlying price from any available ticker
    if let Some(pair) = option_pairs.values().next()
        && let Some(underlying_price) = pair
            .call
            .as_ref()
            .and_then(|c| c.ticker.underlying_price)
            .or_else(|| pair.put.as_ref().and_then(|p| p.ticker.underlying_price))
    {
        info!("💰 Underlying BTC Price: ${:.2}", underlying_price);
    }

    println!();
    info!("🎉 BTC Option Pair analysis completed successfully!");
    info!("💡 Tip: Use different expiry dates by modifying the target_expiry variable");
    info!("📊 This data shows call/put pairs grouped by strike, useful for spread strategies");

    Ok(())
}
