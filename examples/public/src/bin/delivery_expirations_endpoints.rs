//! Delivery Prices and Expirations Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public endpoints:
//! - `/public/get_delivery_prices` - Historical delivery prices
//! - `/public/get_expirations` - Instrument expirations
//!
//! Usage: cargo run --bin delivery_expirations_endpoints

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

    info!("🚀 Deribit HTTP Client - Delivery Prices & Expirations Example");
    info!("================================================================");
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
    // 1. GET DELIVERY PRICES (/public/get_delivery_prices)
    // =================================================================
    info!("📊 1. GET DELIVERY PRICES");
    info!("-------------------------");

    // Test with BTC USD index
    match client.get_delivery_prices("btc_usd", Some(5), Some(0)).await {
        Ok(delivery_prices) => {
            info!("✅ Delivery prices for btc_usd retrieved successfully");
            info!("📊 Found {} delivery price records:", delivery_prices.data.len());
            info!("📈 Total records available: {}", delivery_prices.records_total);
            
            for (i, price_data) in delivery_prices.data.iter().enumerate() {
                info!("   {}. Date: {} - Price: ${:.2}", 
                     i + 1,
                     price_data.date,
                     price_data.delivery_price);
            }
            
            if delivery_prices.data.is_empty() {
                info!("💡 No delivery price data available for btc_usd");
            }
        }
        Err(e) => {
            error!("❌ Get delivery prices for btc_usd error: {}", e);
        }
    }

    // Also test with ETH USD index
    match client.get_delivery_prices("eth_usd", Some(3), Some(0)).await {
        Ok(delivery_prices) => {
            info!("✅ Delivery prices for eth_usd retrieved successfully");
            info!("📊 Found {} ETH delivery price records", delivery_prices.data.len());
            
            for price_data in &delivery_prices.data {
                info!("   ETH delivery: {} - ${:.2}", price_data.date, price_data.delivery_price);
            }
        }
        Err(e) => {
            warn!("⚠️ Get delivery prices for eth_usd error: {}", e);
            info!("💡 This may be expected if ETH delivery data is limited on testnet");
        }
    }
    println!();

    // =================================================================
    // 2. GET EXPIRATIONS (/public/get_expirations)
    // =================================================================
    info!("📅 2. GET EXPIRATIONS");
    info!("--------------------");

    // Test with BTC futures
    match client.get_expirations("BTC", "future", None).await {
        Ok(expirations) => {
            info!("✅ Expirations for BTC futures retrieved successfully");
            
            if let Some(futures) = &expirations.future {
                info!("📊 Found {} BTC future expirations:", futures.len());
                for (i, expiration) in futures.iter().enumerate() {
                    info!("   {}. {}", i + 1, expiration);
                }
            } else {
                info!("💡 No BTC future expirations found");
            }
            
            if let Some(options) = &expirations.option {
                info!("📊 Also found {} BTC option expirations", options.len());
            }
        }
        Err(e) => {
            error!("❌ Get BTC future expirations error: {}", e);
        }
    }

    // Test with ETH options
    match client.get_expirations("ETH", "option", None).await {
        Ok(expirations) => {
            info!("✅ Expirations for ETH options retrieved successfully");
            
            if let Some(options) = &expirations.option {
                info!("📊 Found {} ETH option expirations:", options.len());
                for (i, expiration) in options.iter().take(5).enumerate() {
                    info!("   {}. {}", i + 1, expiration);
                }
                
                if options.len() > 5 {
                    info!("💡 Showing first 5 of {} ETH option expirations", options.len());
                }
            } else {
                info!("💡 No ETH option expirations found");
            }
        }
        Err(e) => {
            warn!("⚠️ Get ETH option expirations error: {}", e);
            info!("💡 This may be expected if ETH options are limited on testnet");
        }
    }

    // Test with any currency and any kind to see all available expirations
    match client.get_expirations("any", "any", None).await {
        Ok(expirations) => {
            info!("✅ All expirations retrieved successfully");
            
            let mut total_expirations = 0;
            
            if let Some(futures) = &expirations.future {
                info!("📈 Total future expirations: {}", futures.len());
                total_expirations += futures.len();
            }
            
            if let Some(options) = &expirations.option {
                info!("📊 Total option expirations: {}", options.len());
                total_expirations += options.len();
            }
            
            info!("🎯 Total expirations across all instruments: {}", total_expirations);
        }
        Err(e) => {
            warn!("⚠️ Get all expirations error: {}", e);
        }
    }
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED ENDPOINTS");
    info!("==============================");
    info!("📊 /public/get_delivery_prices - Historical delivery prices");
    info!("📅 /public/get_expirations - Instrument expirations");
    println!();

    info!("🎉 Delivery prices and expirations endpoints example completed successfully!");
    info!("💡 Tip: Both endpoints provide essential information for derivatives trading");
    info!("🔗 Use delivery prices for settlement tracking and expirations for contract lifecycle management");

    Ok(())
}