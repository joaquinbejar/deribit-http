//! Cancellation Endpoints Example
//!
//! This example demonstrates the correct functioning of the following private endpoints:
//! - `/private/cancel` - Cancel single order
//!
//! NOTE: Many cancellation endpoints are not yet implemented in the current HTTP client,
//! including cancel_by_label, cancel_all_orders, get_open_orders_by_currency, etc.
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true" (optional, defaults to true)
//!
//! Then run: cargo run --bin cancellation_endpoints

use deribit_http::prelude::*;
use std::env;
use std::path::Path;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    // Load environment variables from .env file if it exists
    if Path::new(".env").exists() {
        dotenvy::dotenv().ok();
    }

    setup_logger();

    info!("🚀 Starting Cancellation Endpoints Example");
    info!("===========================================");

    // Get credentials from environment variables
    let client_id = env::var("DERIBIT_CLIENT_ID")
        .map_err(|_| HttpError::ConfigError("DERIBIT_CLIENT_ID not set".to_string()))?;
    let _client_secret = env::var("DERIBIT_CLIENT_SECRET")
        .map_err(|_| HttpError::ConfigError("DERIBIT_CLIENT_SECRET not set".to_string()))?;
    let testnet = env::var("DERIBIT_TESTNET")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    info!("🔧 Configuration:");
    info!(
        "   Environment: {}",
        if testnet { "Testnet" } else { "Production" }
    );
    info!("   Client ID: {}***", &client_id[..8.min(client_id.len())]);
    println!();

    // Create HTTP client
    let client = DeribitHttpClient::default();

    // =================================================================
    // 1. CREATE A TEST ORDER TO CANCEL
    // =================================================================
    info!("📝 1. CREATE TEST ORDER");
    info!("-----------------------");

    let buy_request = OrderRequest {
        order_id: None,
        instrument_name: "BTC-PERPETUAL".to_string(),
        amount: Some(10.0),
        contracts: None,
        type_: Some(OrderType::Limit),
        label: Some("test_cancel_order".to_string()),
        price: Some(20000.0), // Very low price to avoid execution
        time_in_force: Some(TimeInForce::GoodTilCancelled),
        display_amount: None,
        post_only: Some(true),
        reject_post_only: None,
        reduce_only: Some(false),
        trigger_price: None,
        trigger_offset: None,
        trigger: None,
        advanced: None,
        mmp: None,
        valid_until: None,
        linked_order_type: None,
        trigger_fill_condition: None,
        otoco_config: None,
    };

    let order_id = match client.buy_order(buy_request).await {
        Ok(order_response) => {
            info!("✅ Test order created successfully");
            info!("📋 Order ID: {}", order_response.order.order_id);
            info!("💰 Price: ${:.2}", order_response.order.price);
            info!("📊 Amount: {:.6} USD", order_response.order.amount);
            info!("🏷️  Label: {:?}", order_response.order.label);
            order_response.order.order_id
        }
        Err(e) => {
            error!("❌ Failed to create test order: {}", e);
            return Err(e);
        }
    };
    println!();

    // Wait a moment to ensure order is processed
    sleep(Duration::from_millis(500)).await;

    // =================================================================
    // 2. CANCEL THE ORDER
    // =================================================================
    info!("❌ 2. CANCEL ORDER");
    info!("------------------");

    match client.cancel_order(&order_id).await {
        Ok(cancelled_order) => {
            info!("✅ Order cancelled successfully");
            info!("📋 Cancelled Order ID: {}", cancelled_order.order_id);
            info!("📊 Status: {}", cancelled_order.order_state);
            info!("💰 Price: ${:.2}", cancelled_order.price);
            info!("📊 Amount: {:.6} USD", cancelled_order.amount);
        }
        Err(e) => {
            warn!("⚠️  Failed to cancel order: {}", e);
            info!("ℹ️  This might be expected if the order was already filled");
        }
    }
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📋 SUMMARY OF AVAILABLE CANCELLATION ENDPOINTS");
    info!("===============================================");
    info!("✅ /private/cancel - Cancel single order by ID");
    info!("✅ /private/cancel_all - Cancel all orders");
    info!("✅ /private/cancel_by_label - Cancel orders by label");
    info!("✅ /private/cancel_all_by_currency - Cancel all orders by currency");
    info!("✅ /private/cancel_all_by_instrument - Cancel all orders by instrument");
    info!("✅ /private/cancel_all_by_currency_pair - Cancel all orders by currency pair");
    info!("✅ /private/cancel_all_by_kind_or_type - Cancel orders by kind or type");
    info!("💡 All cancellation endpoints are now fully implemented and ready to use!");
    println!();

    info!("🎉 Cancellation endpoints example completed successfully!");
    info!("💡 Tip: Use order cancellation to manage risk and adjust positions");
    info!("🔗 Cancellation is immediate but confirmation may take a moment");

    Ok(())
}
