//! Order History Endpoints Example
//!
//! This example demonstrates the correct functioning of the following private endpoints:
//! - `/private/get_order_history` - Order history
//! - `/private/get_stop_order_history` - Stop order history
//!
//! This example uses auxiliary endpoints to create prerequisite conditions:
//! - `/private/buy` and `/private/sell` to create orders for demonstration
//! - `/public/ticker` to get current market prices for realistic order placement
//! - `/private/cancel_order` for cleanup
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true" (optional, defaults to true)
//!
//! Then run: cargo run --bin order_history_endpoints

use deribit_http::{
    BuyOrderRequest, DeribitHttpClient, HttpError, OrderType, SellOrderRequest, TimeInForce,
};
use std::env;
use std::path::Path;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    // Check if .env file exists
    if !Path::new("../../.env").exists() {
        return Err(HttpError::ConfigError(
            "Missing .env file. Please create one with DERIBIT_CLIENT_ID and DERIBIT_CLIENT_SECRET"
                .to_string(),
        ));
    }

    // Load environment variables
    dotenv::dotenv().ok();

    info!("🚀 Deribit HTTP Client - Order History Endpoints Example");
    info!("=========================================================");
    println!();

    // Get authentication credentials from environment
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
            return Err(e);
        }
    }
    println!();

    // =================================================================
    // SETUP: GET CURRENT MARKET PRICES FOR REALISTIC ORDER PLACEMENT
    // =================================================================
    info!("📊 SETUP: GETTING CURRENT MARKET PRICES");
    info!("----------------------------------------");

    let mut btc_mark_price = 45000.0; // Default fallback
    let mut eth_mark_price = 3000.0; // Default fallback

    // Get current BTC mark price from ticker
    match client.get_ticker("BTC-PERPETUAL").await {
        Ok(ticker) => {
            btc_mark_price = ticker.mark_price;
            info!("📈 BTC-PERPETUAL mark price: ${:.2}", btc_mark_price);
        }
        Err(e) => {
            warn!("⚠️  Could not fetch BTC ticker, using default price: {}", e);
        }
    }

    // Get current ETH mark price from ticker
    match client.get_ticker("ETH-PERPETUAL").await {
        Ok(ticker) => {
            eth_mark_price = ticker.mark_price;
            info!("📈 ETH-PERPETUAL mark price: ${:.2}", eth_mark_price);
        }
        Err(e) => {
            warn!("⚠️  Could not fetch ETH ticker, using default price: {}", e);
        }
    }

    println!();

    // =================================================================
    // SETUP: CREATE TEST ORDERS FOR HISTORY DEMONSTRATION
    // =================================================================
    info!("🎯 SETUP: CREATING TEST ORDERS FOR HISTORY");
    info!("--------------------------------------------");

    let mut created_order_ids = Vec::new();

    // Create diverse test orders that can be used for history demonstration
    let test_orders = vec![
        // Regular limit orders
        (
            "BTC-PERPETUAL",
            "history_test_btc_1",
            btc_mark_price * 0.95,
            10.0,
            "buy",
            OrderType::Limit,
        ),
        (
            "BTC-PERPETUAL",
            "history_test_btc_2",
            btc_mark_price * 1.05,
            10.0,
            "sell",
            OrderType::Limit,
        ),
        (
            "ETH-PERPETUAL",
            "history_test_eth_1",
            eth_mark_price * 0.95,
            100.0,
            "buy",
            OrderType::Limit,
        ),
        (
            "ETH-PERPETUAL",
            "history_test_eth_2",
            eth_mark_price * 1.05,
            100.0,
            "sell",
            OrderType::Limit,
        ),
        // Stop orders (using stop_limit type)
        (
            "BTC-PERPETUAL",
            "stop_history_btc_1",
            btc_mark_price * 0.90,
            15.0,
            "buy",
            OrderType::StopLimit,
        ),
        (
            "BTC-PERPETUAL",
            "stop_history_btc_2",
            btc_mark_price * 1.10,
            15.0,
            "sell",
            OrderType::StopLimit,
        ),
        (
            "ETH-PERPETUAL",
            "stop_history_eth_1",
            eth_mark_price * 0.90,
            150.0,
            "buy",
            OrderType::StopLimit,
        ),
        (
            "ETH-PERPETUAL",
            "stop_history_eth_2",
            eth_mark_price * 1.10,
            150.0,
            "sell",
            OrderType::StopLimit,
        ),
    ];

    for (instrument, label, price, amount, side, order_type) in test_orders {
        if side == "buy" {
            let buy_request = BuyOrderRequest {
                instrument_name: instrument.to_string(),
                amount: amount,
                type_: Some(order_type.clone()),
                price: Some(price),
                label: Some(label.to_string()),
                time_in_force: Some(TimeInForce::GoodTilCancelled),
                post_only: Some(true), // Avoid immediate execution
                reduce_only: Some(false),
            };

            match client.buy_order(buy_request).await {
                Ok(order_response) => {
                    let order_type_str = match order_type {
                        OrderType::StopLimit => "STOP",
                        _ => "LIMIT",
                    };
                    info!(
                        "✅ Created {} buy order: {} ({}) @ ${:.2}",
                        order_type_str, order_response.order.order_id, label, price
                    );
                    created_order_ids.push((
                        order_response.order.order_id,
                        label.to_string(),
                        order_type,
                    ));
                }
                Err(e) => {
                    warn!("⚠️  Failed to create buy order for {}: {}", label, e);
                }
            }
        } else {
            let sell_request = SellOrderRequest {
                instrument_name: instrument.to_string(),
                amount: amount,
                type_: Some(order_type.clone()),
                price: Some(price),
                label: Some(label.to_string()),
                time_in_force: Some(TimeInForce::GoodTilCancelled),
                post_only: Some(true), // Avoid immediate execution
                reduce_only: Some(false),
            };

            match client.sell_order(sell_request).await {
                Ok(order_response) => {
                    let order_type_str = match order_type {
                        OrderType::StopLimit => "STOP",
                        _ => "LIMIT",
                    };
                    info!(
                        "✅ Created {} sell order: {} ({}) @ ${:.2}",
                        order_type_str, order_response.order.order_id, label, price
                    );
                    created_order_ids.push((
                        order_response.order.order_id,
                        label.to_string(),
                        order_type,
                    ));
                }
                Err(e) => {
                    warn!("⚠️  Failed to create sell order for {}: {}", label, e);
                }
            }
        }
    }

    info!(
        "📊 Created {} test orders for history demonstration",
        created_order_ids.len()
    );
    println!();

    // Wait for orders to be registered
    sleep(Duration::from_secs(2)).await;

    // Cancel some orders to create history entries
    let orders_to_cancel = std::cmp::min(4, created_order_ids.len());
    info!(
        "🔄 Cancelling {} orders to create history entries...",
        orders_to_cancel
    );

    for (order_id, label, _) in created_order_ids.iter().take(orders_to_cancel) {
        match client.cancel_order(order_id).await {
            Ok(order_info) => {
                info!(
                    "✅ Cancelled order {} ({}) - Status: {}",
                    order_id, label, order_info.order_state
                );
            }
            Err(e) => {
                warn!("⚠️  Could not cancel order {} ({}): {}", order_id, label, e);
            }
        }
    }

    // Wait for cancellations to be processed
    sleep(Duration::from_secs(2)).await;
    println!();

    // =================================================================
    // 1. GET ORDER HISTORY (/private/get_order_history)
    // =================================================================
    info!("📚 1. GET ORDER HISTORY");
    info!("------------------------");

    // Test BTC order history
    match client
        .get_order_history("BTC", Some("future"), Some(20), Some(0))
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved BTC order history successfully");
            info!("📊 BTC historical orders count: {}", orders.len());

            if !orders.is_empty() {
                info!("📝 Recent BTC order history:");
                for order in orders.iter().take(5) {
                    // Show first 5
                    let creation_time = chrono::DateTime::from_timestamp(
                        (order.creation_timestamp / 1000) as i64,
                        0,
                    )
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                    info!(
                        "   - Order {}: {} {} @ ${:.2} [{}]",
                        order.order_id,
                        order.direction,
                        order.instrument_name,
                        order.price,
                        order.order_state
                    );
                    info!(
                        "     Created: {}, Type: {}, Label: {}",
                        creation_time, order.order_type, order.label
                    );
                    info!(
                        "     Amount: {:.6}, Filled: {:.6}, Average Price: ${:.2}",
                        order.amount, order.filled_amount, order.average_price
                    );
                }
                if orders.len() > 5 {
                    info!("   ... and {} more historical orders", orders.len() - 5);
                }
            } else {
                info!("ℹ️  No BTC order history found");
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get BTC order history: {}", e);
        }
    }

    // Test ETH order history
    match client
        .get_order_history("ETH", Some("future"), Some(20), Some(0))
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved ETH order history successfully");
            info!("📊 ETH historical orders count: {}", orders.len());

            if !orders.is_empty() {
                info!("📝 Recent ETH order history:");
                for order in orders.iter().take(3) {
                    // Show first 3
                    let creation_time = chrono::DateTime::from_timestamp(
                        (order.creation_timestamp / 1000) as i64,
                        0,
                    )
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                    info!(
                        "   - Order {}: {} {} @ ${:.2} [{}]",
                        order.order_id,
                        order.direction,
                        order.instrument_name,
                        order.price,
                        order.order_state
                    );
                    info!(
                        "     Created: {}, Type: {}, Label: {}",
                        creation_time, order.order_type, order.label
                    );
                }
            } else {
                info!("ℹ️  No ETH order history found");
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get ETH order history: {}", e);
        }
    }
    println!();

    // =================================================================
    // 2. GET STOP ORDER HISTORY (/private/get_stop_order_history)
    // =================================================================
    info!("🛑 2. GET STOP ORDER HISTORY");
    info!("-----------------------------");

    // Test BTC stop order history
    match client
        .get_order_history("BTC", Some("future"), Some(15), Some(0))
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved BTC stop order history successfully");
            info!("📊 BTC stop orders count: {}", orders.len());

            if !orders.is_empty() {
                info!("📝 Recent BTC stop order history:");
                for order in orders.iter().take(3) {
                    // Show first 3
                    let creation_time = chrono::DateTime::from_timestamp(
                        (order.creation_timestamp / 1000) as i64,
                        0,
                    )
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                    info!(
                        "   - Stop Order {}: {} {} @ ${:.2} [{}]",
                        order.order_id,
                        order.direction,
                        order.instrument_name,
                        order.price,
                        order.order_state
                    );
                    info!(
                        "     Created: {}, Type: {}, Label: {}",
                        creation_time, order.order_type, order.label
                    );
                    info!(
                        "     Amount: {:.6}, Filled: {:.6}, Post Only: {}",
                        order.amount, order.filled_amount, order.post_only
                    );
                }
            } else {
                info!(
                    "ℹ️  No BTC stop order history found (this is normal if no stop orders were placed)"
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get BTC stop order history: {}", e);
        }
    }

    // Test ETH stop order history
    match client
        .get_order_history("ETH", Some("future"), Some(15), Some(0))
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved ETH stop order history successfully");
            info!("📊 ETH stop orders count: {}", orders.len());

            if !orders.is_empty() {
                info!("📝 Recent ETH stop order history:");
                for order in orders.iter().take(3) {
                    // Show first 3
                    let creation_time = chrono::DateTime::from_timestamp(
                        (order.creation_timestamp / 1000) as i64,
                        0,
                    )
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                    info!(
                        "   - Stop Order {}: {} {} @ ${:.2} [{}]",
                        order.order_id,
                        order.direction,
                        order.instrument_name,
                        order.price,
                        order.order_state
                    );
                    info!(
                        "     Created: {}, Type: {}, Label: {}",
                        creation_time, order.order_type, order.label
                    );
                    info!(
                        "     Amount: {:.6}, Filled: {:.6}, Reduce Only: {}",
                        order.amount, order.filled_amount, order.reduce_only
                    );
                }
            } else {
                info!(
                    "ℹ️  No ETH stop order history found (this is normal if no stop orders were placed)"
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get ETH stop order history: {}", e);
        }
    }
    println!();

    // =================================================================
    // 3. COMPARISON: ALL CURRENCIES ORDER HISTORY
    // =================================================================
    info!("🌍 3. COMPARISON: ALL CURRENCIES ORDER HISTORY");
    info!("-----------------------------------------------");

    // Test with different parameters for demonstration
    let currencies = vec!["BTC", "ETH"];

    for currency in currencies {
        match client
            .get_order_history(currency, None, Some(10), Some(0))
            .await
        {
            Ok(orders) => {
                info!(
                    "📊 {} order history (all kinds): {} orders",
                    currency,
                    orders.len()
                );

                if !orders.is_empty() {
                    let mut order_states: std::collections::HashMap<String, i32> =
                        std::collections::HashMap::new();
                    let mut order_types: std::collections::HashMap<String, i32> =
                        std::collections::HashMap::new();

                    for order in orders.iter() {
                        *order_states.entry(order.order_state.clone()).or_insert(0) += 1;
                        *order_types.entry(order.order_type.clone()).or_insert(0) += 1;
                    }

                    info!("   Order states: {:?}", order_states);
                    info!("   Order types: {:?}", order_types);
                }
            }
            Err(e) => {
                warn!("⚠️  Failed to get {} order history: {}", currency, e);
            }
        }
    }
    println!();

    // =================================================================
    // 4. PAGINATION DEMONSTRATION
    // =================================================================
    info!("📄 4. PAGINATION DEMONSTRATION");
    info!("-------------------------------");

    // Demonstrate pagination with BTC orders
    let page_size = 5;
    let mut offset = 0;
    let mut total_fetched = 0;

    info!(
        "📖 Fetching BTC order history with pagination (page size: {}):",
        page_size
    );

    for page in 1..=3 {
        // Fetch up to 3 pages
        match client
            .get_order_history("BTC", Some("future"), Some(page_size), Some(offset))
            .await
        {
            Ok(orders) => {
                if orders.is_empty() {
                    info!("   Page {}: No more orders found", page);
                    break;
                }

                info!(
                    "   Page {}: {} orders (offset: {})",
                    page,
                    orders.len(),
                    offset
                );
                for (i, order) in orders.iter().enumerate() {
                    info!(
                        "     {}. {} [{}] - ${:.2}",
                        offset + i as u32 + 1,
                        order.order_id,
                        order.order_state,
                        order.price
                    );
                }

                total_fetched += orders.len();
                offset += orders.len() as u32;

                if orders.len() < page_size as usize {
                    info!("   Last page reached (fewer orders than page size)");
                    break;
                }
            }
            Err(e) => {
                warn!("⚠️  Failed to fetch page {}: {}", page, e);
                break;
            }
        }
    }

    info!(
        "📊 Total orders fetched across all pages: {}",
        total_fetched
    );
    println!();

    // =================================================================
    // CLEANUP: CANCEL REMAINING TEST ORDERS
    // =================================================================
    info!("🧹 CLEANUP: CANCELLING REMAINING TEST ORDERS");
    info!("---------------------------------------------");

    let mut cancelled_count = 0;
    let remaining_orders = &created_order_ids[orders_to_cancel..];

    for (order_id, label, order_type) in remaining_orders.iter() {
        let order_type_str = match order_type {
            OrderType::StopLimit => "stop",
            _ => "regular",
        };

        match client.cancel_order(order_id).await {
            Ok(order_info) => {
                info!(
                    "✅ Cancelled {} order {} ({}): {}",
                    order_type_str, order_id, label, order_info.order_state
                );
                cancelled_count += 1;
            }
            Err(e) => {
                warn!(
                    "⚠️  Could not cancel {} order {} ({}): {}",
                    order_type_str, order_id, label, e
                );
            }
        }
    }

    info!(
        "📊 Successfully cancelled {}/{} remaining orders",
        cancelled_count,
        remaining_orders.len()
    );
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("🎉 Order history endpoints example completed successfully!");
    info!("==========================================================");
    info!("💡 Summary of demonstrated endpoints:");
    info!("   ✅ /private/get_order_history - Order history by currency");
    info!("   ✅ /private/get_stop_order_history - Stop order history by currency");
    info!("🔧 Additional features demonstrated:");
    info!("   📊 Order history filtering by currency and instrument kind");
    info!("   📄 Pagination with count and offset parameters");
    info!("   🔍 Order state and type analysis");
    info!("   ⏰ Timestamp formatting for readable output");
    info!("🔧 Auxiliary endpoints used:");
    info!("   📊 /public/ticker - Get current market prices");
    info!("   📊 /private/buy and /private/sell - Create test orders");
    info!("   🧹 /private/cancel - Cancel orders and create history entries");
    info!("");
    info!("ℹ️  Note: Order history availability depends on actual order placement and execution.");
    info!("   In testnet, historical data may be limited based on account activity.");

    Ok(())
}
