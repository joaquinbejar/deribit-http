//! Position Management Endpoints Example
//!
//! This example demonstrates the position management endpoints added in v0.6.0:
//! - `/private/get_position` - Get specific position
//! - `/private/get_positions` - Get all positions
//! - `/private/move_positions` - Move positions between accounts
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true"
//!
//! Then run: cargo run --bin position_endpoints

use deribit_http::prelude::setup_logger;
use deribit_http::{DeribitHttpClient, HttpError};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    setup_logger();

    let client = DeribitHttpClient::new();

    info!("📈 Position Management Endpoints Example");
    info!("=========================================");
    info!("⚠️  Running on testnet for safety");
    println!();

    // =================================================================
    // 1. GET ALL POSITIONS
    // =================================================================
    info!("📊 1. GET ALL POSITIONS");
    info!("------------------------");

    let positions = match client.get_positions(None, None, None).await {
        Ok(pos) => {
            info!("✅ Retrieved all positions");
            info!("   📈 Total positions: {}", pos.len());

            if pos.is_empty() {
                warn!("⚠️  No open positions found");
                info!("   You need open positions to see detailed output");
            }

            for (i, position) in pos.iter().take(5).enumerate() {
                info!("   📋 Position {}:", i + 1);
                info!("      📈 Instrument: {}", position.instrument_name);
                info!("      💰 Size: {}", position.size);
                info!("      📊 Direction: {:?}", position.direction);
                info!("      💵 Average Price: ${:.2}", position.average_price);
                if let Some(mark) = position.mark_price {
                    info!("      📍 Mark Price: ${:.2}", mark);
                }
                if let Some(pnl) = position.unrealized_profit_loss {
                    info!("      💸 Unrealized PnL: ${:.2}", pnl);
                }
                if let Some(leverage) = position.leverage {
                    info!("      ⚖️  Leverage: {:.1}x", leverage);
                }
                println!();
            }

            if pos.len() > 5 {
                info!("   ... and {} more positions", pos.len() - 5);
            }

            pos
        }
        Err(e) => {
            error!("❌ Failed to get positions: {}", e);
            return Err(e);
        }
    };
    println!();

    // =================================================================
    // 2. GET POSITIONS BY CURRENCY
    // =================================================================
    info!("₿ 2. GET BTC POSITIONS");
    info!("-----------------------");

    match client.get_positions(Some("BTC"), None, None).await {
        Ok(btc_positions) => {
            info!("✅ Retrieved BTC positions");
            info!("   📈 BTC positions: {}", btc_positions.len());

            for pos in btc_positions.iter() {
                info!("      {} - Size: {}", pos.instrument_name, pos.size);
            }
        }
        Err(e) => {
            warn!("⚠️  Could not get BTC positions: {}", e);
        }
    }
    println!();

    // =================================================================
    // 3. GET POSITIONS BY KIND
    // =================================================================
    info!("📊 3. GET OPTION POSITIONS");
    info!("--------------------------");

    match client.get_positions(None, Some("option"), None).await {
        Ok(option_positions) => {
            info!("✅ Retrieved option positions");
            info!("   📈 Option positions: {}", option_positions.len());

            for pos in option_positions.iter().take(3) {
                info!("      {} - Size: {}", pos.instrument_name, pos.size);
            }
        }
        Err(e) => {
            warn!("⚠️  Could not get option positions: {}", e);
        }
    }
    println!();

    // =================================================================
    // 4. GET SPECIFIC POSITION
    // =================================================================
    info!("🔍 4. GET SPECIFIC POSITION");
    info!("----------------------------");

    if let Some(first_pos) = positions.first() {
        match client.get_position(&first_pos.instrument_name).await {
            Ok(pos_list) => {
                info!("✅ Retrieved specific position(s)");
                for position in pos_list.iter() {
                    info!("   📈 Instrument: {}", position.instrument_name);
                    info!("   💰 Size: {}", position.size);
                    info!("   📊 Direction: {:?}", position.direction);
                    info!("   💵 Average Price: ${:.2}", position.average_price);
                    if let Some(initial_margin) = position.initial_margin {
                        info!("   🔒 Initial Margin: ${:.2}", initial_margin);
                    }
                    if let Some(maint_margin) = position.maintenance_margin {
                        info!("   🔒 Maintenance Margin: ${:.2}", maint_margin);
                    }
                }
            }
            Err(e) => {
                warn!("⚠️  Could not get specific position: {}", e);
            }
        }
    } else {
        info!("   No positions available to query");
        info!("   Example usage:");
        info!("   let positions = client.get_position(\"BTC-PERPETUAL\").await?;");
    }
    println!();

    // =================================================================
    // 5. MOVE POSITIONS (DEMONSTRATION)
    // =================================================================
    info!("🔄 5. MOVE POSITIONS");
    info!("--------------------");
    info!("⚠️  Skipping - transfers positions between accounts!");
    info!("   Move positions allows transferring positions to subaccounts");
    info!("   ");
    info!("   Example usage:");
    info!("   use deribit_http::model::request::position::*;");
    info!("   ");
    info!("   let trades = vec![");
    info!("       MovePositionTrade::new(");
    info!("           \"BTC-PERPETUAL\".to_string(),");
    info!("           1000.0,  // Amount to move");
    info!("       ),");
    info!("   ];");
    info!("   let request = MovePositionsRequest::new(");
    info!("       \"BTC\".to_string(),");
    info!("       12345,  // Destination subaccount ID");
    info!("       trades,");
    info!("   );");
    info!("   let result = client.move_positions(&request).await?;");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📊 SUMMARY");
    info!("==========");
    info!("✅ Demonstrated position management endpoints");
    info!("   - Retrieved {} total positions", positions.len());
    info!("   - Showed filtering by currency/kind");
    info!("   - Demonstrated move positions pattern");
    info!("");
    info!("ℹ️  Position Information:");
    info!("   - size: Position size (+ long, - short)");
    info!("   - direction: buy/sell");
    info!("   - average_price: Entry price");
    info!("   - mark_price: Current mark price");
    info!("   - unrealized_profit_loss: Current PnL");
    info!("");
    info!("🔐 API Key Requirements:");
    info!("   - account:read - for viewing positions");
    info!("   - trade:read_write - for moving positions");

    Ok(())
}
