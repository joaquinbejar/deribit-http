//! Block Trade Endpoints Example
//!
//! This example demonstrates the block trade endpoints added in v0.6.0:
//! - `/private/get_block_trades` - Get block trades
//! - `/private/get_block_trade` - Get specific block trade
//! - `/private/execute_block_trade` - Execute a block trade
//! - `/private/verify_block_trade` - Verify block trade signature
//! - `/private/invalidate_block_trade_signature` - Invalidate signature
//!
//! Block trades are large, privately negotiated trades executed
//! outside the public order book.
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true"
//!
//! Then run: cargo run --bin block_trade_endpoints

use deribit_http::model::block_trade::GetBlockTradesRequest;
use deribit_http::prelude::setup_logger;
use deribit_http::{DeribitHttpClient, HttpError};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    setup_logger();

    let client = DeribitHttpClient::new();

    info!("📦 Block Trade Endpoints Example");
    info!("=================================");
    info!("⚠️  Running on testnet for safety");
    info!("ℹ️  Block trades: Large privately negotiated trades");
    println!();

    // =================================================================
    // 1. GET BLOCK TRADES (BTC)
    // =================================================================
    info!("📋 1. GET BTC BLOCK TRADES");
    info!("---------------------------");

    let request = GetBlockTradesRequest {
        currency: Some("BTC".to_string()),
        ..Default::default()
    };

    match client.get_block_trades(&request).await {
        Ok(block_trades) => {
            info!("✅ Retrieved BTC block trades");
            info!("   📊 Total block trades: {}", block_trades.len());

            for (i, block_trade) in block_trades.iter().take(5).enumerate() {
                info!("   📦 Block Trade {}:", i + 1);
                info!("      🆔 ID: {}", block_trade.id);
                info!("      � Timestamp: {}", block_trade.timestamp);
                info!("      � Legs: {}", block_trade.trades.len());
                for leg in block_trade.trades.iter().take(3) {
                    info!(
                        "         � {}: {} @ {}",
                        leg.instrument_name, leg.amount, leg.price
                    );
                }
                println!();
            }

            if block_trades.len() > 5 {
                info!("   ... and {} more block trades", block_trades.len() - 5);
            }
        }
        Err(e) => {
            warn!("⚠️  Could not get block trades: {}", e);
            info!("   Block trades may not be available or none exist");
        }
    }
    println!();

    // =================================================================
    // 2. GET RECENT BLOCK TRADES
    // =================================================================
    info!("📋 2. GET RECENT BLOCK TRADES");
    info!("------------------------------");

    let recent_request = GetBlockTradesRequest {
        count: Some(10),
        ..Default::default()
    };

    match client.get_block_trades(&recent_request).await {
        Ok(block_trades) => {
            info!("✅ Retrieved recent block trades");
            info!("   📊 Total: {}", block_trades.len());
        }
        Err(e) => {
            warn!("⚠️  Could not get recent block trades: {}", e);
        }
    }
    println!();

    // =================================================================
    // 3. GET SPECIFIC BLOCK TRADE (DEMONSTRATION)
    // =================================================================
    info!("🔍 3. GET SPECIFIC BLOCK TRADE");
    info!("-------------------------------");
    info!("⚠️  Skipping - requires valid block trade ID");
    info!("   Example usage:");
    info!("   let trade = client.get_block_trade(\"block_trade_id\").await?;");
    info!("   println!(\"Trade: {{:?}}\", trade);");
    println!();

    // =================================================================
    // 4. EXECUTE BLOCK TRADE (DEMONSTRATION)
    // =================================================================
    info!("⚡ 4. EXECUTE BLOCK TRADE");
    info!("-------------------------");
    info!("⚠️  Skipping - executes real trade!");
    info!("   Block trades require:");
    info!("   1. Pre-negotiated counterparty");
    info!("   2. Matching trade parameters");
    info!("   3. Valid signatures from both parties");
    info!("   ");
    info!("   Example usage:");
    info!("   use deribit_http::model::block_trade::*;");
    info!("   ");
    info!("   let trade = BlockTradeRequest {{");
    info!("       instrument_name: \"BTC-PERPETUAL\".to_string(),");
    info!("       direction: \"buy\".to_string(),");
    info!("       amount: 10000.0,");
    info!("       price: 50000.0,");
    info!("       ..Default::default()");
    info!("   }};");
    info!(
        "   let result = client.execute_block_trade(&[trade], \"role\", \"counterparty\").await?;"
    );
    println!();

    // =================================================================
    // 5. VERIFY BLOCK TRADE (DEMONSTRATION)
    // =================================================================
    info!("✅ 5. VERIFY BLOCK TRADE SIGNATURE");
    info!("-----------------------------------");
    info!("⚠️  Skipping - requires valid signature");
    info!("   Example usage:");
    info!("   let is_valid = client.verify_block_trade(");
    info!("       \"signature\",");
    info!("       \"timestamp\",");
    info!("       \"nonce\"");
    info!("   ).await?;");
    println!();

    // =================================================================
    // 6. INVALIDATE BLOCK TRADE SIGNATURE (DEMONSTRATION)
    // =================================================================
    info!("❌ 6. INVALIDATE BLOCK TRADE SIGNATURE");
    info!("--------------------------------------");
    info!("⚠️  Skipping - invalidates existing signature");
    info!("   Example usage:");
    info!("   client.invalidate_block_trade_signature(\"signature\").await?;");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📊 SUMMARY");
    info!("==========");
    info!("✅ Demonstrated block trade endpoints");
    info!("   - Retrieved recent block trades");
    info!("   - Showed execution patterns");
    info!("");
    info!("ℹ️  Block Trade Characteristics:");
    info!("   - Minimum size requirements apply");
    info!("   - Executed outside public order book");
    info!("   - Require counterparty agreement");
    info!("   - Used for large institutional trades");
    info!("");
    info!("🔐 API Key Requirements:");
    info!("   - trade:read - for viewing block trades");
    info!("   - trade:read_write - for executing trades");
    info!("   - block_trade scope may be required");

    Ok(())
}
