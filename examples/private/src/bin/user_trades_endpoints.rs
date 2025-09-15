//! User Trades Endpoints Example
//!
//! This example demonstrates the correct functioning of the following private endpoints:
//! - `/private/get_user_trades_by_currency` - User trades by currency
//! - `/private/get_user_trades_by_currency_and_time` - User trades by currency and time
//! - `/private/get_user_trades_by_instrument` - User trades by instrument
//! - `/private/get_user_trades_by_instrument_and_time` - User trades by instrument and time
//! - `/private/get_user_trades_by_order` - User trades by order
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
//! Then run: cargo run --bin user_trades_endpoints

use deribit_http::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    // Initialize logging
    setup_logger();

    info!("🚀 Deribit HTTP Client - User Trades Endpoints Example");
    info!("======================================================");
    println!();

    // Create HTTP client
    let client = DeribitHttpClient::new();

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

    // Calculate current timestamp and time range for queries
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let one_day_ago = current_timestamp - (24 * 60 * 60 * 1000); // 24 hours ago
    let one_hour_ago = current_timestamp - (60 * 60 * 1000); // 1 hour ago

    info!("⏰ Time range for queries:");
    info!("   Current timestamp: {}", current_timestamp);
    info!("   One day ago: {}", one_day_ago);
    info!("   One hour ago: {}", one_hour_ago);
    println!();

    // =================================================================
    // SETUP: CREATE TEST ORDERS (MIGHT GENERATE TRADES)
    // =================================================================
    info!("🎯 SETUP: CREATING TEST ORDERS");
    info!("--------------------------------");

    let mut created_order_ids = Vec::new();

    // Create market buy/sell orders that might execute and generate trades
    let test_orders = vec![
        (
            "BTC-PERPETUAL",
            "user_trades_test_btc_1",
            btc_mark_price * 0.999,
            10.0,
            "buy",
        ), // Slightly below market
        (
            "BTC-PERPETUAL",
            "user_trades_test_btc_2",
            btc_mark_price * 1.001,
            10.0,
            "sell",
        ), // Slightly above market
        (
            "ETH-PERPETUAL",
            "user_trades_test_eth_1",
            eth_mark_price * 0.999,
            100.0,
            "buy",
        ), // Slightly below market
        (
            "ETH-PERPETUAL",
            "user_trades_test_eth_2",
            eth_mark_price * 1.001,
            100.0,
            "sell",
        ), // Slightly above market
    ];

    for (instrument, label, price, amount, side) in test_orders {
        if side == "buy" {
            let buy_request = OrderRequest {
                order_id: None,
                instrument_name: instrument.to_string(),
                amount: Some(amount),
                contracts: None,
                type_: Some(OrderType::Limit),
                label: Some(label.to_string()),
                price: Some(price),
                time_in_force: Some(TimeInForce::GoodTilCancelled),
                display_amount: None,
                post_only: Some(false), // Allow execution to potentially generate trades
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

            match client.buy_order(buy_request).await {
                Ok(order_response) => {
                    info!(
                        "✅ Created buy order: {} ({}) @ ${:.2}",
                        order_response.order.order_id, label, price
                    );
                    created_order_ids.push((order_response.order.order_id, label.to_string()));

                    // Check if order was executed
                    if order_response.order.filled_amount > 0.0 {
                        info!(
                            "🎉 Order partially filled: {:.6} filled",
                            order_response.order.filled_amount
                        );
                    }
                }
                Err(e) => {
                    warn!("⚠️  Failed to create buy order for {}: {}", label, e);
                }
            }
        } else {
            let sell_request = OrderRequest {
                order_id: None,
                instrument_name: instrument.to_string(),
                amount: Some(amount),
                contracts: None,
                type_: Some(OrderType::Limit),
                label: Some(label.to_string()),
                price: Some(price),
                time_in_force: Some(TimeInForce::GoodTilCancelled),
                display_amount: None,
                post_only: Some(false), // Allow execution to potentially generate trades
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

            match client.sell_order(sell_request).await {
                Ok(order_response) => {
                    info!(
                        "✅ Created sell order: {} ({}) @ ${:.2}",
                        order_response.order.order_id, label, price
                    );
                    created_order_ids.push((order_response.order.order_id, label.to_string()));

                    // Check if order was executed
                    if order_response.order.filled_amount > 0.0 {
                        info!(
                            "🎉 Order partially filled: {:.6} filled",
                            order_response.order.filled_amount
                        );
                    }
                }
                Err(e) => {
                    warn!("⚠️  Failed to create sell order for {}: {}", label, e);
                }
            }
        }
    }

    info!("📊 Created {} test orders", created_order_ids.len());
    println!();

    // Wait for orders to be processed and potentially generate trades
    sleep(Duration::from_secs(3)).await;

    // =================================================================
    // 1. GET USER TRADES BY CURRENCY (/private/get_user_trades_by_currency)
    // =================================================================
    info!("💰 1. GET USER TRADES BY CURRENCY");
    info!("----------------------------------");

    // Test BTC currency trades
    match client
        .get_user_trades_by_currency(
            "BTC",
            Some("future"),
            None,
            None,
            Some(10),
            Some(true),
            Some("desc"),
        )
        .await
    {
        Ok(trades) => {
            info!("✅ Retrieved BTC user trades successfully");
            info!("📊 BTC trades count: {}", trades.len());

            if !trades.is_empty() {
                info!("📝 Recent BTC trades:");
                for trade in trades.iter().take(3) {
                    // Show first 3
                    info!(
                        "   - Trade {}: {} {} @ ${:.2} (Fee: {:.8} {}, Liquidity: {})",
                        trade.trade_id,
                        trade.direction,
                        trade.instrument_name,
                        trade.price,
                        trade.fee,
                        trade.fee_currency,
                        trade.liquidity
                    );
                    info!(
                        "     Order ID: {}, Label: {}, Amount: {:.6}",
                        trade.order_id, trade.label, trade.amount
                    );
                }
            } else {
                info!("ℹ️  No recent BTC trades found (this is normal if no trades were executed)");
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get BTC user trades: {}", e);
        }
    }

    // Test ETH currency trades
    match client
        .get_user_trades_by_currency(
            "ETH",
            Some("future"),
            None,
            None,
            Some(10),
            Some(true),
            Some("desc"),
        )
        .await
    {
        Ok(trades) => {
            info!("✅ Retrieved ETH user trades successfully");
            info!("📊 ETH trades count: {}", trades.len());

            if !trades.is_empty() {
                info!("📝 Recent ETH trades:");
                for trade in trades.iter().take(3) {
                    // Show first 3
                    info!(
                        "   - Trade {}: {} {} @ ${:.2} (Fee: {:.8} {}, Liquidity: {})",
                        trade.trade_id,
                        trade.direction,
                        trade.instrument_name,
                        trade.price,
                        trade.fee,
                        trade.fee_currency,
                        trade.liquidity
                    );
                    info!(
                        "     Order ID: {}, Label: {}, Amount: {:.6}",
                        trade.order_id, trade.label, trade.amount
                    );
                }
            } else {
                info!("ℹ️  No recent ETH trades found (this is normal if no trades were executed)");
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get ETH user trades: {}", e);
        }
    }
    println!();

    // =================================================================
    // 2. GET USER TRADES BY CURRENCY AND TIME (/private/get_user_trades_by_currency_and_time)
    // =================================================================
    info!("⏰ 2. GET USER TRADES BY CURRENCY AND TIME");
    info!("------------------------------------------");

    // Test BTC trades in the last 24 hours
    match client
        .get_user_trades_by_currency_and_time(
            "BTC",
            one_day_ago,
            current_timestamp,
            Some("future"),
            Some(20),
            Some(true),
            Some("desc"),
        )
        .await
    {
        Ok(trades) => {
            info!("✅ Retrieved BTC trades for last 24 hours successfully");
            info!("📊 BTC trades in last 24h: {}", trades.len());

            if !trades.is_empty() {
                info!("📝 BTC trades in time range:");
                for trade in trades.iter().take(2) {
                    // Show first 2
                    let trade_time =
                        chrono::DateTime::from_timestamp((trade.timestamp / 1000) as i64, 0)
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                            .unwrap_or_else(|| "Unknown".to_string());

                    info!(
                        "   - {} @ ${:.2} on {} ({} {})",
                        trade.direction,
                        trade.price,
                        trade_time,
                        trade.amount,
                        trade.instrument_name
                    );
                    info!(
                        "     Trade ID: {}, Seq: {}, Mark Price: ${:.2}",
                        trade.trade_id, trade.trade_seq, trade.mark_price
                    );
                }
            } else {
                info!("ℹ️  No BTC trades found in the last 24 hours");
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get BTC trades by time: {}", e);
        }
    }

    // Test ETH trades in the last hour
    match client
        .get_user_trades_by_currency_and_time(
            "ETH",
            one_hour_ago,
            current_timestamp,
            Some("future"),
            Some(20),
            Some(true),
            Some("desc"),
        )
        .await
    {
        Ok(trades) => {
            info!("✅ Retrieved ETH trades for last hour successfully");
            info!("📊 ETH trades in last hour: {}", trades.len());

            if !trades.is_empty() {
                info!("📝 ETH trades in time range:");
                for trade in trades.iter().take(2) {
                    // Show first 2
                    let trade_time =
                        chrono::DateTime::from_timestamp((trade.timestamp / 1000) as i64, 0)
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                            .unwrap_or_else(|| "Unknown".to_string());

                    info!(
                        "   - {} @ ${:.2} on {} ({} {})",
                        trade.direction,
                        trade.price,
                        trade_time,
                        trade.amount,
                        trade.instrument_name
                    );
                    info!(
                        "     Trade ID: {}, Seq: {}, Mark Price: ${:.2}",
                        trade.trade_id, trade.trade_seq, trade.mark_price
                    );
                }
            } else {
                info!("ℹ️  No ETH trades found in the last hour");
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get ETH trades by time: {}", e);
        }
    }
    println!();

    // =================================================================
    // 3. GET USER TRADES BY INSTRUMENT (/private/get_user_trades_by_instrument)
    // =================================================================
    info!("🎯 3. GET USER TRADES BY INSTRUMENT");
    info!("------------------------------------");

    // Test BTC-PERPETUAL trades
    match client
        .get_user_trades_by_instrument(
            "BTC-PERPETUAL",
            None,
            None,
            Some(10),
            Some(true),
            Some("desc"),
        )
        .await
    {
        Ok(trades) => {
            info!("✅ Retrieved BTC-PERPETUAL trades successfully");
            info!("📊 BTC-PERPETUAL trades count: {}", trades.len());

            if !trades.is_empty() {
                info!("📝 Recent BTC-PERPETUAL trades:");
                for trade in trades.iter().take(3) {
                    // Show first 3
                    info!(
                        "   - Trade {}: {} @ ${:.2} (Amount: {:.6}, Fee: {:.8})",
                        trade.trade_id, trade.direction, trade.price, trade.amount, trade.fee
                    );
                    info!(
                        "     Timestamp: {}, Liquidity: {}, Self Trade: {}",
                        trade.timestamp, trade.liquidity, trade.self_trade
                    );
                }
            } else {
                info!("ℹ️  No recent BTC-PERPETUAL trades found");
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get BTC-PERPETUAL trades: {}", e);
        }
    }

    // Test ETH-PERPETUAL trades
    match client
        .get_user_trades_by_instrument(
            "ETH-PERPETUAL",
            None,
            None,
            Some(10),
            Some(true),
            Some("desc"),
        )
        .await
    {
        Ok(trades) => {
            info!("✅ Retrieved ETH-PERPETUAL trades successfully");
            info!("📊 ETH-PERPETUAL trades count: {}", trades.len());

            if !trades.is_empty() {
                info!("📝 Recent ETH-PERPETUAL trades:");
                for trade in trades.iter().take(3) {
                    // Show first 3
                    info!(
                        "   - Trade {}: {} @ ${:.2} (Amount: {:.6}, Fee: {:.8})",
                        trade.trade_id, trade.direction, trade.price, trade.amount, trade.fee
                    );
                    info!(
                        "     Timestamp: {}, Liquidity: {}, Self Trade: {}",
                        trade.timestamp, trade.liquidity, trade.self_trade
                    );
                }
            } else {
                info!("ℹ️  No recent ETH-PERPETUAL trades found");
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get ETH-PERPETUAL trades: {}", e);
        }
    }
    println!();

    // =================================================================
    // 4. GET USER TRADES BY INSTRUMENT AND TIME (/private/get_user_trades_by_instrument_and_time)
    // =================================================================
    info!("⏰ 4. GET USER TRADES BY INSTRUMENT AND TIME");
    info!("--------------------------------------------");

    // Test BTC-PERPETUAL trades in the last 24 hours
    match client
        .get_user_trades_by_instrument_and_time(
            "BTC-PERPETUAL",
            one_day_ago,
            current_timestamp,
            Some(15),
            Some(true),
            Some("desc"),
        )
        .await
    {
        Ok(trades) => {
            info!("✅ Retrieved BTC-PERPETUAL trades for last 24 hours successfully");
            info!("📊 BTC-PERPETUAL trades in last 24h: {}", trades.len());

            if !trades.is_empty() {
                info!("📝 BTC-PERPETUAL trades in time range:");
                for trade in trades.iter().take(2) {
                    // Show first 2
                    let trade_time =
                        chrono::DateTime::from_timestamp((trade.timestamp / 1000) as i64, 0)
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                            .unwrap_or_else(|| "Unknown".to_string());

                    info!(
                        "   - {} @ ${:.2} on {}",
                        trade.direction, trade.price, trade_time
                    );
                    info!(
                        "     Amount: {:.6}, Index Price: ${:.2}, Tick Dir: {}",
                        trade.amount, trade.index_price, trade.tick_direction
                    );
                }
            } else {
                info!("ℹ️  No BTC-PERPETUAL trades found in the last 24 hours");
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get BTC-PERPETUAL trades by time: {}", e);
        }
    }

    // Test ETH-PERPETUAL trades in the last hour
    match client
        .get_user_trades_by_instrument_and_time(
            "ETH-PERPETUAL",
            one_hour_ago,
            current_timestamp,
            Some(15),
            Some(true),
            Some("desc"),
        )
        .await
    {
        Ok(trades) => {
            info!("✅ Retrieved ETH-PERPETUAL trades for last hour successfully");
            info!("📊 ETH-PERPETUAL trades in last hour: {}", trades.len());

            if !trades.is_empty() {
                info!("📝 ETH-PERPETUAL trades in time range:");
                for trade in trades.iter().take(2) {
                    // Show first 2
                    let trade_time =
                        chrono::DateTime::from_timestamp((trade.timestamp / 1000) as i64, 0)
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                            .unwrap_or_else(|| "Unknown".to_string());

                    info!(
                        "   - {} @ ${:.2} on {}",
                        trade.direction, trade.price, trade_time
                    );
                    info!(
                        "     Amount: {:.6}, Index Price: ${:.2}, Tick Dir: {}",
                        trade.amount, trade.index_price, trade.tick_direction
                    );
                }
            } else {
                info!("ℹ️  No ETH-PERPETUAL trades found in the last hour");
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to get ETH-PERPETUAL trades by time: {}", e);
        }
    }
    println!();

    // =================================================================
    // 5. GET USER TRADES BY ORDER (/private/get_user_trades_by_order)
    // =================================================================
    info!("📋 5. GET USER TRADES BY ORDER");
    info!("------------------------------");

    // Test with created order IDs
    for (order_id, label) in created_order_ids.iter().take(3) {
        // Test first 3 orders
        match client
            .get_user_trades_by_order(order_id, Some("desc"))
            .await
        {
            Ok(trades) => {
                info!(
                    "✅ Retrieved trades for order {} ({}) successfully",
                    order_id, label
                );
                info!("📊 Trades for this order: {}", trades.len());

                if !trades.is_empty() {
                    info!("📝 Order trades:");
                    for trade in trades.iter() {
                        info!(
                            "   - Trade {}: {} @ ${:.2}",
                            trade.trade_id, trade.direction, trade.price
                        );
                        info!(
                            "     Amount: {:.6}, Fee: {:.8} {}, Liquidity: {}",
                            trade.amount, trade.fee, trade.fee_currency, trade.liquidity
                        );
                        info!(
                            "     Order Type: {}, Matching ID: {:?}",
                            trade.order_type, trade.matching_id
                        );
                    }
                } else {
                    info!(
                        "ℹ️  No trades found for order {} (order may not have executed)",
                        order_id
                    );
                }
            }
            Err(e) => {
                warn!("⚠️  Failed to get trades for order {}: {}", order_id, e);
            }
        }
        println!();
    }

    // =================================================================
    // CLEANUP: CANCEL REMAINING TEST ORDERS
    // =================================================================
    info!("🧹 CLEANUP: CANCELLING REMAINING TEST ORDERS");
    info!("---------------------------------------------");

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
                warn!(
                    "⚠️  Could not cancel order {} ({}): {} (may have been executed)",
                    order_id, label, e
                );
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
    info!("🎉 User trades endpoints example completed successfully!");
    info!("========================================================");
    info!("💡 Summary of demonstrated endpoints:");
    info!("   ✅ /private/get_user_trades_by_currency - User trades by currency");
    info!("   ✅ /private/get_user_trades_by_currency_and_time - User trades by currency and time");
    info!("   ✅ /private/get_user_trades_by_instrument - User trades by instrument");
    info!(
        "   ✅ /private/get_user_trades_by_instrument_and_time - User trades by instrument and time"
    );
    info!("   ✅ /private/get_user_trades_by_order - User trades by order");
    info!("🔧 Auxiliary endpoints used:");
    info!("   📊 /public/ticker - Get current market prices");
    info!("   📊 /private/buy and /private/sell - Create test orders");
    info!("   🧹 /private/cancel - Cleanup test orders");
    info!("");
    info!("ℹ️  Note: Trade data availability depends on actual trade execution.");
    info!("   In testnet, trades may not always execute due to market conditions.");

    Ok(())
}
