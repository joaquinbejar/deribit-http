//! Address Beneficiary Endpoints Example
//!
//! This example demonstrates the address beneficiary endpoints added in v0.6.0:
//! - `/private/list_address_beneficiaries` - List all beneficiaries
//! - `/private/get_address_beneficiary` - Get specific beneficiary
//! - `/private/save_address_beneficiary` - Create/update beneficiary
//! - `/private/delete_address_beneficiary` - Remove beneficiary
//!
//! These endpoints are used for Travel Rule compliance, storing
//! beneficiary information for withdrawal addresses.
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true"
//!
//! Then run: cargo run --bin beneficiary_endpoints

use deribit_http::model::beneficiary::ListAddressBeneficiariesRequest;
use deribit_http::prelude::setup_logger;
use deribit_http::{DeribitHttpClient, HttpError};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    setup_logger();

    let client = DeribitHttpClient::new();

    info!("📋 Address Beneficiary Endpoints Example");
    info!("=========================================");
    info!("⚠️  Running on testnet for safety");
    info!("ℹ️  Travel Rule: Beneficiary info for withdrawals");
    println!();

    // =================================================================
    // 1. LIST ADDRESS BENEFICIARIES
    // =================================================================
    info!("📋 1. LIST ADDRESS BENEFICIARIES");
    info!("---------------------------------");

    let request = ListAddressBeneficiariesRequest {
        currency: Some("BTC".to_string()),
        limit: Some(10),
        ..Default::default()
    };

    match client.list_address_beneficiaries(Some(&request)).await {
        Ok(response) => {
            info!("✅ Retrieved beneficiaries");
            info!("   📊 Total count: {:?}", response.count);
            info!("   📋 Retrieved: {}", response.data.len());

            for (i, beneficiary) in response.data.iter().take(5).enumerate() {
                info!("   👤 Beneficiary {}:", i + 1);
                info!("      📍 Address: {}", beneficiary.address);
                info!("      💱 Currency: {}", beneficiary.currency);
                info!("      ✅ Agreed: {}", beneficiary.agreed);
                info!("      🏠 Personal: {}", beneficiary.personal);
                if let Some(ref name) = beneficiary.beneficiary_first_name {
                    info!("      👤 First name: {}", name);
                }
                if let Some(ref name) = beneficiary.beneficiary_last_name {
                    info!("      👤 Last name: {}", name);
                }
                if let Some(ref vasp) = beneficiary.beneficiary_vasp_name {
                    info!("      🏦 VASP: {}", vasp);
                }
                println!();
            }

            if response.data.len() > 5 {
                info!("   ... and {} more beneficiaries", response.data.len() - 5);
            }
        }
        Err(e) => {
            warn!("⚠️  Could not list beneficiaries: {}", e);
            info!("   This may be normal if no beneficiaries exist");
        }
    }
    println!();

    // =================================================================
    // 2. LIST ALL BENEFICIARIES (NO FILTER)
    // =================================================================
    info!("📋 2. LIST ALL BENEFICIARIES");
    info!("-----------------------------");

    match client.list_address_beneficiaries(None).await {
        Ok(response) => {
            info!("✅ Retrieved all beneficiaries");
            info!("   📊 Total across all currencies: {:?}", response.count);
        }
        Err(e) => {
            warn!("⚠️  Could not list all beneficiaries: {}", e);
        }
    }
    println!();

    // =================================================================
    // 3. GET SPECIFIC BENEFICIARY (DEMONSTRATION)
    // =================================================================
    info!("🔍 3. GET SPECIFIC BENEFICIARY");
    info!("-------------------------------");
    info!("⚠️  Skipping - requires existing beneficiary address");
    info!("   Example usage:");
    info!("   let beneficiary = client.get_address_beneficiary(");
    info!("       \"BTC\",");
    info!("       \"bc1q...\",  // Address");
    info!("       None        // Optional tag for XRP");
    info!("   ).await?;");
    println!();

    // =================================================================
    // 4. SAVE ADDRESS BENEFICIARY (DEMONSTRATION)
    // =================================================================
    info!("💾 4. SAVE ADDRESS BENEFICIARY");
    info!("-------------------------------");
    info!("⚠️  Skipping - creates/updates beneficiary info");
    info!("   Example usage:");
    info!("   use deribit_http::model::SaveAddressBeneficiaryRequest;");
    info!("   ");
    info!("   let request = SaveAddressBeneficiaryRequest {{");
    info!("       currency: \"BTC\".to_string(),");
    info!("       address: \"bc1q...\".to_string(),");
    info!("       agreed: true,");
    info!("       personal: false,  // false = hosted wallet");
    info!("       unhosted: false,");
    info!("       beneficiary_vasp_name: \"Exchange Name\".to_string(),");
    info!("       beneficiary_vasp_did: \"did:web:exchange.com\".to_string(),");
    info!("       beneficiary_address: \"123 Main St, City\".to_string(),");
    info!("       beneficiary_first_name: Some(\"John\".to_string()),");
    info!("       beneficiary_last_name: Some(\"Doe\".to_string()),");
    info!("       ..Default::default()");
    info!("   }};");
    info!("   let saved = client.save_address_beneficiary(&request).await?;");
    println!();

    // =================================================================
    // 5. DELETE ADDRESS BENEFICIARY (DEMONSTRATION)
    // =================================================================
    info!("🗑️  5. DELETE ADDRESS BENEFICIARY");
    info!("---------------------------------");
    info!("⚠️  Skipping - permanently removes beneficiary info");
    info!("   Example usage:");
    info!("   client.delete_address_beneficiary(");
    info!("       \"BTC\",");
    info!("       \"bc1q...\",");
    info!("       None  // Optional tag for XRP");
    info!("   ).await?;");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📊 SUMMARY");
    info!("==========");
    info!("✅ Demonstrated address beneficiary endpoints");
    info!("   - Listed beneficiaries with filtering");
    info!("   - Showed get/save/delete patterns");
    info!("");
    info!("ℹ️  Travel Rule Compliance:");
    info!("   - Required for withdrawals above certain thresholds");
    info!("   - Stores recipient identity information");
    info!("   - VASP = Virtual Asset Service Provider");
    info!("");
    info!("📋 Beneficiary Fields:");
    info!("   - personal: true if self-hosted wallet");
    info!("   - agreed: consent to share info");
    info!("   - beneficiary_vasp_*: Exchange/VASP details");
    info!("   - beneficiary_first/last_name: Individual recipient");
    info!("   - beneficiary_company_name: Business recipient");

    Ok(())
}
