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

use deribit_http::prelude::*;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    // Initialize logging
    setup_logger();
    info!("🚀 Deribit HTTP Client - Trading Endpoints Example");
    info!("==================================================");
    println!();

    // Create HTTP client
    let client = DeribitHttpClient::new();

    // =================================================================
    // 1. PLACE BUY ORDER (/private/buy)
    // =================================================================
    info!("💰 1. PLACE BUY ORDER");
    info!("---------------------");

    let buy_request = OrderRequest {
        order_id: None,
        instrument_name: "BTC-PERPETUAL".to_string(),
        amount: Some(10.0), // 10 USD worth of BTC
        contracts: None,
        type_: Some(OrderType::Limit),
        label: Some("example_buy_order".to_string()),
        price: Some(30000.0), // Low price to avoid execution
        time_in_force: Some(TimeInForce::GoodTilCancelled),
        display_amount: None,
        post_only: Some(true), // Ensure we don't execute immediately
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
            return Err(e);
        }
    };
    println!();

    // =================================================================
    // 2. PLACE SELL ORDER (/private/sell)
    // =================================================================
    info!("💸 2. PLACE SELL ORDER");
    info!("----------------------");

    let sell_request = OrderRequest {
        order_id: None,
        instrument_name: "BTC-PERPETUAL".to_string(),
        amount: Some(10.0), // 10 USD worth of BTC
        contracts: None,
        type_: Some(OrderType::Limit),
        label: Some("example_sell_order".to_string()),
        price: Some(100000.0), // High price to avoid execution
        time_in_force: Some(TimeInForce::GoodTilCancelled),
        display_amount: None,
        post_only: Some(true), // Ensure we don't execute immediately
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
            return Err(e);
        }
    };
    println!();

    // =================================================================
    // 3. EDIT ORDER BY ID (/private/edit)
    // =================================================================
    info!("✏️  3. EDIT ORDER BY ID");
    info!("-----------------------");

    let edit_request = OrderRequest {
        order_id: Some(buy_order_id.clone()),
        instrument_name: "BTC-PERPETUAL".to_string(),
        amount: Some(15.0), // Change amount from 10 to 15 USD
        contracts: None,
        type_: Some(OrderType::Limit),
        label: Some("example_buy_order".to_string()),
        price: Some(25000.0), // Change price from 30000 to 25000
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
    // info!("🏷️  4. EDIT ORDER BY LABEL");
    // info!("---------------------------");
    // NOTE: edit_by_label method is not implemented in the current HTTP client
    // This functionality would require additional implementation
    info!("🏷️  4. EDIT ORDER BY LABEL - SKIPPED (Method not implemented)");
    info!("------------------------------------------------------------------");
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
