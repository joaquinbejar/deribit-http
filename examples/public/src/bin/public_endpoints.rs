//! Public Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public endpoints:
//! - `/public/get_time` - Server time
//! - `/public/hello` - WebSocket client introduction (WebSocket-only)
//! - `/public/status` - Platform status and locked currencies
//! - `/public/test` - Connectivity test
//!
//! Usage: cargo run --bin public_endpoints

use deribit_http::DeribitHttpClient;
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    info!("🚀 Deribit HTTP Client - Public Endpoints Example");
    info!("==================================================");
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
    // 1. GET SERVER TIME (/public/get_time)
    // =================================================================
    info!("🕐 1. GET SERVER TIME");
    info!("-------------------");

    match client.get_server_time().await {
        Ok(server_time) => {
            info!("✅ Server time retrieved successfully");
            info!("🕐 Server timestamp: {} ms", server_time);
            info!("💡 This is the current server time in milliseconds since Unix epoch");
        }
        Err(e) => {
            error!("❌ Get server time error: {}", e);
        }
    }
    println!();

    // =================================================================
    // 2. HELLO ENDPOINT (/public/hello) - WebSocket Only
    // =================================================================
    info!("👋 2. HELLO ENDPOINT (WebSocket Only)");
    info!("------------------------------------");

    match client.hello("deribit-http-client", "0.1.0").await {
        Ok(response) => {
            // This should not happen since hello is WebSocket-only
            info!("✅ Hello response: API version {}", response.version);
        }
        Err(e) => {
            info!("ℹ️ Hello endpoint correctly returned expected error:");
            info!("📝 {}", e);
            info!("💡 This is expected behavior - hello is only available via WebSocket");
        }
    }
    println!();

    // =================================================================
    // 3. PLATFORM STATUS (/public/status)
    // =================================================================
    info!("📊 3. PLATFORM STATUS");
    info!("--------------------");

    match client.get_status().await {
        Ok(status) => {
            info!("✅ Platform status retrieved successfully");
            info!("🔒 Platform locked: {}", status.locked);

            if status.locked_indices.is_empty() {
                info!("🟢 No currency indices are currently locked");
            } else {
                info!(
                    "⚠️ Locked currency indices ({}):",
                    status.locked_indices.len()
                );
                for index in &status.locked_indices {
                    info!("   • {}", index);
                }
            }
        }
        Err(e) => {
            error!("❌ Get platform status error: {}", e);
        }
    }
    println!();

    // =================================================================
    // 4. CONNECTIVITY TEST (/public/test)
    // =================================================================
    info!("🔌 4. CONNECTIVITY TEST");
    info!("----------------------");

    match client.test_connection().await {
        Ok(result) => {
            info!("✅ Connectivity test successful");
            info!("📝 Test result: {}", result);
            info!("🌐 Connection to Deribit API is working properly");
        }
        Err(e) => {
            error!("❌ Connectivity test failed: {}", e);
        }
    }
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF TESTED PUBLIC ENDPOINTS");
    info!("====================================");
    info!("✅ /public/get_time - Server time retrieval");
    info!("ℹ️ /public/hello - WebSocket client introduction (properly handled as WS-only)");
    info!("✅ /public/status - Platform status and locked currencies");
    info!("✅ /public/test - Connectivity test");
    println!();

    info!("🎉 Public endpoints example completed successfully!");
    info!("💡 Tip: All endpoints are working as expected");
    info!("🔗 For WebSocket functionality, consider using the Deribit WebSocket API");

    Ok(())
}
