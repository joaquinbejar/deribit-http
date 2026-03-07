//! Combo Instruments Endpoints Example
//!
//! This example demonstrates the combo (spread) endpoints added in v0.6.0:
//! - `/private/get_combo_ids` - Get available combo IDs
//! - `/private/get_combo_details` - Get combo instrument details
//! - `/private/create_combo` - Create a new combo instrument
//! - `/private/verify_combo` - Verify combo parameters
//! - `/private/execute_combo` - Execute a combo trade
//!
//! Combos are spread instruments combining multiple legs
//! (e.g., calendar spreads, straddles).
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true"
//!
//! Then run: cargo run --bin combo_endpoints

use deribit_http::prelude::setup_logger;
use deribit_http::{DeribitHttpClient, HttpError};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    setup_logger();

    let client = DeribitHttpClient::new();

    info!("🔗 Combo Instruments Endpoints Example");
    info!("=======================================");
    info!("⚠️  Running on testnet for safety");
    info!("ℹ️  Combos: Multi-leg spread instruments");
    println!();

    // =================================================================
    // 1. GET COMBO IDS
    // =================================================================
    info!("📋 1. GET COMBO IDS");
    info!("--------------------");

    match client.get_combo_ids("BTC", None).await {
        Ok(combo_ids) => {
            info!("✅ Retrieved BTC combo IDs");
            info!("   📊 Total combos: {}", combo_ids.len());

            for (i, combo_id) in combo_ids.iter().take(10).enumerate() {
                info!("   🔗 Combo {}: {}", i + 1, combo_id);
            }

            if combo_ids.len() > 10 {
                info!("   ... and {} more combos", combo_ids.len() - 10);
            }
        }
        Err(e) => {
            warn!("⚠️  Could not get combo IDs: {}", e);
            info!("   Combos may not be available for this currency");
        }
    }
    println!();

    // =================================================================
    // 2. GET ETH COMBO IDS
    // =================================================================
    info!("📋 2. GET ETH COMBO IDS");
    info!("------------------------");

    match client.get_combo_ids("ETH", None).await {
        Ok(combo_ids) => {
            info!("✅ Retrieved ETH combo IDs");
            info!("   📊 Total combos: {}", combo_ids.len());
        }
        Err(e) => {
            warn!("⚠️  Could not get ETH combo IDs: {}", e);
        }
    }
    println!();

    // =================================================================
    // 3. GET COMBO DETAILS (DEMONSTRATION)
    // =================================================================
    info!("🔍 3. GET COMBO DETAILS");
    info!("------------------------");
    info!("⚠️  Skipping - requires valid combo ID");
    info!("   Example usage:");
    info!("   let details = client.get_combo_details(\"combo_id\").await?;");
    info!("   println!(\"Legs: {{:?}}\", details.legs);");
    info!("   println!(\"State: {{}}\", details.state);");
    println!();

    // =================================================================
    // 4. CREATE COMBO (DEMONSTRATION)
    // =================================================================
    info!("➕ 4. CREATE COMBO");
    info!("------------------");
    info!("⚠️  Skipping - creates new combo instrument");
    info!("   Example usage:");
    info!("   use deribit_http::model::combo::*;");
    info!("   ");
    info!("   // Calendar spread example:");
    info!("   let legs = vec![");
    info!("       ComboLeg {{");
    info!("           instrument_name: \"BTC-25DEC25\".to_string(),");
    info!("           direction: \"buy\".to_string(),");
    info!("           amount: 1.0,");
    info!("       }},");
    info!("       ComboLeg {{");
    info!("           instrument_name: \"BTC-28MAR26\".to_string(),");
    info!("           direction: \"sell\".to_string(),");
    info!("           amount: 1.0,");
    info!("       }},");
    info!("   ];");
    info!("   let combo = client.create_combo(&legs).await?;");
    println!();

    // =================================================================
    // 5. VERIFY COMBO (DEMONSTRATION)
    // =================================================================
    info!("✅ 5. VERIFY COMBO");
    info!("------------------");
    info!("⚠️  Skipping - requires combo parameters");
    info!("   Example usage:");
    info!("   let is_valid = client.verify_combo(&legs).await?;");
    info!("   if is_valid {{");
    info!("       println!(\"Combo parameters are valid\");");
    info!("   }}");
    println!();

    // =================================================================
    // 6. EXECUTE COMBO (DEMONSTRATION)
    // =================================================================
    info!("⚡ 6. EXECUTE COMBO");
    info!("-------------------");
    info!("⚠️  Skipping - executes real trade!");
    info!("   Example usage:");
    info!("   let order = client.execute_combo(");
    info!("       \"combo_id\",");
    info!("       \"buy\",     // Direction");
    info!("       1.0,        // Amount");
    info!("       Some(100.0) // Price (optional for market)");
    info!("   ).await?;");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📊 SUMMARY");
    info!("==========");
    info!("✅ Demonstrated combo instrument endpoints");
    info!("   - Retrieved available combos");
    info!("   - Showed create/verify/execute patterns");
    info!("");
    info!("ℹ️  Combo Types:");
    info!("   - Calendar Spreads: Same strike, different expiry");
    info!("   - Straddles: Call + Put, same strike/expiry");
    info!("   - Strangles: Call + Put, different strikes");
    info!("   - Custom: Any valid leg combination");
    info!("");
    info!("💡 Benefits of Combos:");
    info!("   - Single execution for multi-leg strategies");
    info!("   - Reduced margin requirements");
    info!("   - Better price discovery");

    Ok(())
}
