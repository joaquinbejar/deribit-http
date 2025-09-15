//! Order Query Endpoints Example
//!
//! This example demonstrates the correct functioning of the following private endpoints:
//! - `/private/get_open_orders` - Get open orders across all instruments
//! - `/private/get_open_orders_by_currency` - Get open orders by currency
//! - `/private/get_open_orders_by_instrument` - Get open orders by instrument
//! - `/private/get_open_orders_by_label` - Get open orders by label
//! - `/private/get_order_history_by_currency` - Get order history by currency
//! - `/private/get_order_history_by_instrument` - Get order history by instrument
//! - `/private/get_order_state` - Get specific order state
//!
//! This example uses auxiliary endpoints to create prerequisite conditions:
//! - `/private/buy` and `/private/sell` to create orders for demonstration
//! - `/private/cancel_order` for cleanup
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true" (optional, defaults to true)
//!
//! Then run: cargo run --bin order_query_endpoints

use deribit_http::prelude::*;
use tokio::time::{Duration, sleep};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    // Initialize logging
    setup_logger();
    info!("🚀 Deribit HTTP Client - Order Query Endpoints Example");
    info!("======================================================");
    println!();

    // Create HTTP client
    let client = DeribitHttpClient::new();

    // =================================================================
    // SETUP: CREATE TEST ORDERS FOR DEMONSTRATION
    // =================================================================
    info!("🎯 SETUP: CREATING TEST ORDERS");
    info!("--------------------------------");

    let mut created_order_ids = Vec::new();

    // Create diverse test orders with different currencies, instruments, and labels
    let test_orders = vec![
        // BTC orders with different labels
        ("BTC-PERPETUAL", "query_test_btc_1", 30000.0, 10.0, "buy"),
        ("BTC-PERPETUAL", "query_test_btc_2", 29000.0, 15.0, "buy"),
        (
            "BTC-PERPETUAL",
            "query_test_btc_sell",
            80000.0,
            10.0,
            "sell",
        ),
        // ETH orders
        ("ETH-PERPETUAL", "query_test_eth_1", 2000.0, 100.0, "buy"),
        ("ETH-PERPETUAL", "query_test_eth_2", 1950.0, 150.0, "buy"),
        (
            "ETH-PERPETUAL",
            "query_test_eth_sell",
            5000.0,
            100.0,
            "sell",
        ),
    ];

    for (instrument, label, price, amount, side) in test_orders {
        if side == "buy" {
            let buy_request = OrderRequest {
                instrument_name: instrument.to_string(),
                amount,
                type_: Some(OrderType::Limit),
                price: Some(price),
                label: Some(label.to_string()),
                time_in_force: Some(TimeInForce::GoodTilCancelled),
                post_only: Some(true), // Avoid immediate execution
                reduce_only: Some(false),
            };

            match client.buy_order(buy_request).await {
                Ok(order_response) => {
                    info!(
                        "✅ Created buy order: {} ({}) @ ${:.2}",
                        order_response.order.order_id, label, price
                    );
                    created_order_ids.push((order_response.order.order_id, label.to_string()));
                }
                Err(e) => {
                    warn!("⚠️  Failed to create buy order for {}: {}", label, e);
                }
            }
        } else {
            let sell_request = OrderRequest {
                instrument_name: instrument.to_string(),
                amount,
                type_: Some(OrderType::Limit),
                price: Some(price),
                label: Some(label.to_string()),
                time_in_force: Some(TimeInForce::GoodTilCancelled),
                post_only: Some(true), // Avoid immediate execution
                reduce_only: Some(false),
            };

            match client.sell_order(sell_request).await {
                Ok(order_response) => {
                    info!(
                        "✅ Created sell order: {} ({}) @ ${:.2}",
                        order_response.order.order_id, label, price
                    );
                    created_order_ids.push((order_response.order.order_id, label.to_string()));
                }
                Err(e) => {
                    warn!("⚠️  Failed to create sell order for {}: {}", label, e);
                }
            }
        }
    }

    info!(
        "📊 Created {} test orders for demonstration",
        created_order_ids.len()
    );
    println!();

    // Wait for orders to be registered
    sleep(Duration::from_secs(2)).await;

    // =================================================================
    // 1. GET OPEN ORDERS (/private/get_open_orders)
    // =================================================================
    info!("📋 1. GET OPEN ORDERS (ALL)");
    info!("----------------------------");

    match client.get_open_orders(Some("future"), None).await {
        Ok(orders) => {
            info!("✅ Retrieved all open orders successfully");
            info!("📊 Total open orders: {}", orders.len());

            if !orders.is_empty() {
                info!("📝 Sample orders:");
                for order in orders.iter().take(5) {
                    // Show first 5
                    info!(
                        "   - {}: {} {} @ ${:.2} ({})",
                        order.order_id,
                        order.direction,
                        order.instrument_name,
                        order.price,
                        order.label
                    );
                }
                if orders.len() > 5 {
                    info!("   ... and {} more orders", orders.len() - 5);
                }
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get all open orders: {}", e);
        }
    }
    println!();

    // =================================================================
    // 2. GET OPEN ORDERS BY CURRENCY (/private/get_open_orders_by_currency)
    // =================================================================
    info!("💰 2. GET OPEN ORDERS BY CURRENCY");
    info!("----------------------------------");

    // Test BTC currency
    match client
        .get_open_orders_by_currency("BTC", Some("future"), None)
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved BTC open orders successfully");
            info!("📊 BTC open orders: {}", orders.len());

            for order in orders.iter().take(3) {
                // Show first 3
                info!(
                    "   - BTC: {} {} @ ${:.2} ({})",
                    order.direction, order.instrument_name, order.price, order.label
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get BTC open orders: {}", e);
        }
    }

    // Test ETH currency
    match client
        .get_open_orders_by_currency("ETH", Some("future"), None)
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved ETH open orders successfully");
            info!("📊 ETH open orders: {}", orders.len());

            for order in orders.iter().take(3) {
                // Show first 3
                info!(
                    "   - ETH: {} {} @ ${:.2} ({})",
                    order.direction, order.instrument_name, order.price, order.label
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get ETH open orders: {}", e);
        }
    }
    println!();

    // =================================================================
    // 3. GET OPEN ORDERS BY INSTRUMENT (/private/get_open_orders_by_instrument)
    // =================================================================
    info!("🎯 3. GET OPEN ORDERS BY INSTRUMENT");
    info!("------------------------------------");

    // Test BTC-PERPETUAL
    match client
        .get_open_orders_by_instrument("BTC-PERPETUAL", None)
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved BTC-PERPETUAL open orders successfully");
            info!("📊 BTC-PERPETUAL open orders: {}", orders.len());

            for order in orders.iter().take(3) {
                // Show first 3
                info!(
                    "   - {}: {} @ ${:.2} ({})",
                    order.order_id, order.direction, order.price, order.label
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get BTC-PERPETUAL orders: {}", e);
        }
    }

    // Test ETH-PERPETUAL
    match client
        .get_open_orders_by_instrument("ETH-PERPETUAL", None)
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved ETH-PERPETUAL open orders successfully");
            info!("📊 ETH-PERPETUAL open orders: {}", orders.len());

            for order in orders.iter().take(3) {
                // Show first 3
                info!(
                    "   - {}: {} @ ${:.2} ({})",
                    order.order_id, order.direction, order.price, order.label
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get ETH-PERPETUAL orders: {}", e);
        }
    }
    println!();

    // =================================================================
    // 4. GET OPEN ORDERS BY LABEL (/private/get_open_orders_by_label)
    // =================================================================
    info!("🏷️  4. GET OPEN ORDERS BY LABEL");
    info!("--------------------------------");

    let test_labels = vec!["query_test_btc_1", "query_test_eth_1"];

    for label in test_labels {
        match client.get_open_orders_by_label(label).await {
            Ok(orders) => {
                info!("✅ Retrieved orders with label '{}' successfully", label);
                info!("📊 Orders with label '{}': {}", label, orders.len());

                for order in orders.iter() {
                    info!(
                        "   - {}: {} {} @ ${:.2}",
                        order.order_id, order.direction, order.instrument_name, order.price
                    );
                }
            }
            Err(e) => {
                warn!("⚠️  Failed to get orders with label '{}': {}", label, e);
            }
        }
    }
    println!();

    // =================================================================
    // 5. GET ORDER HISTORY BY CURRENCY (/private/get_order_history_by_currency)
    // =================================================================
    info!("📚 5. GET ORDER HISTORY BY CURRENCY");
    info!("------------------------------------");

    // Test BTC history
    match client
        .get_order_history_by_currency("BTC", Some("future"), Some(10), Some(0))
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved BTC order history successfully");
            info!("📊 BTC historical orders: {}", orders.len());

            for order in orders.iter().take(3) {
                // Show first 3
                info!(
                    "   - {}: {} {} @ ${:.2} [{}]",
                    order.order_id,
                    order.direction,
                    order.instrument_name,
                    order.price,
                    order.order_state
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get BTC order history: {}", e);
        }
    }

    // Test ETH history
    match client
        .get_order_history_by_currency("ETH", Some("future"), Some(10), Some(0))
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved ETH order history successfully");
            info!("📊 ETH historical orders: {}", orders.len());

            for order in orders.iter().take(3) {
                // Show first 3
                info!(
                    "   - {}: {} {} @ ${:.2} [{}]",
                    order.order_id,
                    order.direction,
                    order.instrument_name,
                    order.price,
                    order.order_state
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get ETH order history: {}", e);
        }
    }
    println!();

    // =================================================================
    // 6. GET ORDER HISTORY BY INSTRUMENT (/private/get_order_history_by_instrument)
    // =================================================================
    info!("🎯 6. GET ORDER HISTORY BY INSTRUMENT");
    info!("--------------------------------------");

    // Test BTC-PERPETUAL history
    match client
        .get_order_history_by_instrument("BTC-PERPETUAL", Some(10), Some(0))
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved BTC-PERPETUAL order history successfully");
            info!("📊 BTC-PERPETUAL historical orders: {}", orders.len());

            for order in orders.iter().take(3) {
                // Show first 3
                info!(
                    "   - {}: {} @ ${:.2} [{}] ({})",
                    order.order_id, order.direction, order.price, order.order_state, order.label
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get BTC-PERPETUAL order history: {}", e);
        }
    }

    // Test ETH-PERPETUAL history
    match client
        .get_order_history_by_instrument("ETH-PERPETUAL", Some(10), Some(0))
        .await
    {
        Ok(orders) => {
            info!("✅ Retrieved ETH-PERPETUAL order history successfully");
            info!("📊 ETH-PERPETUAL historical orders: {}", orders.len());

            for order in orders.iter().take(3) {
                // Show first 3
                info!(
                    "   - {}: {} @ ${:.2} [{}] ({})",
                    order.order_id, order.direction, order.price, order.order_state, order.label
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get ETH-PERPETUAL order history: {}", e);
        }
    }
    println!();

    // =================================================================
    // 7. GET ORDER STATE (/private/get_order_state)
    // =================================================================
    info!("🔍 7. GET ORDER STATE");
    info!("---------------------");

    // Test with created order IDs
    for (order_id, label) in created_order_ids.iter().take(3) {
        // Test first 3 orders
        match client.get_order_state(order_id).await {
            Ok(order_info) => {
                info!(
                    "✅ Retrieved order state for {} ({}) successfully",
                    order_id, label
                );
                info!("📊 Order Details:");
                info!("   - ID: {}", order_info.order_id);
                info!("   - State: {}", order_info.order_state);
                info!("   - Instrument: {}", order_info.instrument_name);
                info!("   - Direction: {}", order_info.direction);
                info!("   - Price: ${:.2}", order_info.price);
                info!("   - Amount: {:.6}", order_info.amount);
                info!("   - Filled: {:.6}", order_info.filled_amount);
                info!("   - Label: {}", order_info.label);
                info!("   - Time in Force: {}", order_info.time_in_force);
                info!("   - Post Only: {}", order_info.post_only);
                info!("   - Reduce Only: {}", order_info.reduce_only);
            }
            Err(e) => {
                warn!("⚠️  Failed to get order state for {}: {}", order_id, e);
            }
        }
        println!();
    }

    // =================================================================
    // CLEANUP: CANCEL TEST ORDERS
    // =================================================================
    info!("🧹 CLEANUP: CANCELLING TEST ORDERS");
    info!("-----------------------------------");

    let mut cancelled_count = 0;
    for (order_id, label) in created_order_ids.iter() {
        match client.cancel_order(order_id).await {
            Ok(order_info) => {
                info!(
                    "✅ Cancelled order {} ({}): {}",
                    order_id, label, order_info.order_state
                );
                cancelled_count += 1;
            }
            Err(e) => {
                warn!("⚠️  Could not cancel order {} ({}): {}", order_id, label, e);
            }
        }
    }

    info!(
        "📊 Successfully cancelled {}/{} orders",
        cancelled_count,
        created_order_ids.len()
    );
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("🎉 Order query endpoints example completed successfully!");
    info!("========================================================");
    info!("💡 Summary of demonstrated endpoints:");
    info!("   ✅ /private/get_open_orders - Get all open orders");
    info!("   ✅ /private/get_open_orders_by_currency - Get open orders by currency");
    info!("   ✅ /private/get_open_orders_by_instrument - Get open orders by instrument");
    info!("   ✅ /private/get_open_orders_by_label - Get open orders by label");
    info!("   ✅ /private/get_order_history_by_currency - Get order history by currency");
    info!("   ✅ /private/get_order_history_by_instrument - Get order history by instrument");
    info!("   ✅ /private/get_order_state - Get specific order state");
    info!("🔧 Auxiliary endpoints used:");
    info!("   📊 /private/buy and /private/sell - Create test orders");
    info!("   🧹 /private/cancel - Cleanup test orders");

    Ok(())
}
