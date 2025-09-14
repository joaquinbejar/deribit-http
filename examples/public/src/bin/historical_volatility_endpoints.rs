//! Historical Volatility Endpoint Example
//!
//! This example demonstrates the correct functioning of the following public endpoint:
//! - `/public/get_historical_volatility` - Historical volatility
//!
//! Usage: cargo run --bin historical_volatility_endpoints

use deribit_base::prelude::setup_logger;
use deribit_http::DeribitHttpClient;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_logger();

    info!("🚀 Deribit HTTP Client - Historical Volatility Endpoint Example");
    info!("==============================================================");
    println!();

    // Create HTTP client
    let client = DeribitHttpClient::new();

    // =================================================================
    // 1. GET HISTORICAL VOLATILITY (/public/get_historical_volatility)
    // =================================================================
    info!("📊 1. GET HISTORICAL VOLATILITY");
    info!("-------------------------------");

    // Test with BTC
    match client.get_historical_volatility("BTC").await {
        Ok(volatility_data) => {
            info!("✅ Historical volatility for BTC retrieved successfully");
            info!("📊 Found {} volatility data points:", volatility_data.len());

            // Display first few data points
            for (i, data_point) in volatility_data.iter().take(5).enumerate() {
                let timestamp = data_point[0] as u64;
                let volatility = data_point[1];
                info!(
                    "   {}. Timestamp: {} - Volatility: {:.4}%",
                    i + 1,
                    timestamp,
                    volatility
                );
            }

            if volatility_data.len() > 5 {
                info!(
                    "💡 Showing first 5 of {} volatility data points",
                    volatility_data.len()
                );
            }

            if volatility_data.is_empty() {
                info!("💡 No historical volatility data available for BTC");
            }

            // Calculate and display some basic statistics
            if !volatility_data.is_empty() {
                let volatilities: Vec<f64> = volatility_data.iter().map(|point| point[1]).collect();
                let avg_volatility = volatilities.iter().sum::<f64>() / volatilities.len() as f64;
                let max_volatility = volatilities
                    .iter()
                    .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let min_volatility = volatilities.iter().fold(f64::INFINITY, |a, &b| a.min(b));

                info!("📈 Volatility Statistics:");
                info!("   Average: {:.4}%", avg_volatility);
                info!("   Maximum: {:.4}%", max_volatility);
                info!("   Minimum: {:.4}%", min_volatility);
            }
        }
        Err(e) => {
            error!("❌ Get historical volatility for BTC error: {}", e);
        }
    }
    println!();

    // Test with ETH
    match client.get_historical_volatility("ETH").await {
        Ok(volatility_data) => {
            info!("✅ Historical volatility for ETH retrieved successfully");
            info!(
                "📊 Found {} ETH volatility data points",
                volatility_data.len()
            );

            // Display first few data points for ETH
            for (i, data_point) in volatility_data.iter().take(3).enumerate() {
                let timestamp = data_point[0] as u64;
                let volatility = data_point[1];
                info!(
                    "   ETH {}. Volatility: {:.4}% at {}",
                    i + 1,
                    volatility,
                    timestamp
                );
            }

            // Calculate ETH volatility statistics
            if !volatility_data.is_empty() {
                let volatilities: Vec<f64> = volatility_data.iter().map(|point| point[1]).collect();
                let avg_volatility = volatilities.iter().sum::<f64>() / volatilities.len() as f64;
                info!("📈 ETH Average Volatility: {:.4}%", avg_volatility);
            }
        }
        Err(e) => {
            warn!("⚠️ Get historical volatility for ETH error: {}", e);
            info!("💡 This may be expected if ETH volatility data is limited on testnet");
        }
    }
    println!();

    // Test with USDC (may not have volatility data)
    match client.get_historical_volatility("USDC").await {
        Ok(volatility_data) => {
            info!("✅ Historical volatility for USDC retrieved successfully");
            if volatility_data.is_empty() {
                info!("💡 No volatility data for USDC (expected for stablecoin)");
            } else {
                info!(
                    "📊 Found {} USDC volatility data points",
                    volatility_data.len()
                );
            }
        }
        Err(e) => {
            warn!("⚠️ Get historical volatility for USDC error: {}", e);
            info!("💡 This is expected - USDC is a stablecoin with minimal volatility");
        }
    }
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED ENDPOINT");
    info!("=============================");
    info!("📊 /public/get_historical_volatility - Historical volatility data for cryptocurrencies");
    println!();

    info!("🎉 Historical volatility endpoint example completed successfully!");
    info!("💡 Tip: Use this endpoint to analyze price volatility patterns over time");
    info!("🔗 Historical volatility is crucial for options pricing and risk management");
    info!("📈 Data format: [timestamp, volatility_percentage] pairs");

    Ok(())
}
