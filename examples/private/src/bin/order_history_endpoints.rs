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

use deribit_http::model::trigger::Trigger;
use deribit_http::prelude::*;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};

// Función auxiliar para redondear precios según el tick size
fn round_to_tick_size(price: f64, tick_size: f64) -> f64 {
    if tick_size <= 0.0 {
        return price;
    }
    (price / tick_size).round() * tick_size
}

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    // Initialize logging
    setup_logger();

    // Create HTTP client
    let client = DeribitHttpClient::new();
    info!("✅ Successfully created Deribit client");

    // Test connectivity first
    info!("🔌 Testing connectivity to Deribit API...");
    match client.get_ticker("BTC-PERPETUAL").await {
        Ok(ticker) => {
            if let Some(price) = ticker.last_price {
                info!("✅ Connectivity test successful - BTC price: ${:.2}", price);
            } else {
                info!("✅ Connectivity test successful - BTC price: N/A");
            }
        }
        Err(e) => {
            error!("❌ Connectivity test failed: {}", e);
            info!("💡 Verifica que las credenciales estén configuradas correctamente:");
            info!("   - DERIBIT_CLIENT_ID");
            info!("   - DERIBIT_CLIENT_SECRET");
            info!("   - DERIBIT_TESTNET=true (opcional)");
            return Err(e);
        }
    }

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

    // Obtener información del instrumento para conocer el tick size
    info!("🔍 Obteniendo información del instrumento BTC-PERPETUAL para conocer el tick size...");
    let btc_instrument = match client.get_instrument("BTC-PERPETUAL").await {
        Ok(instrument) => {
            info!("✅ Información del instrumento obtenida");
            if let Some(tick_size) = instrument.tick_size {
                info!("📏 Tick size de BTC-PERPETUAL: {}", tick_size);
            } else {
                info!("⚠️ Tick size no disponible, usando valor por defecto");
            }
            instrument
        }
        Err(e) => {
            warn!("❌ Error al obtener información del instrumento: {}", e);
            // Usar valores por defecto si no se puede obtener la información
            info!("💡 Usando tick size por defecto de 0.5");
            // Crear un instrumento mínimo con tick size por defecto
            Instrument {
                tick_size: Some(0.5),
                instrument_name: "BTC-PERPETUAL".to_string(),
                instrument_id: None,
                kind: Some(InstrumentKind::Future),
                currency: Some("BTC".to_string()),
                contract_size: None,
                creation_timestamp: None,
                max_leverage: None,
                maker_commission: None,
                expiration_timestamp: None,
                settlement_period: None,
                instrument_type: None,
                quote_currency: Some("USD".to_string()),
                min_trade_amount: None,
                option_type: None,
                strike: None,
                base_currency: None,
                is_active: Some(true),
                price_index: None,
                settlement_currency: None,
                taker_commission: None,
                counter_currency: None,
            }
        }
    };

    // Obtener información del instrumento ETH-PERPETUAL
    info!("🔍 Obteniendo información del instrumento ETH-PERPETUAL...");
    let eth_instrument = match client.get_instrument("ETH-PERPETUAL").await {
        Ok(instrument) => {
            info!("✅ Información del instrumento obtenida");
            if let Some(tick_size) = instrument.tick_size {
                info!("📏 Tick size de ETH-PERPETUAL: {}", tick_size);
            }
            instrument
        }
        Err(e) => {
            warn!("❌ Error al obtener información del instrumento: {}", e);
            info!("💡 Usando tick size por defecto de 0.05");
            Instrument {
                tick_size: Some(0.05),
                instrument_name: "ETH-PERPETUAL".to_string(),
                instrument_id: None,
                kind: Some(InstrumentKind::Future),
                currency: Some("ETH".to_string()),
                contract_size: None,
                creation_timestamp: None,
                max_leverage: None,
                maker_commission: None,
                expiration_timestamp: None,
                settlement_period: None,
                instrument_type: None,
                quote_currency: Some("USD".to_string()),
                min_trade_amount: None,
                option_type: None,
                strike: None,
                base_currency: None,
                is_active: Some(true),
                price_index: None,
                settlement_currency: None,
                taker_commission: None,
                counter_currency: None,
            }
        }
    };

    // Redondear los precios de mercado según el tick size
    let btc_tick_size = btc_instrument.tick_size.unwrap_or(0.5);
    let eth_tick_size = eth_instrument.tick_size.unwrap_or(0.05);

    let rounded_btc_price = round_to_tick_size(btc_mark_price, btc_tick_size);
    let rounded_eth_price = round_to_tick_size(eth_mark_price, eth_tick_size);

    info!(
        "💰 Precio de BTC redondeado al tick size: {} -> {}",
        btc_mark_price, rounded_btc_price
    );
    info!(
        "💰 Precio de ETH redondeado al tick size: {} -> {}",
        eth_mark_price, rounded_eth_price
    );

    // Actualizar los precios base con los valores redondeados
    let btc_mark_price = rounded_btc_price;
    let eth_mark_price = rounded_eth_price;

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
            round_to_tick_size(btc_mark_price * 0.95, btc_tick_size),
            10.0,
            "buy",
            OrderType::Limit,
        ),
        (
            "BTC-PERPETUAL",
            "history_test_btc_2",
            round_to_tick_size(btc_mark_price * 1.05, btc_tick_size),
            10.0,
            "sell",
            OrderType::Limit,
        ),
        (
            "ETH-PERPETUAL",
            "history_test_eth_1",
            round_to_tick_size(eth_mark_price * 0.95, eth_tick_size),
            100.0,
            "buy",
            OrderType::Limit,
        ),
        (
            "ETH-PERPETUAL",
            "history_test_eth_2",
            round_to_tick_size(eth_mark_price * 1.05, eth_tick_size),
            100.0,
            "sell",
            OrderType::Limit,
        ),
        // Stop orders (using stop_limit type)
        (
            "BTC-PERPETUAL",
            "stop_history_btc_1",
            round_to_tick_size(btc_mark_price * 0.90, btc_tick_size),
            10.0,
            "buy",
            OrderType::StopLimit,
        ),
        (
            "BTC-PERPETUAL",
            "stop_history_btc_2",
            round_to_tick_size(btc_mark_price * 1.10, btc_tick_size),
            10.0,
            "sell",
            OrderType::StopLimit,
        ),
        (
            "ETH-PERPETUAL",
            "stop_history_eth_1",
            round_to_tick_size(eth_mark_price * 0.90, eth_tick_size),
            150.0,
            "buy",
            OrderType::StopLimit,
        ),
        (
            "ETH-PERPETUAL",
            "stop_history_eth_2",
            round_to_tick_size(eth_mark_price * 1.10, eth_tick_size),
            150.0,
            "sell",
            OrderType::StopLimit,
        ),
    ];

    for (instrument, label, price, amount, side, order_type) in test_orders {
        info!(
            "💲 Creando orden {} @ ${:.2} (tick size: {})",
            label,
            price,
            if instrument.contains("BTC") {
                btc_tick_size
            } else {
                eth_tick_size
            }
        );
        if side == "buy" {
            let (trigger_price_val, trigger_val) = if order_type == OrderType::StopLimit {
                // Para stop buy: trigger price por encima del precio actual
                let trigger_price = if instrument.contains("BTC") {
                    round_to_tick_size(btc_mark_price * 1.05, btc_tick_size)
                } else {
                    round_to_tick_size(eth_mark_price * 1.05, eth_tick_size)
                };
                (Some(trigger_price), Some(Trigger::LastPrice))
            } else {
                (None, None)
            };

            let buy_request = OrderRequest {
                order_id: None,
                instrument_name: instrument.to_string(),
                amount: Some(amount),
                contracts: None,
                type_: Some(order_type),
                label: Some(label.to_string()),
                price: Some(price),
                time_in_force: Some(TimeInForce::GoodTilCancelled),
                display_amount: None,
                post_only: Some(true), // Avoid immediate execution
                reject_post_only: None,
                reduce_only: Some(false),
                trigger_price: trigger_price_val,
                trigger_offset: None,
                trigger: trigger_val,
                advanced: None,
                mmp: None,
                valid_until: None,
                linked_order_type: None,
                trigger_fill_condition: None,
                otoco_config: None,
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
            let (trigger_price_val, trigger_val) = if order_type == OrderType::StopLimit {
                // Para stop sell: trigger price por debajo del precio actual
                let trigger_price = if instrument.contains("BTC") {
                    round_to_tick_size(btc_mark_price * 0.95, btc_tick_size)
                } else {
                    round_to_tick_size(eth_mark_price * 0.95, eth_tick_size)
                };
                (Some(trigger_price), Some(Trigger::LastPrice))
            } else {
                (None, None)
            };

            let sell_request = OrderRequest {
                order_id: None,
                instrument_name: instrument.to_string(),
                amount: Some(amount),
                contracts: None,
                type_: Some(order_type),
                label: Some(label.to_string()),
                price: Some(price),
                time_in_force: Some(TimeInForce::GoodTilCancelled),
                display_amount: None,
                post_only: Some(true), // Avoid immediate execution
                reject_post_only: None,
                reduce_only: Some(false),
                trigger_price: trigger_price_val,
                trigger_offset: None,
                trigger: trigger_val,
                advanced: None,
                mmp: None,
                valid_until: None,
                linked_order_type: None,
                trigger_fill_condition: None,
                otoco_config: None,
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

    // Log detailed information about created orders
    if created_order_ids.is_empty() {
        warn!("⚠️  No orders were created successfully - this may affect history demonstration");
    } else {
        info!(
            "✅ Successfully created {} orders for history",
            created_order_ids.len()
        );
        for (order_id, label, order_type) in &created_order_ids {
            info!("   - Order {}: {} ({:?})", order_id, label, order_type);
        }
    }
    println!();

    // Wait for orders to be registered
    sleep(Duration::from_secs(2)).await;

    // Cancel some orders to create history entries
    let orders_to_cancel = std::cmp::min(4, created_order_ids.len());
    info!(
        "🔄 Cancelling {} out of {} orders to create history entries...",
        orders_to_cancel,
        created_order_ids.len()
    );

    if orders_to_cancel == 0 {
        warn!("⚠️  No orders available to cancel - skipping cancellation step");
    } else {
        for (order_id, label, _) in created_order_ids.iter().take(orders_to_cancel) {
            info!("🔧 Attempting to cancel order {} ({})", order_id, label);
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
                        order.amount,
                        order.filled_amount.unwrap_or(0.0),
                        order.average_price.unwrap_or(0.0)
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
                        order.amount,
                        order.filled_amount.unwrap_or(0.0),
                        order.post_only
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
                        order.amount,
                        order.filled_amount.unwrap_or(0.0),
                        order.reduce_only
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
