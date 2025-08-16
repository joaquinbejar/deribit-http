//! Basic example of using the Deribit HTTP client

use deribit_http::{
    DeribitHttpClient,
    config::{HttpConfig, load_from_env},
    error::HttpError,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::init();

    info!("Deribit HTTP Client - Basic Example");
    info!("===================================");

    // Load configuration from environment or use defaults
    let config = match load_from_env() {
        Ok(config) => {
            info!("✅ Configuration loaded from environment");
            config
        }
        Err(_) => {
            info!("⚠️  Using default configuration (testnet)");
            HttpConfig::testnet()
        }
    };

    info!("📡 Base URL: {}", config.base_url);
    info!("⏱️  Timeout: {:?}", config.timeout);
    info!("🔄 Max retries: {}", config.max_retries);
    info!("🔐 Has credentials: {}", config.has_credentials());
    info!();

    // Create the HTTP client
    let client = DeribitHttpClient::with_config(config)?;
    info!("✅ HTTP client created successfully");

    // Test basic connectivity (when implemented)
    info!("🔍 Testing connectivity...");
    match test_connectivity(&client).await {
        Ok(_) => info!("✅ Connectivity test passed"),
        Err(e) => info!("❌ Connectivity test failed: {}", e),
    }

    // Example of getting server time (when implemented)
    info!("🕐 Getting server time...");
    match get_server_time(&client).await {
        Ok(time) => info!("✅ Server time: {}", time),
        Err(e) => info!("❌ Failed to get server time: {}", e),
    }

    // Example of getting instruments (when implemented)
    info!("📊 Getting instruments...");
    match get_instruments(&client).await {
        Ok(count) => info!("✅ Found {} instruments", count),
        Err(e) => info!("❌ Failed to get instruments: {}", e),
    }

    info!();
    info!("🎉 Basic example completed!");
    info!();
    info!("Next steps:");
    info!("- Set environment variables for authentication:");
    info!("  export DERIBIT_CLIENT_ID=your_client_id");
    info!("  export DERIBIT_CLIENT_SECRET=your_client_secret");
    info!("- Or use API key authentication:");
    info!("  export DERIBIT_API_KEY=your_api_key");
    info!("  export DERIBIT_API_SECRET=your_api_secret");
    info!("- Set DERIBIT_TESTNET=false for production");

    Ok(())
}

// Placeholder functions for future implementation
async fn test_connectivity(_client: &DeribitHttpClient) -> Result<(), HttpError> {
    // TODO: Implement public/test endpoint
    info!("  📝 Note: Connectivity test not yet implemented");
    Ok(())
}

async fn get_server_time(_client: &DeribitHttpClient) -> Result<String, HttpError> {
    // TODO: Implement public/get_time endpoint
    info!("  📝 Note: Get server time not yet implemented");
    Ok("Not implemented".to_string())
}

async fn get_instruments(_client: &DeribitHttpClient) -> Result<usize, HttpError> {
    // TODO: Implement public/get_instruments endpoint
    info!("  📝 Note: Get instruments not yet implemented");
    Ok(0)
}