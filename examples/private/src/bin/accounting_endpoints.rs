//! Accounting Endpoints Example
//!
//! This example demonstrates the correct functioning of the following private endpoints:
//! - `/private/get_positions` - Get all positions with optional filters
//! - Position filtering and analysis for specific instruments
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true" (optional, defaults to true)
//!
//! Then run: cargo run --bin accounting_endpoints

use deribit_base::prelude::setup_logger;
use deribit_http::{DeribitHttpClient, HttpError};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    // Initialize logging
    setup_logger();

    // Create HTTP client
    let client = DeribitHttpClient::new();

    // =================================================================
    // 1. GET ALL POSITIONS (/private/get_positions)
    // =================================================================
    info!("📊 1. GET ALL POSITIONS");
    info!("-----------------------");

    let all_positions = match client.get_positions(None, None, None).await {
        Ok(positions) => {
            info!("✅ Retrieved all positions successfully");
            info!("📈 Total positions found: {}", positions.len());

            if positions.is_empty() {
                warn!(
                    "⚠️  No positions found. You may need to have some open positions to see results."
                );
                return Ok(());
            }

            // Display summary of all positions
            for (i, position) in positions.iter().enumerate() {
                info!("📋 Position {}: {}", i + 1, position.instrument_name);
                info!("   💰 Size: {:.6}", position.size);
                info!(
                    "   💵 Mark Price: ${:.2}",
                    position.mark_price.unwrap_or(0.0)
                );
                info!("   📊 Direction: {:?}", position.direction);
                info!(
                    "   💸 Unrealized PnL: ${:.2}",
                    position.unrealized_profit_loss.unwrap_or(0.0)
                );
                info!("   📈 Average Price: ${:.2}", position.average_price);
                println!();
            }

            positions
        }
        Err(e) => {
            error!("❌ Failed to get positions: {}", e);
            return Err(e);
        }
    };

    // =================================================================
    // 2. GET POSITIONS BY CURRENCY (/private/get_positions?currency=BTC)
    // =================================================================
    info!("₿ 2. GET BTC POSITIONS");
    info!("----------------------");

    let _btc_positions = match client.get_positions(Some("BTC"), None, None).await {
        Ok(positions) => {
            info!("✅ Retrieved BTC positions successfully");
            info!("₿ BTC positions found: {}", positions.len());

            for position in &positions {
                info!("📋 BTC Position: {}", position.instrument_name);
                info!("   💰 Size: {:.6} BTC", position.size);
                info!(
                    "   💵 Mark Price: ${:.2}",
                    position.mark_price.unwrap_or(0.0)
                );
                info!("   📊 Direction: {:?}", position.direction);
                info!(
                    "   💸 Unrealized PnL: ${:.2}",
                    position.unrealized_profit_loss.unwrap_or(0.0)
                );
                println!();
            }

            positions
        }
        Err(e) => {
            warn!("⚠️  Failed to get BTC positions: {}", e);
            Vec::new()
        }
    };

    // =================================================================
    // 3. GET POSITIONS BY KIND (/private/get_positions?kind=future)
    // =================================================================
    info!("🔮 3. GET FUTURE POSITIONS");
    info!("---------------------------");

    let _future_positions = match client.get_positions(None, Some("future"), None).await {
        Ok(positions) => {
            info!("✅ Retrieved future positions successfully");
            info!("🔮 Future positions found: {}", positions.len());

            for position in &positions {
                info!("📋 Future Position: {}", position.instrument_name);
                info!("   💰 Size: {:.6}", position.size);
                info!(
                    "   💵 Mark Price: ${:.2}",
                    position.mark_price.unwrap_or(0.0)
                );
                info!("   📊 Direction: {:?}", position.direction);
                info!(
                    "   💸 Unrealized PnL: ${:.2}",
                    position.unrealized_profit_loss.unwrap_or(0.0)
                );
                println!();
            }

            positions
        }
        Err(e) => {
            warn!("⚠️  Failed to get future positions: {}", e);
            Vec::new()
        }
    };

    // =================================================================
    // 4. ANALYZE SPECIFIC INSTRUMENT FROM POSITIONS
    // =================================================================
    info!("🔍 4. ANALYZE SPECIFIC INSTRUMENT");
    info!("----------------------------------");

    // Find the first position to analyze in detail
    if let Some(first_position) = all_positions.first() {
        let instrument_name = &first_position.instrument_name;
        info!("🎯 Analyzing position for: {}", instrument_name);

        // Since there's no single get_position endpoint in the current implementation,
        // we'll filter from the positions we already have
        let specific_position = all_positions
            .iter()
            .find(|p| p.instrument_name == *instrument_name)
            .unwrap(); // We know it exists since we got it from all_positions

        info!("📊 DETAILED POSITION ANALYSIS");
        info!("------------------------------");
        info!("🏷️  Instrument: {}", specific_position.instrument_name);
        info!("💰 Position Size: {:.6}", specific_position.size);
        info!("📈 Direction: {:?}", specific_position.direction);
        info!(
            "💵 Mark Price: ${:.2}",
            specific_position.mark_price.unwrap_or(0.0)
        );

        let avg_price = specific_position.average_price;
        info!("📊 Average Price: ${:.2}", avg_price);
        if let Some(mark_price) = specific_position.mark_price {
            let price_diff = mark_price - avg_price;
            let price_diff_pct = (price_diff / avg_price) * 100.0;
            info!(
                "📈 Price Difference: ${:.2} ({:.2}%)",
                price_diff, price_diff_pct
            );
        }

        if let Some(unrealized_pnl) = specific_position.unrealized_profit_loss {
            info!("💸 Unrealized PnL: ${:.2}", unrealized_pnl);
            if unrealized_pnl > 0.0 {
                info!("✅ Position is profitable");
            } else if unrealized_pnl < 0.0 {
                info!("❌ Position is at a loss");
            } else {
                info!("➖ Position is break-even");
            }
        }

        if let Some(realized_pnl) = specific_position.realized_profit_loss {
            info!("💰 Realized PnL: ${:.2}", realized_pnl);
        }

        if let Some(total_pnl) = specific_position.total_profit_loss {
            info!("🎯 Total PnL: ${:.2}", total_pnl);
        }

        if let Some(maintenance_margin) = specific_position.maintenance_margin {
            info!("🛡️  Maintenance Margin: ${:.2}", maintenance_margin);
        }

        if let Some(initial_margin) = specific_position.initial_margin {
            info!("🏦 Initial Margin: ${:.2}", initial_margin);
        }

        println!();
    } else {
        warn!("⚠️  No positions available for detailed analysis");
    }

    // =================================================================
    // 5. PORTFOLIO SUMMARY
    // =================================================================
    info!("📈 5. PORTFOLIO SUMMARY");
    info!("------------------------");

    let total_unrealized_pnl: f64 = all_positions
        .iter()
        .map(|p| p.unrealized_profit_loss.unwrap_or(0.0))
        .sum();

    let total_realized_pnl: f64 = all_positions
        .iter()
        .map(|p| p.realized_profit_loss.unwrap_or(0.0))
        .sum();

    let profitable_positions = all_positions
        .iter()
        .filter(|p| p.unrealized_profit_loss.unwrap_or(0.0) > 0.0)
        .count();

    let losing_positions = all_positions
        .iter()
        .filter(|p| p.unrealized_profit_loss.unwrap_or(0.0) < 0.0)
        .count();

    info!("📊 Total Positions: {}", all_positions.len());
    info!("✅ Profitable Positions: {}", profitable_positions);
    info!("❌ Losing Positions: {}", losing_positions);
    info!("💸 Total Unrealized PnL: ${:.2}", total_unrealized_pnl);
    info!("💰 Total Realized PnL: ${:.2}", total_realized_pnl);
    info!(
        "🎯 Combined PnL: ${:.2}",
        total_unrealized_pnl + total_realized_pnl
    );

    // Group positions by currency
    let mut btc_count = 0;
    let mut eth_count = 0;
    let mut other_count = 0;

    for position in &all_positions {
        if position.instrument_name.starts_with("BTC") {
            btc_count += 1;
        } else if position.instrument_name.starts_with("ETH") {
            eth_count += 1;
        } else {
            other_count += 1;
        }
    }

    info!("₿ BTC Positions: {}", btc_count);
    info!("Ξ ETH Positions: {}", eth_count);
    info!("🔗 Other Positions: {}", other_count);

    println!();
    info!("🎉 Accounting endpoints example completed successfully!");
    info!("======================================================");

    Ok(())
}
