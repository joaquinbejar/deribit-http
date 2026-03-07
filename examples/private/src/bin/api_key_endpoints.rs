//! API Key Management Endpoints Example
//!
//! This example demonstrates the API key management endpoints added in v0.6.0:
//! - `/private/list_api_keys` - List all API keys
//! - `/private/get_api_key` - Get specific API key details
//! - `/private/create_api_key` - Create a new API key
//! - `/private/edit_api_key` - Edit an existing API key
//! - `/private/enable_api_key` - Enable a disabled API key
//! - `/private/disable_api_key` - Disable an API key
//! - `/private/reset_api_key` - Reset API key secret
//! - `/private/set_api_key_as_default` - Set default API key
//! - `/private/remove_api_key` - Delete an API key
//!
//! ⚠️ WARNING: API key operations can affect account access!
//!
//! Usage:
//! Set environment variables:
//! - DERIBIT_CLIENT_ID="your_client_id"
//! - DERIBIT_CLIENT_SECRET="your_client_secret"
//! - DERIBIT_TESTNET="true"
//!
//! Then run: cargo run --bin api_key_endpoints

use deribit_http::prelude::setup_logger;
use deribit_http::{DeribitHttpClient, HttpError};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    setup_logger();

    let client = DeribitHttpClient::new();

    info!("🔑 API Key Management Endpoints Example");
    info!("========================================");
    info!("⚠️  Running on testnet for safety");
    println!();

    // =================================================================
    // 1. LIST ALL API KEYS
    // =================================================================
    info!("📋 1. LIST ALL API KEYS");
    info!("------------------------");

    let api_keys = match client.list_api_keys().await {
        Ok(keys) => {
            info!("✅ Retrieved API keys successfully");
            info!("   🔑 Total API keys: {}", keys.len());

            for (i, key) in keys.iter().enumerate() {
                info!("   📋 API Key {}: {}", i + 1, key.id);
                info!("      📝 Name: {}", key.name);
                info!("      ✅ Enabled: {}", key.enabled);
                info!("      ⭐ Default: {}", key.default);
                info!(
                    "      🆔 Client ID: {}...",
                    &key.client_id[..8.min(key.client_id.len())]
                );
                if !key.max_scope.is_empty() {
                    info!("      🔐 Max Scope: {}", key.max_scope);
                }
                println!();
            }
            keys
        }
        Err(e) => {
            error!("❌ Failed to list API keys: {}", e);
            return Err(e);
        }
    };

    // =================================================================
    // 2. INSPECT FIRST API KEY (from list)
    // =================================================================
    info!("🔍 2. INSPECT FIRST API KEY");
    info!("----------------------------");

    if let Some(first_key) = api_keys.first() {
        info!("✅ First API key details (from list):");
        info!("   🆔 ID: {}", first_key.id);
        info!("   📝 Name: {}", first_key.name);
        info!("   ✅ Enabled: {}", first_key.enabled);
        info!("   ⭐ Default: {}", first_key.default);
        info!("   📅 Created: {}", first_key.timestamp);
        info!("   🔐 Max Scope: {}", first_key.max_scope);
    } else {
        info!("   No API keys available to inspect");
    }
    println!();

    // =================================================================
    // 3. CREATE API KEY (DEMONSTRATION)
    // =================================================================
    info!("➕ 3. CREATE API KEY");
    info!("--------------------");
    info!("⚠️  Skipping actual creation - security implications");
    info!("   Example usage:");
    info!("   use deribit_http::model::api_key::CreateApiKeyRequest;");
    info!("   ");
    info!("   let request = CreateApiKeyRequest {{");
    info!("       max_scope: \"account:read trade:read\".to_string(),");
    info!("       name: \"My Trading Bot\".to_string(),");
    info!("       default: false,");
    info!("   }};");
    info!("   let new_key = client.create_api_key(&request).await?;");
    info!("   // ⚠️  Save the client_secret - shown only once!");
    println!();

    // =================================================================
    // 4. EDIT API KEY (DEMONSTRATION)
    // =================================================================
    info!("✏️  4. EDIT API KEY");
    info!("-------------------");
    info!("⚠️  Skipping - affects existing key");
    info!("   Example usage:");
    info!("   use deribit_http::model::api_key::EditApiKeyRequest;");
    info!("   ");
    info!("   let request = EditApiKeyRequest {{");
    info!("       id: key_id,");
    info!("       max_scope: \"account:read\".to_string(),");
    info!("       name: \"Updated Name\".to_string(),");
    info!("       enabled: true,");
    info!("       ..Default::default()");
    info!("   }};");
    info!("   client.edit_api_key(&request).await?;");
    println!();

    // =================================================================
    // 5. ENABLE/DISABLE API KEY (DEMONSTRATION)
    // =================================================================
    info!("🔄 5. ENABLE/DISABLE API KEY");
    info!("-----------------------------");
    info!("⚠️  Skipping - affects key access");
    info!("   Example usage:");
    info!("   // Disable a key");
    info!("   client.disable_api_key(key_id).await?;");
    info!("   ");
    info!("   // Enable a key");
    info!("   client.enable_api_key(key_id).await?;");
    println!();

    // =================================================================
    // 6. RESET API KEY (DEMONSTRATION)
    // =================================================================
    info!("🔄 6. RESET API KEY SECRET");
    info!("--------------------------");
    info!("⚠️  Skipping - INVALIDATES current secret!");
    info!("   Example usage:");
    info!("   let new_key = client.reset_api_key(key_id).await?;");
    info!("   // ⚠️  Save new client_secret - old one is invalid!");
    println!();

    // =================================================================
    // 7. SET DEFAULT API KEY (DEMONSTRATION)
    // =================================================================
    info!("⭐ 7. SET DEFAULT API KEY");
    info!("-------------------------");
    info!("⚠️  Skipping - changes default key");
    info!("   Example usage:");
    info!("   client.set_api_key_as_default(key_id).await?;");
    println!();

    // =================================================================
    // 8. REMOVE API KEY (DEMONSTRATION)
    // =================================================================
    info!("🗑️  8. REMOVE API KEY");
    info!("---------------------");
    info!("⚠️  Skipping - PERMANENT deletion!");
    info!("   Example usage:");
    info!("   client.remove_api_key(key_id).await?;");
    info!("   // ⚠️  Cannot be undone!");
    println!();

    // =================================================================
    // SUMMARY
    // =================================================================
    info!("📊 SUMMARY");
    info!("==========");
    info!("✅ Demonstrated API key management endpoints");
    info!("   - Listed {} API keys", api_keys.len());
    info!("   - Showed create/edit/enable/disable patterns");
    info!("");
    info!("💡 Security Best Practices:");
    info!("   - Use minimal required scopes");
    info!("   - Rotate keys periodically");
    info!("   - Never share client_secret");
    info!("   - Disable unused keys");

    Ok(())
}
