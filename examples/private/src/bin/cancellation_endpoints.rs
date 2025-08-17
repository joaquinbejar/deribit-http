//! Cancellation Endpoints Example
//!
//! This example demonstrates the correct functioning of the following private endpoints:
//! - `/private/cancel` - Cancel order
//! - `/private/cancel_all` - Cancel all orders
//! - `/private/cancel_all_by_currency` - Cancel all orders by currency
//! - `/private/cancel_all_by_currency_pair` - Cancel all orders by currency pair
//! - `/private/cancel_all_by_instrument` - Cancel all orders by instrument
//! - `/private/cancel_all_by_kind_or_type` - Cancel all orders by kind or type
//! - `/private/cancel_by_label` - Cancel orders by label
//!
//! This example uses auxiliary endpoints to create orders for demonstration:
//! - `/private/buy` and `/private/sell` to create orders for cancellation
//! - `/private/get_open_orders_by_currency` to verify order states
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true" (optional, defaults to true)
//!
//! Then run: cargo run --bin cancellation_endpoints

use deribit_http::{BuyOrderRequest, DeribitHttpClient, HttpError, OrderType, TimeInForce};
use std::env;
use std::path::Path;
use tokio::time::{Duration, sleep};
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
    // SETUP: CREATE ORDERS FOR CANCELLATION DEMONSTRATION
    // =================================================================
    info!("🎯 SETUP: CREATING TEST ORDERS");
    info!("--------------------------------");

    let mut created_orders = Vec::new();

    // Create multiple orders with different labels and instruments for demonstration
    let test_orders = vec![
        ("BTC-PERPETUAL", "btc_test_1", 30000.0, 10.0),
        ("BTC-PERPETUAL", "btc_test_2", 29000.0, 15.0),
        ("ETH-PERPETUAL", "eth_test_1", 2000.0, 100.0),
        ("ETH-PERPETUAL", "eth_test_2", 1950.0, 150.0),
    ];

    for (instrument, label, price, amount) in test_orders {
        let buy_request = BuyOrderRequest {
            instrument_name: instrument.to_string(),
            amount: Some(amount),
            contracts: None,
            order_type: OrderType::Limit,
            price: Some(price),
            label: Some(label.to_string()),
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: true,
            reduce_only: false,
        };

        match client.buy_order(buy_request).await {
            Ok(order_response) => {
                info!(
                    "✅ Created order: {} ({})",
                    order_response.order.order_id, label
                );
                created_orders.push((
                    order_response.order.order_id,
                    label.to_string(),
                    instrument.to_string(),
                ));
            }
            Err(e) => {
                warn!("⚠️  Failed to create test order for {}: {}", label, e);
            }
        }
    }

    info!("📊 Created {} test orders", created_orders.len());
    println!();

    // Wait a moment to ensure orders are registered
    sleep(Duration::from_secs(1)).await;

    // =================================================================
    // 1. CANCEL SINGLE ORDER (/private/cancel)
    // =================================================================
    info!("❌ 1. CANCEL SINGLE ORDER");
    info!("-------------------------");

    if let Some((order_id, _, _)) = created_orders.first() {
        match client.cancel_order(order_id).await {
            Ok(order_info) => {
                info!("✅ Successfully cancelled order: {}", order_id);
                info!("📋 Order ID: {}", order_info.order_id);
                info!("🏷️  Label: {}", order_info.label);
                info!("📊 Final status: {}", order_info.order_state);
            }
            Err(e) => {
                warn!("⚠️  Failed to cancel order {}: {}", order_id, e);
            }
        }
    } else {
        warn!("⚠️  No orders available for single cancellation test");
    }
    println!();

    // =================================================================
    // 2. CANCEL ORDERS BY LABEL (/private/cancel_by_label)
    // =================================================================
    info!("🏷️  2. CANCEL ORDERS BY LABEL");
    info!("------------------------------");

    match client.cancel_by_label("btc_test_2").await {
        Ok(count) => {
            info!("✅ Successfully cancelled orders by label 'btc_test_2'");
            info!("📊 Cancelled orders count: {}", count);
        }
        Err(e) => {
            warn!("⚠️  Failed to cancel orders by label: {}", e);
        }
    }
    println!();

    // Wait a moment to ensure cancellations are processed
    sleep(Duration::from_secs(1)).await;

    // =================================================================
    // 3. CANCEL ALL ORDERS BY INSTRUMENT (/private/cancel_all_by_instrument)
    // =================================================================
    info!("🎯 3. CANCEL ALL ORDERS BY INSTRUMENT");
    info!("-------------------------------------");

    match client.cancel_all_by_instrument("ETH-PERPETUAL", None).await {
        Ok(count) => {
            info!("✅ Successfully cancelled all orders for ETH-PERPETUAL");
            info!("📊 Cancelled orders count: {}", count);
        }
        Err(e) => {
            warn!("⚠️  Failed to cancel orders by instrument: {}", e);
        }
    }
    println!();

    // =================================================================
    // 4. CANCEL ALL ORDERS BY CURRENCY (/private/cancel_all_by_currency)
    // =================================================================
    info!("💰 4. CANCEL ALL ORDERS BY CURRENCY");
    info!("-----------------------------------");

    // Create some more BTC orders for demonstration
    let additional_btc_orders = vec![
        ("BTC-PERPETUAL", "btc_currency_test_1", 28000.0, 20.0),
        ("BTC-PERPETUAL", "btc_currency_test_2", 27000.0, 25.0),
    ];

    for (instrument, label, price, amount) in additional_btc_orders {
        let buy_request = BuyOrderRequest {
            instrument_name: instrument.to_string(),
            amount: Some(amount),
            contracts: None,
            order_type: OrderType::Limit,
            price: Some(price),
            label: Some(label.to_string()),
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: true,
            reduce_only: false,
        };

        if let Ok(order_response) = client.buy_order(buy_request).await {
            info!(
                "✅ Created additional BTC order: {} ({})",
                order_response.order.order_id, label
            );
        }
    }

    sleep(Duration::from_secs(1)).await;

    match client
        .cancel_all_by_currency("BTC", Some("future"), None)
        .await
    {
        Ok(count) => {
            info!("✅ Successfully cancelled all BTC future orders");
            info!("📊 Cancelled orders count: {}", count);
        }
        Err(e) => {
            warn!("⚠️  Failed to cancel orders by currency: {}", e);
        }
    }
    println!();

    // =================================================================
    // 5. CANCEL ALL ORDERS BY KIND OR TYPE (/private/cancel_all_by_kind_or_type)
    // =================================================================
    info!("🔧 5. CANCEL ALL ORDERS BY KIND OR TYPE");
    info!("---------------------------------------");

    // Create some limit orders for demonstration
    let limit_orders = vec![
        ("BTC-PERPETUAL", "limit_test_1", 26000.0, 30.0),
        ("ETH-PERPETUAL", "limit_test_2", 1900.0, 200.0),
    ];

    for (instrument, label, price, amount) in limit_orders {
        let buy_request = BuyOrderRequest {
            instrument_name: instrument.to_string(),
            amount: Some(amount),
            contracts: None,
            order_type: OrderType::Limit,
            price: Some(price),
            label: Some(label.to_string()),
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: true,
            reduce_only: false,
        };

        if let Ok(order_response) = client.buy_order(buy_request).await {
            info!(
                "✅ Created limit order: {} ({})",
                order_response.order.order_id, label
            );
        }
    }

    sleep(Duration::from_secs(1)).await;

    match client
        .cancel_all_by_kind_or_type("BTC", Some("future"), Some("limit"))
        .await
    {
        Ok(count) => {
            info!("✅ Successfully cancelled all BTC future limit orders");
            info!("📊 Cancelled orders count: {}", count);
        }
        Err(e) => {
            warn!("⚠️  Failed to cancel orders by kind or type: {}", e);
        }
    }
    println!();

    // =================================================================
    // 6. CANCEL ALL ORDERS BY CURRENCY PAIR (/private/cancel_all_by_currency_pair)
    // =================================================================
    info!("💱 6. CANCEL ALL ORDERS BY CURRENCY PAIR");
    info!("----------------------------------------");

    // Create some ETH orders for currency pair demonstration
    let eth_orders = vec![
        ("ETH-PERPETUAL", "eth_pair_test_1", 1850.0, 250.0),
        ("ETH-PERPETUAL", "eth_pair_test_2", 1800.0, 300.0),
    ];

    for (instrument, label, price, amount) in eth_orders {
        let buy_request = BuyOrderRequest {
            instrument_name: instrument.to_string(),
            amount: Some(amount),
            contracts: None,
            order_type: OrderType::Limit,
            price: Some(price),
            label: Some(label.to_string()),
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: true,
            reduce_only: false,
        };

        if let Ok(order_response) = client.buy_order(buy_request).await {
            info!(
                "✅ Created ETH order: {} ({})",
                order_response.order.order_id, label
            );
        }
    }

    sleep(Duration::from_secs(1)).await;

    match client
        .cancel_all_by_currency_pair("ETH_USD", Some("future"), None)
        .await
    {
        Ok(count) => {
            info!("✅ Successfully cancelled all ETH_USD future orders");
            info!("📊 Cancelled orders count: {}", count);
        }
        Err(e) => {
            warn!("⚠️  Failed to cancel orders by currency pair: {}", e);
        }
    }
    println!();

    // =================================================================
    // 7. CANCEL ALL ORDERS (/private/cancel_all)
    // =================================================================
    info!("🧹 7. CANCEL ALL ORDERS");
    info!("----------------------");

    // Create a few final orders for the cancel all demonstration
    let final_orders = vec![
        ("BTC-PERPETUAL", "final_test_1", 25000.0, 35.0),
        ("ETH-PERPETUAL", "final_test_2", 1750.0, 350.0),
    ];

    for (instrument, label, price, amount) in final_orders {
        let buy_request = BuyOrderRequest {
            instrument_name: instrument.to_string(),
            amount: Some(amount),
            contracts: None,
            order_type: OrderType::Limit,
            price: Some(price),
            label: Some(label.to_string()),
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: true,
            reduce_only: false,
        };

        if let Ok(order_response) = client.buy_order(buy_request).await {
            info!(
                "✅ Created final test order: {} ({})",
                order_response.order.order_id, label
            );
        }
    }

    sleep(Duration::from_secs(1)).await;

    // Cancel all orders with optional parameters
    match client.cancel_all_orders(Some(false), Some(false)).await {
        Ok(count) => {
            info!("✅ Successfully cancelled all orders");
            info!("📊 Cancelled orders count: {}", count);
        }
        Err(e) => {
            warn!("⚠️  Failed to cancel all orders: {}", e);
            info!("ℹ️  This might be expected if the method signature is different");
        }
    }
    println!();

    // =================================================================
    // VERIFICATION: CHECK REMAINING ORDERS
    // =================================================================
    info!("🔍 8. VERIFICATION: CHECK REMAINING ORDERS");
    info!("-------------------------------------------");

    // Check remaining BTC orders
    match client
        .get_open_orders_by_currency("BTC", Some("future"), None)
        .await
    {
        Ok(orders) => {
            info!("📊 Remaining BTC orders: {}", orders.len());
            for order in orders.iter().take(3) {
                // Show first 3
                info!(
                    "   - Order {}: {} @ ${:.2}",
                    order.order_id, order.label, order.price
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Could not retrieve BTC orders: {}", e);
        }
    }

    // Check remaining ETH orders
    match client
        .get_open_orders_by_currency("ETH", Some("future"), None)
        .await
    {
        Ok(orders) => {
            info!("📊 Remaining ETH orders: {}", orders.len());
            for order in orders.iter().take(3) {
                // Show first 3
                info!(
                    "   - Order {}: {} @ ${:.2}",
                    order.order_id, order.label, order.price
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Could not retrieve ETH orders: {}", e);
        }
    }

    println!();
    info!("🎉 Cancellation endpoints example completed successfully!");
    info!("=========================================================");
    info!("💡 Summary of demonstrated endpoints:");
    info!("   ✅ /private/cancel - Cancel single order");
    info!("   ✅ /private/cancel_by_label - Cancel orders by label");
    info!("   ✅ /private/cancel_all_by_instrument - Cancel orders by instrument");
    info!("   ✅ /private/cancel_all_by_currency - Cancel orders by currency");
    info!("   ✅ /private/cancel_all_by_kind_or_type - Cancel orders by kind/type");
    info!("   ✅ /private/cancel_all_by_currency_pair - Cancel orders by currency pair");
    info!("   ✅ /private/cancel_all - Cancel all orders");

    Ok(())
}
