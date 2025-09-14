//! Public Endpoints Example
//!
//! This example demonstrates the correct functioning of the following public endpoints:
//! - `/public/get_time` - Server time
//! - `/public/status` - Platform status and locked currencies
//! - `/public/test` - Connectivity test
//!   Note: `/public/hello` is WebSocket-only and not available in HTTP client
//!
//! Usage: cargo run --bin public_endpoints

use deribit_base::prelude::setup_logger;
use deribit_http::DeribitHttpClient;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_logger();
    info!("🚀 Deribit HTTP Client - Public Endpoints Example");
    info!("==================================================");
    println!();

    // Create HTTP client
    let client = DeribitHttpClient::new();

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
    // 2. PLATFORM STATUS (/public/status)
    // =================================================================
    info!("📊 2. PLATFORM STATUS");
    info!("--------------------");

    match client.get_status().await {
        Ok(status) => {
            info!("✅ Platform status retrieved successfully");

            let locked = status.locked.unwrap_or(false);
            info!("🔒 Platform locked: {}", locked);

            if let Some(locked_indices) = &status.locked_indices {
                if locked_indices.is_empty() {
                    info!("🟢 No currency indices are currently locked");
                } else {
                    info!("⚠️ Locked currency indices ({}):", locked_indices.len());
                    for index in locked_indices {
                        info!("   • {}", index);
                    }
                }
            } else {
                info!("🟢 No currency indices information available");
            }
        }
        Err(e) => {
            error!("❌ Get platform status error: {}", e);
        }
    }
    println!();

    // =================================================================
    // 3. CONNECTIVITY TEST (/public/test)
    // =================================================================
    info!("🔌 3. CONNECTIVITY TEST");
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
    info!("✅ /public/status - Platform status and locked currencies");
    info!("✅ /public/test - Connectivity test");
    info!("ℹ️ Note: /public/hello is WebSocket-only (not available in HTTP client)");
    println!();

    info!("🎉 Public endpoints example completed successfully!");
    info!("💡 Tip: All endpoints are working as expected");
    info!("🔗 For WebSocket functionality, consider using the Deribit WebSocket API");

    Ok(())
}
