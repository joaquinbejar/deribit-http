//! Trading Endpoints Example
//!
//! This example demonstrates the correct functioning of the following private endpoints:
//! - `/private/buy` - Place buy order
//! - `/private/sell` - Place sell order  
//! - `/private/edit` - Modify order
//! - `/private/edit_by_label` - Edit order by label
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true" (optional, defaults to true)
//!
//! Then run: cargo run --bin trading_endpoints

use deribit_http::{
    BuyOrderRequest, DeribitHttpClient, EditOrderRequest, HttpError, OrderType, SellOrderRequest,
    TimeInForce,
};
use std::env;
use std::path::Path;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("debug").init();

    // Check if .env file exists
    if !Path::new("../../.env").exists() {
        return Err(HttpError::ConfigError(
            "Missing .env file. Please create one with DERIBIT_CLIENT_ID and DERIBIT_CLIENT_SECRET"
                .to_string(),
        ));
    }

    // Load environment variables
    dotenv::dotenv().ok();

    info!("🚀 Deribit HTTP Client - Trading Endpoints Example");
    info!("==================================================");
    println!();

    // Get authentication credentials from environment
    // Check environment variables
    let client_id = env::var("DERIBIT_CLIENT_ID").map_err(|_| {
        HttpError::ConfigError("DERIBIT_CLIENT_ID not found in environment variables".to_string())
    })?;
    let client_secret = env::var("DERIBIT_CLIENT_SECRET").map_err(|_| {
        HttpError::ConfigError(
            "DERIBIT_CLIENT_SECRET not found in environment variables".to_string(),
        )
    })?;

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
    // AUTHENTICATION
    // =================================================================
    info!("🔐 AUTHENTICATING WITH OAUTH2");
    info!("------------------------------");

    match client.authenticate_oauth2(&client_id, &client_secret).await {
        Ok(auth_token) => {
            info!("✅ Authentication successful");
            info!(
                "🎫 Access token expires in: {} seconds",
                auth_token.expires_in
            );
            info!(
                "🔄 Refresh token available: {}",
                auth_token.refresh_token.is_some()
            );
        }
        Err(e) => {
            error!("❌ Authentication failed: {}", e);
            return Err(e.into());
        }
    }
    println!();

    // =================================================================
    // 1. PLACE BUY ORDER (/private/buy)
    // =================================================================
    info!("💰 1. PLACE BUY ORDER");
    info!("---------------------");

    let buy_request = BuyOrderRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
        amount: Some(10.0), // 10 USD worth
        contracts: None,
        order_type: OrderType::Limit,
        price: Some(30000.0), // Low price to avoid execution
        label: Some("example_buy_order".to_string()),
        time_in_force: TimeInForce::GoodTilCancelled,
        post_only: true, // Ensure we don't execute immediately
        reduce_only: false,
    };

    let buy_order_id = match client.buy_order(buy_request).await {
        Ok(order_response) => {
            info!("✅ Buy order placed successfully");
            info!("📋 Order ID: {}", order_response.order.order_id);
            info!("💰 Price: ${:.2}", order_response.order.price);
            info!("📊 Amount: {:.6} BTC", order_response.order.amount);
            info!("🏷️  Label: {:?}", order_response.order.label);
            info!("📊 Status: {}", order_response.order.order_state);
            order_response.order.order_id.clone()
        }
        Err(e) => {
            error!("❌ Failed to place buy order: {}", e);
            return Err(e.into());
        }
    };
    println!();

    // =================================================================
    // 2. PLACE SELL ORDER (/private/sell)
    // =================================================================
    info!("💸 2. PLACE SELL ORDER");
    info!("----------------------");

    let sell_request = SellOrderRequest {
        instrument_name: "BTC-PERPETUAL".to_string(),
        amount: Some(10.0), // 10 USD worth
        contracts: None,
        order_type: OrderType::Limit,
        price: Some(100000.0), // High price to avoid execution
        label: Some("example_sell_order".to_string()),
        time_in_force: TimeInForce::GoodTilCancelled,
        post_only: true, // Ensure we don't execute immediately
        reduce_only: false,
    };

    let sell_order_id = match client.sell_order(sell_request).await {
        Ok(order_response) => {
            info!("✅ Sell order placed successfully");
            info!("📋 Order ID: {}", order_response.order.order_id);
            info!("💰 Price: ${:.2}", order_response.order.price);
            info!("📊 Amount: {:.6} BTC", order_response.order.amount);
            info!("🏷️  Label: {:?}", order_response.order.label);
            info!("📊 Status: {}", order_response.order.order_state);
            order_response.order.order_id.clone()
        }
        Err(e) => {
            error!("❌ Failed to place sell order: {}", e);
            return Err(e.into());
        }
    };
    println!();

    // =================================================================
    // 3. EDIT ORDER BY ID (/private/edit)
    // =================================================================
    info!("✏️  3. EDIT ORDER BY ID");
    info!("-----------------------");

    let edit_request = EditOrderRequest {
        order_id: buy_order_id.clone(),
        amount: Some(15.0),   // Change amount from 10 to 15 USD
        price: Some(25000.0), // Change price from 30000 to 25000
        advanced: None,
        post_only: true,
        reduce_only: false,
    };

    match client.edit_order(edit_request).await {
        Ok(order_response) => {
            info!("✅ Order edited successfully");
            info!("📋 Order ID: {}", order_response.order.order_id);
            info!("💰 New Price: ${:.2}", order_response.order.price);
            info!("📊 New Amount: {:.6} BTC", order_response.order.amount);
            info!("📊 Status: {}", order_response.order.order_state);
        }
        Err(e) => {
            warn!("⚠️  Failed to edit order by ID: {}", e);
            info!("ℹ️  This might be expected if the order was already filled or cancelled");
        }
    };
    println!();

    // =================================================================
    // 4. EDIT ORDER BY LABEL (/private/edit_by_label)
    // =================================================================
    info!("🏷️  4. EDIT ORDER BY LABEL");
    info!("---------------------------");

    match client
        .edit_by_label("example_sell_order", Some(20.0), Some(95000.0))
        .await
    {
        Ok(order_response) => {
            info!("✅ Order edited by label successfully");
            info!("📋 Order ID: {}", order_response.order.order_id);
            info!("💰 New Price: ${:.2}", order_response.order.price);
            info!("📊 New Amount: {:.6} BTC", order_response.order.amount);
            info!("🏷️  Label: {:?}", order_response.order.label);
            info!("📊 Status: {}", order_response.order.order_state);
        }
        Err(e) => {
            warn!("⚠️  Failed to edit order by label: {}", e);
            info!("ℹ️  This might be expected if the order was already filled or cancelled");
        }
    };
    println!();

    // =================================================================
    // CLEANUP (Optional - cancel remaining orders)
    // =================================================================
    info!("🧹 5. CLEANUP - CANCEL ORDERS");
    info!("------------------------------");

    // Try to cancel the buy order
    match client.cancel_order(&buy_order_id).await {
        Ok(order_info) => {
            info!("✅ Buy order cancelled: {}", order_info.order_id);
            info!("📊 Final status: {}", order_info.order_state);
        }
        Err(e) => {
            warn!("⚠️  Could not cancel buy order: {}", e);
        }
    }

    // Try to cancel the sell order
    match client.cancel_order(&sell_order_id).await {
        Ok(order_info) => {
            info!("✅ Sell order cancelled: {}", order_info.order_id);
            info!("📊 Final status: {}", order_info.order_state);
        }
        Err(e) => {
            warn!("⚠️  Could not cancel sell order: {}", e);
        }
    }

    println!();
    info!("🎉 Trading endpoints example completed successfully!");
    info!("===================================================");

    Ok(())
}
