//! Mass Quote Endpoints Example
//!
//! This example demonstrates the correct functioning of the following private endpoints:
//! - `/private/mass_quote` - Mass quote submission
//! - `/private/cancel_quotes` - Cancel mass quotes
//!
//! This example uses auxiliary endpoints to verify quote states and demonstrate functionality:
//! - `/public/get_order_book` - Get current market prices for realistic quotes
//! - `/private/get_open_orders_by_instrument` - Verify quote orders
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true" (optional, defaults to true)
//!
//! Then run: cargo run --bin mass_quote_endpoints

use deribit_http::prelude::*;
use std::env;
use std::path::Path;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    // Load environment variables from .env file if it exists
    if Path::new(".env").exists() {
        dotenv::dotenv().ok();
    }

    setup_logger();

    info!("🚀 Deribit HTTP Client - Mass Quote Endpoints Example");
    info!("====================================================");
    println!();

    // Get authentication credentials from environment
    let _client_id = env::var("DERIBIT_CLIENT_ID").map_err(|_| {
        HttpError::ConfigError("Missing DERIBIT_CLIENT_ID environment variable".to_string())
    })?;
    let _client_secret = env::var("DERIBIT_CLIENT_SECRET").map_err(|_| {
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
    let client = DeribitHttpClient::default();
    info!(
        "✅ HTTP client created for {}: {}",
        if use_testnet { "testnet" } else { "production" },
        client.base_url()
    );
    println!();

    // =================================================================
    // SETUP: GET CURRENT MARKET PRICES FOR REALISTIC QUOTES
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
    // 1. MASS QUOTE SUBMISSION (/private/mass_quote)
    // =================================================================
    info!("📝 1. MASS QUOTE SUBMISSION");
    info!("----------------------------");

    // Create mass quotes for multiple instruments with realistic spreads
    let btc_bid_price = btc_mark_price * 0.99; // 1% below mark
    let btc_ask_price = btc_mark_price * 1.01; // 1% above mark
    let eth_bid_price = eth_mark_price * 0.99; // 1% below mark
    let eth_ask_price = eth_mark_price * 1.01; // 1% above mark

    let mass_quote_items = vec![
        // BTC bid quote
        MassQuoteItem {
            instrument_name: "BTC-PERPETUAL".to_string(),
            side: OrderSide::Buy,
            amount: 100.0, // 100 USD worth
            price: btc_bid_price,
        },
        // BTC ask quote
        MassQuoteItem {
            instrument_name: "BTC-PERPETUAL".to_string(),
            side: OrderSide::Sell,
            amount: 100.0, // 100 USD worth
            price: btc_ask_price,
        },
        // ETH bid quote
        MassQuoteItem {
            instrument_name: "ETH-PERPETUAL".to_string(),
            side: OrderSide::Buy,
            amount: 500.0, // 500 USD worth
            price: eth_bid_price,
        },
        // ETH ask quote
        MassQuoteItem {
            instrument_name: "ETH-PERPETUAL".to_string(),
            side: OrderSide::Sell,
            amount: 500.0, // 500 USD worth
            price: eth_ask_price,
        },
        // Additional BTC bid at lower price
        MassQuoteItem {
            instrument_name: "BTC-PERPETUAL".to_string(),
            side: OrderSide::Buy,
            amount: 50.0,
            price: btc_mark_price * 0.98, // 2% below mark
        },
        // Additional ETH ask at higher price
        MassQuoteItem {
            instrument_name: "ETH-PERPETUAL".to_string(),
            side: OrderSide::Sell,
            amount: 250.0,
            price: eth_mark_price * 1.02, // 2% above mark
        },
    ];

    let mass_quotes = MassQuoteRequest {
        items: mass_quote_items,
        label: Some("example_mass_quotes".to_string()),
    };

    info!(
        "📊 Submitting mass quotes with {} items:",
        mass_quotes.items.len()
    );
    for (i, item) in mass_quotes.items.iter().enumerate() {
        let side_str = match item.side {
            OrderSide::Buy => "📉 Bid",
            OrderSide::Sell => "📈 Ask",
        };
        info!(
            "   {}. {} - {}: ${:.2} (Amount: {:.0} USD)",
            i + 1,
            item.instrument_name,
            side_str,
            item.price,
            item.amount
        );
    }

    match client.mass_quote(mass_quotes).await {
        Ok(response) => {
            info!("✅ Mass quote submission successful");
            info!("📊 Quote results:");

            for (i, result) in response.quotes.iter().enumerate() {
                let status_icon = if result.success { "✅" } else { "❌" };
                info!(
                    "   {}) {} {}: {}",
                    i + 1,
                    status_icon,
                    result.instrument_name,
                    if result.success { "Success" } else { "Failed" }
                );

                if let Some(error) = &result.error {
                    info!("      ⚠️  Error: {}", error);
                }
            }

            let success_count = response.quotes.iter().filter(|q| q.success).count();
            let total_count = response.quotes.len();
            info!(
                "📈 Summary: {}/{} quotes successful",
                success_count, total_count
            );
        }
        Err(e) => {
            error!("❌ Mass quote submission failed: {}", e);
            info!("ℹ️  This might happen if the market is closed or parameters are invalid");
        }
    }
    println!();

    // Wait for quotes to be processed
    sleep(Duration::from_secs(2)).await;

    // =================================================================
    // 2. VERIFY QUOTES WITH AUXILIARY ENDPOINTS
    // =================================================================
    info!("🔍 2. VERIFY QUOTES (AUXILIARY ENDPOINT)");
    info!("----------------------------------------");

    // Check for quote orders in BTC-PERPETUAL
    match client
        .get_open_orders_by_instrument("BTC-PERPETUAL", None)
        .await
    {
        Ok(orders) => {
            let quote_orders: Vec<_> = orders
                .iter()
                .filter(|order| order.label.contains("quote") || order.order_type == "limit")
                .collect();

            info!("📊 BTC-PERPETUAL orders found: {}", orders.len());
            if !quote_orders.is_empty() {
                info!("📝 Potential quote orders:");
                for order in quote_orders.iter().take(3) {
                    // Show first 3
                    info!(
                        "   - {} @ ${:.2}: {}",
                        order.order_id, order.price, order.direction
                    );
                }
            }
        }
        Err(e) => {
            warn!("⚠️  Could not retrieve BTC orders: {}", e);
        }
    }

    // Check for quote orders in ETH-PERPETUAL
    match client
        .get_open_orders_by_instrument("ETH-PERPETUAL", None)
        .await
    {
        Ok(orders) => {
            let quote_orders: Vec<_> = orders
                .iter()
                .filter(|order| order.label.contains("quote") || order.order_type == "limit")
                .collect();

            info!("📊 ETH-PERPETUAL orders found: {}", orders.len());
            if !quote_orders.is_empty() {
                info!("📝 Potential quote orders:");
                for order in quote_orders.iter().take(3) {
                    // Show first 3
                    info!(
                        "   - {} @ ${:.2}: {}",
                        order.order_id, order.price, order.direction
                    );
                }
            }
        }
        Err(e) => {
            warn!("⚠️  Could not retrieve ETH orders: {}", e);
        }
    }
    println!();

    // =================================================================
    // 3. CANCEL MASS QUOTES (/private/cancel_quotes)
    // =================================================================
    info!("❌ 3. CANCEL MASS QUOTES");
    info!("------------------------");

    match client.cancel_quotes(Some("all")).await {
        Ok(cancelled_count) => {
            info!("✅ Mass quote cancellation successful");
            info!("📊 Cancelled quotes count: {}", cancelled_count);

            if cancelled_count > 0 {
                info!("🧹 Successfully cancelled {} quote orders", cancelled_count);
            } else {
                info!("ℹ️  No quote orders were found to cancel");
            }
        }
        Err(e) => {
            warn!("⚠️  Mass quote cancellation failed: {}", e);
            info!("ℹ️  This might happen if there were no active quotes to cancel");
        }
    }
    println!();

    // Wait for cancellations to be processed
    sleep(Duration::from_secs(1)).await;

    // =================================================================
    // 4. VERIFICATION: CHECK REMAINING QUOTE ORDERS
    // =================================================================
    info!("🔍 4. VERIFICATION: CHECK REMAINING QUOTE ORDERS");
    info!("------------------------------------------------");

    // Verify BTC quotes were cancelled
    match client
        .get_open_orders_by_instrument("BTC-PERPETUAL", None)
        .await
    {
        Ok(orders) => {
            let quote_orders: Vec<_> = orders
                .iter()
                .filter(|order| order.label.contains("quote"))
                .collect();

            info!("📊 Remaining BTC quote orders: {}", quote_orders.len());
            if quote_orders.is_empty() {
                info!("✅ All BTC quote orders successfully cancelled");
            } else {
                info!("⚠️  Some BTC quote orders remain:");
                for order in quote_orders.iter().take(2) {
                    info!("   - {} @ ${:.2}", order.order_id, order.price);
                }
            }
        }
        Err(e) => {
            warn!("⚠️  Could not verify BTC quote cancellation: {}", e);
        }
    }

    // Verify ETH quotes were cancelled
    match client
        .get_open_orders_by_instrument("ETH-PERPETUAL", None)
        .await
    {
        Ok(orders) => {
            let quote_orders: Vec<_> = orders
                .iter()
                .filter(|order| order.label.contains("quote"))
                .collect();

            info!("📊 Remaining ETH quote orders: {}", quote_orders.len());
            if quote_orders.is_empty() {
                info!("✅ All ETH quote orders successfully cancelled");
            } else {
                info!("⚠️  Some ETH quote orders remain:");
                for order in quote_orders.iter().take(2) {
                    info!("   - {} @ ${:.2}", order.order_id, order.price);
                }
            }
        }
        Err(e) => {
            warn!("⚠️  Could not verify ETH quote cancellation: {}", e);
        }
    }

    println!();
    info!("🎉 Mass quote endpoints example completed successfully!");
    info!("======================================================");
    info!("💡 Summary of demonstrated endpoints:");
    info!("   ✅ /private/mass_quote - Mass quote submission");
    info!("   ✅ /private/cancel_quotes - Cancel mass quotes");
    info!("🔧 Auxiliary endpoints used:");
    info!("   📊 /public/ticker - Get current market prices");
    info!("   🔍 /private/get_open_orders_by_instrument - Verify quote states");

    Ok(())
}
