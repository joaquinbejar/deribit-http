//! Funding Rate Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public endpoints:
//! - `/public/get_funding_rate_history` - Historical funding rates
//! - `/public/get_funding_rate_value` - Funding rate value for period
//!
//! Usage: cargo run --bin funding_rate_endpoints

use deribit_http::DeribitHttpClient;
use std::env;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    info!("🚀 Deribit HTTP Client - Funding Rate Endpoints Example");
    info!("========================================================");
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

    // Define time range for testing (last 24 hours from a known historical point)
    let end_timestamp = 1569974400000u64; // Known historical timestamp
    let start_timestamp = end_timestamp - (24 * 60 * 60 * 1000); // 24 hours before

    info!("⏰ Time range for testing:");
    info!(
        "   Start: {} ({})",
        start_timestamp, "24 hours ago from reference point"
    );
    info!("   End: {} ({})", end_timestamp, "reference point");
    println!();

    // =================================================================
    // 1. GET FUNDING RATE HISTORY (/public/get_funding_rate_history)
    // =================================================================
    info!("📈 1. GET FUNDING RATE HISTORY");
    info!("------------------------------");

    // Test with BTC-PERPETUAL
    match client
        .get_funding_rate_history("BTC-PERPETUAL", start_timestamp, end_timestamp)
        .await
    {
        Ok(funding_history) => {
            info!("✅ Funding rate history for BTC-PERPETUAL retrieved successfully");
            info!("📊 Found {} funding rate records:", funding_history.len());

            for (i, rate_data) in funding_history.iter().take(5).enumerate() {
                info!(
                    "   {}. Timestamp: {} - Interest 8h: {:.8} - Interest 1h: {:.8}",
                    i + 1,
                    rate_data.timestamp,
                    rate_data.interest_8h,
                    rate_data.interest_1h
                );
                info!(
                    "      Index Price: ${:.2} | Prev Index Price: ${:.2}",
                    rate_data.index_price, rate_data.prev_index_price
                );
            }

            if funding_history.len() > 5 {
                info!(
                    "💡 Showing first 5 of {} funding rate records",
                    funding_history.len()
                );
            }

            if funding_history.is_empty() {
                info!("💡 No funding rate history data available for the specified time range");
            }
        }
        Err(e) => {
            error!("❌ Get funding rate history for BTC-PERPETUAL error: {}", e);
        }
    }

    // Also test with ETH-PERPETUAL
    match client
        .get_funding_rate_history("ETH-PERPETUAL", start_timestamp, end_timestamp)
        .await
    {
        Ok(funding_history) => {
            info!("✅ Funding rate history for ETH-PERPETUAL retrieved successfully");
            info!(
                "📊 Found {} ETH funding rate records",
                funding_history.len()
            );

            for (i, rate_data) in funding_history.iter().take(3).enumerate() {
                info!(
                    "   ETH {}. Interest 8h: {:.8} at {}",
                    i + 1,
                    rate_data.interest_8h,
                    rate_data.timestamp
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get funding rate history for ETH-PERPETUAL error: {}", e);
            info!("💡 This may be expected if ETH perpetual data is limited for this time range");
        }
    }
    println!();

    // =================================================================
    // 2. GET FUNDING RATE VALUE (/public/get_funding_rate_value)
    // =================================================================
    info!("💰 2. GET FUNDING RATE VALUE");
    info!("----------------------------");

    // Test with BTC-PERPETUAL for the same time period
    match client
        .get_funding_rate_value("BTC-PERPETUAL", start_timestamp, end_timestamp)
        .await
    {
        Ok(funding_rate_value) => {
            info!("✅ Funding rate value for BTC-PERPETUAL retrieved successfully");
            info!(
                "📈 Funding rate value for period: {:.10}",
                funding_rate_value
            );
            info!("💡 This represents the cumulative funding rate for the entire period");

            // Provide some context about the value
            if funding_rate_value > 0.0 {
                info!("📊 Positive funding rate indicates longs pay shorts");
            } else if funding_rate_value < 0.0 {
                info!("📊 Negative funding rate indicates shorts pay longs");
            } else {
                info!("📊 Zero funding rate indicates balanced positions");
            }
        }
        Err(e) => {
            error!("❌ Get funding rate value for BTC-PERPETUAL error: {}", e);
        }
    }

    // Test with a shorter time period (last 8 hours from the reference point)
    let short_start_timestamp = end_timestamp - (8 * 60 * 60 * 1000); // 8 hours before

    match client
        .get_funding_rate_value("BTC-PERPETUAL", short_start_timestamp, end_timestamp)
        .await
    {
        Ok(funding_rate_value) => {
            info!("✅ Funding rate value for BTC-PERPETUAL (8h period) retrieved successfully");
            info!("📈 8-hour funding rate value: {:.10}", funding_rate_value);
            info!("💡 This is typically closer to the standard 8-hour funding cycle");
        }
        Err(e) => {
            warn!(
                "⚠️ Get funding rate value for BTC-PERPETUAL (8h) error: {}",
                e
            );
            info!("💡 This may be expected if data is not available for this specific range");
        }
    }

    // Also test with ETH-PERPETUAL
    match client
        .get_funding_rate_value("ETH-PERPETUAL", start_timestamp, end_timestamp)
        .await
    {
        Ok(funding_rate_value) => {
            info!("✅ Funding rate value for ETH-PERPETUAL retrieved successfully");
            info!(
                "📈 ETH funding rate value for period: {:.10}",
                funding_rate_value
            );
        }
        Err(e) => {
            warn!("⚠️ Get funding rate value for ETH-PERPETUAL error: {}", e);
            info!("💡 This may be expected if ETH perpetual data is limited for this period");
        }
    }
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED ENDPOINTS");
    info!("==============================");
    info!(
        "📈 /public/get_funding_rate_history - Historical funding rates for PERPETUAL instruments"
    );
    info!("💰 /public/get_funding_rate_value - Funding rate value for specific time periods");
    println!();

    info!("🎉 Funding rate endpoints example completed successfully!");
    info!(
        "💡 Tip: Both endpoints are essential for understanding perpetual contract funding mechanisms"
    );
    info!(
        "🔗 Use funding rate history for analysis and funding rate value for period calculations"
    );
    info!(
        "⚠️ Note: Both endpoints work only with PERPETUAL instruments (BTC-PERPETUAL, ETH-PERPETUAL, etc.)"
    );

    Ok(())
}
