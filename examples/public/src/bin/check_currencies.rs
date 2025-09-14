//! Simple currency checker
//! This example only shows supported currencies

use deribit_base::prelude::setup_logger;
use deribit_http::DeribitHttpClient;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();
    
    info!("🔍 Checking supported currencies on current environment");
    info!("====================================================");
    
    let client = DeribitHttpClient::new();
    
    match client.get_currencies().await {
        Ok(currencies) => {
            info!("✅ Successfully retrieved currencies");
            info!("📊 Total supported currencies: {}", currencies.len());
            info!("\n📋 Available currencies:");
            
            for currency in currencies {
                info!("   • {} - {}", currency.currency, currency.currency_long);
                if let Some(apr) = currency.apr {
                    info!("     📈 APR: {}% (yield token)", apr);
                }
            }
        }
        Err(e) => {
            error!("❌ Failed to get currencies: {}", e);
        }
    }
    
    Ok(())
}