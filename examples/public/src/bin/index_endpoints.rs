//! Index Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public endpoints:
//! - `/public/get_index` - Current index price
//! - `/public/get_index_price` - Index price by name
//! - `/public/get_index_price_names` - All supported price indexes
//!
//! Usage: cargo run --bin index_endpoints

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

    info!("🚀 Deribit HTTP Client - Index Endpoints Example");
    info!("================================================");
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
    // 1. GET INDEX PRICE NAMES (/public/get_index_price_names)
    // =================================================================
    info!("📋 1. GET ALL SUPPORTED INDEX PRICE NAMES");
    info!("-----------------------------------------");

    let mut available_indexes = Vec::new();
    match client.get_index_price_names().await {
        Ok(index_names) => {
            info!("✅ Index price names retrieved successfully");
            info!("📊 Found {} supported price indexes:", index_names.len());
            
            for (i, index_name) in index_names.iter().take(10).enumerate() {
                info!("   {}. {}", i + 1, index_name);
                available_indexes.push(index_name.clone());
            }
            
            if index_names.len() > 10 {
                info!("💡 Showing first 10 of {} available indexes", index_names.len());
                available_indexes.extend(index_names.iter().skip(10).cloned());
            } else {
                available_indexes = index_names;
            }
            
            if available_indexes.is_empty() {
                info!("💡 No index price names available");
            }
        }
        Err(e) => {
            error!("❌ Get index price names error: {}", e);
        }
    }
    println!();

    // =================================================================
    // 2. GET INDEX BY CURRENCY (/public/get_index)
    // =================================================================
    info!("📈 2. GET CURRENT INDEX PRICE BY CURRENCY");
    info!("-----------------------------------------");

    // Test with BTC
    match client.get_index("BTC").await {
        Ok(index_data) => {
            info!("✅ Current index for BTC retrieved successfully");
            info!("📊 Index data for BTC:");
            
            if let Some(btc_price) = index_data.btc {
                info!("   💰 BTC Index Price: ${:.2}", btc_price);
            }
            
            info!("   📈 Estimated Delivery Price: ${:.2}", index_data.edp);
            
            // Display other currency prices if available
            if let Some(eth_price) = index_data.eth {
                info!("   💰 ETH Index Price: ${:.2}", eth_price);
            }
            
            if let Some(usdc_price) = index_data.usdc {
                info!("   💰 USDC Index Price: ${:.2}", usdc_price);
            }
            
            if let Some(usdt_price) = index_data.usdt {
                info!("   💰 USDT Index Price: ${:.2}", usdt_price);
            }
            
            if let Some(eurr_price) = index_data.eurr {
                info!("   💰 EURR Index Price: ${:.2}", eurr_price);
            }
        }
        Err(e) => {
            error!("❌ Get index for BTC error: {}", e);
        }
    }

    // Test with ETH
    match client.get_index("ETH").await {
        Ok(index_data) => {
            info!("✅ Current index for ETH retrieved successfully");
            info!("📊 ETH Index data:");
            
            if let Some(eth_price) = index_data.eth {
                info!("   💰 ETH Index Price: ${:.2}", eth_price);
            }
            
            info!("   📈 ETH Estimated Delivery Price: ${:.2}", index_data.edp);
        }
        Err(e) => {
            warn!("⚠️ Get index for ETH error: {}", e);
            info!("💡 This may be expected if ETH index is not available");
        }
    }

    // Test with USDC
    match client.get_index("USDC").await {
        Ok(index_data) => {
            info!("✅ Current index for USDC retrieved successfully");
            info!("📊 USDC Index data:");
            
            if let Some(usdc_price) = index_data.usdc {
                info!("   💰 USDC Index Price: ${:.6}", usdc_price);
            }
            
            info!("   📈 USDC Estimated Delivery Price: ${:.6}", index_data.edp);
        }
        Err(e) => {
            warn!("⚠️ Get index for USDC error: {}", e);
            info!("💡 This may be expected for stablecoin indexes");
        }
    }
    println!();

    // =================================================================
    // 3. GET INDEX PRICE BY NAME (/public/get_index_price)
    // =================================================================
    info!("🎯 3. GET INDEX PRICE BY SPECIFIC INDEX NAME");
    info!("--------------------------------------------");

    // Test with btc_usd if available
    if available_indexes.contains(&"btc_usd".to_string()) {
        match client.get_index_price("btc_usd").await {
            Ok(index_price_data) => {
                info!("✅ Index price for btc_usd retrieved successfully");
                info!("📊 BTC/USD Index details:");
                info!("   💰 Index Price: ${:.2}", index_price_data.index_price);
                info!("   📈 Estimated Delivery Price: ${:.2}", index_price_data.estimated_delivery_price);
                
                let price_diff = index_price_data.index_price - index_price_data.estimated_delivery_price;
                info!("   📊 Price Difference: ${:.2}", price_diff);
            }
            Err(e) => {
                error!("❌ Get index price for btc_usd error: {}", e);
            }
        }
    } else {
        info!("⚠️ btc_usd index not found in available indexes, skipping test");
    }

    // Test with eth_usd if available
    if available_indexes.contains(&"eth_usd".to_string()) {
        match client.get_index_price("eth_usd").await {
            Ok(index_price_data) => {
                info!("✅ Index price for eth_usd retrieved successfully");
                info!("📊 ETH/USD Index details:");
                info!("   💰 Index Price: ${:.2}", index_price_data.index_price);
                info!("   📈 Estimated Delivery Price: ${:.2}", index_price_data.estimated_delivery_price);
            }
            Err(e) => {
                warn!("⚠️ Get index price for eth_usd error: {}", e);
                info!("💡 This may be expected if ETH/USD index is not available on testnet");
            }
        }
    } else {
        info!("⚠️ eth_usd index not found in available indexes, skipping test");
    }

    // Test with btc_usdc if available
    if available_indexes.contains(&"btc_usdc".to_string()) {
        match client.get_index_price("btc_usdc").await {
            Ok(index_price_data) => {
                info!("✅ Index price for btc_usdc retrieved successfully");
                info!("📊 BTC/USDC Index details:");
                info!("   💰 Index Price: ${:.2}", index_price_data.index_price);
                info!("   📈 Estimated Delivery Price: ${:.2}", index_price_data.estimated_delivery_price);
            }
            Err(e) => {
                warn!("⚠️ Get index price for btc_usdc error: {}", e);
            }
        }
    } else {
        info!("ℹ️ btc_usdc index not found in available indexes, skipping test");
    }

    // Test with a non-existent index to demonstrate error handling
    match client.get_index_price("non_existent_index").await {
        Ok(index_price_data) => {
            info!("⚠️ Unexpected success for non-existent index: ${:.2}", index_price_data.index_price);
        }
        Err(e) => {
            info!("✅ Expected error for non-existent index: {}", e);
            info!("💡 This demonstrates proper error handling for invalid index names");
        }
    }
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED INDEX ENDPOINTS");
    info!("====================================");
    info!("📋 /public/get_index_price_names - All supported price indexes");
    info!("📈 /public/get_index - Current index price by currency");
    info!("🎯 /public/get_index_price - Index price by specific index name");
    println!();

    info!("🎉 Index endpoints example completed successfully!");
    info!("💡 Tip: Use get_index_price_names() first to discover available indexes");
    info!("📈 Index prices are essential for derivatives pricing and risk management");
    info!("🔗 These endpoints provide real-time market data for trading decisions");

    Ok(())
}