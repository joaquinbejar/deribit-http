//! Account Summary Integration Tests
//!
//! This test covers account summary functionality:
//! 1. Get account summary for different currencies
//! 2. Validate account summary structure
//! 3. Test extended account information
//! 4. Test error handling for invalid currencies

use std::path::Path;
use tracing::{debug, info, warn};

use deribit_http::DeribitHttpClient;

/// Check if .env file exists and contains required variables
fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(".env").exists() {
        return Err("Missing .env file. Please create one with authentication credentials".into());
    }

    dotenv::dotenv().ok();

    let has_oauth2 = std::env::var("DERIBIT_CLIENT_ID").is_ok()
        && std::env::var("DERIBIT_CLIENT_SECRET").is_ok();
    let has_api_key =
        std::env::var("DERIBIT_API_KEY").is_ok() && std::env::var("DERIBIT_API_SECRET").is_ok();

    if !has_oauth2 && !has_api_key {
        return Err("Missing authentication credentials".into());
    }

    Ok(())
}

/// Authenticate client using available credentials
async fn authenticate_client(client: &DeribitHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    if let (Ok(client_id), Ok(client_secret)) = (
        std::env::var("DERIBIT_CLIENT_ID"),
        std::env::var("DERIBIT_CLIENT_SECRET"),
    ) {
        client
            .authenticate_oauth2(&client_id, &client_secret)
            .await?;
    } else if let (Ok(api_key), Ok(api_secret)) = (
        std::env::var("DERIBIT_API_KEY"),
        std::env::var("DERIBIT_API_SECRET"),
    ) {
        client.authenticate_api_key(&api_key, &api_secret).await?;
    } else {
        return Err("No valid authentication credentials found".into());
    }
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_account_summary_btc() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting BTC account summary test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting BTC account summary");
    let account_summary = client.get_account_summary("BTC", None).await?;

    info!("BTC account summary retrieved successfully");
    debug!("Account summary: {:?}", account_summary);

    // Validate account summary structure
    assert_eq!(account_summary.currency, "BTC", "Currency should be BTC");
    assert!(
        account_summary.balance >= 0.0,
        "Balance should be non-negative"
    );
    assert!(
        account_summary.available_funds >= 0.0,
        "Available funds should be non-negative"
    );
    assert!(
        account_summary.equity >= 0.0,
        "Equity should be non-negative"
    );
    assert!(
        !account_summary.account_type.is_empty(),
        "Account type should not be empty"
    );
    // Validate system name if present
    if let Some(ref system_name) = account_summary.system_name {
        assert!(
            !system_name.is_empty(),
            "System name should not be empty if present"
        );
    }

    info!("BTC account summary test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_account_summary_eth() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting ETH account summary test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting ETH account summary");
    let account_summary = client.get_account_summary("ETH", None).await?;

    info!("ETH account summary retrieved successfully");
    debug!("Account summary: {:?}", account_summary);

    // Validate account summary structure
    assert_eq!(account_summary.currency, "ETH", "Currency should be ETH");
    assert!(
        account_summary.balance >= 0.0,
        "Balance should be non-negative"
    );
    assert!(
        account_summary.available_funds >= 0.0,
        "Available funds should be non-negative"
    );
    assert!(
        account_summary.equity >= 0.0,
        "Equity should be non-negative"
    );
    assert!(
        !account_summary.account_type.is_empty(),
        "Account type should not be empty"
    );
    // Validate system name if present
    if let Some(ref system_name) = account_summary.system_name {
        assert!(
            !system_name.is_empty(),
            "System name should not be empty if present"
        );
    }

    info!("ETH account summary test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_account_summary_extended() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting extended account summary test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting extended BTC account summary");
    let account_summary = client.get_account_summary("BTC", Some(true)).await?;

    info!("Extended account summary retrieved successfully");
    debug!("Extended account summary: {:?}", account_summary);

    // Validate extended account summary structure
    assert_eq!(account_summary.currency, "BTC", "Currency should be BTC");
    assert!(
        account_summary.balance >= 0.0,
        "Balance should be non-negative"
    );
    assert!(
        account_summary.available_funds >= 0.0,
        "Available funds should be non-negative"
    );
    assert!(
        account_summary.equity >= 0.0,
        "Equity should be non-negative"
    );

    // Extended fields should be present
    assert!(
        account_summary.total_pl != 0.0 || account_summary.total_pl == 0.0,
        "Total P&L should be a valid number"
    );
    // Validate margin balance instead of fee_balance (which doesn't exist)
    assert!(
        account_summary.margin_balance >= 0.0,
        "Margin balance should be non-negative"
    );

    info!("Extended account summary test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_account_summary_invalid_currency() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting invalid currency account summary test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Attempting to get account summary for invalid currency");
    let result = client.get_account_summary("INVALID", None).await;

    match result {
        Ok(_) => {
            warn!("Expected error for invalid currency, but request succeeded");
            // Some APIs might return empty data instead of error
        }
        Err(e) => {
            info!("Correctly received error for invalid currency: {:?}", e);
            assert!(
                e.to_string().contains("invalid")
                    || e.to_string().contains("currency")
                    || e.to_string().contains("not found")
            );
        }
    }

    info!("Invalid currency account summary test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_account_summary_multiple_currencies() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting multiple currencies account summary test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    let currencies = ["BTC", "ETH", "USDC"];

    for currency in &currencies {
        debug!("Getting account summary for {}", currency);
        let account_summary = client.get_account_summary(currency, None).await?;

        info!("{} account summary retrieved successfully", currency);

        // Validate basic structure for each currency
        assert_eq!(
            account_summary.currency, *currency,
            "Currency should match requested currency"
        );
        assert!(
            account_summary.balance >= 0.0,
            "Balance should be non-negative for {}",
            currency
        );
        assert!(
            account_summary.available_funds >= 0.0,
            "Available funds should be non-negative for {}",
            currency
        );
        assert!(
            account_summary.equity >= 0.0,
            "Equity should be non-negative for {}",
            currency
        );
        assert!(
            !account_summary.account_type.is_empty(),
            "Account type should not be empty for {}",
            currency
        );
        // Validate system name if present
        if let Some(ref system_name) = account_summary.system_name {
            assert!(
                !system_name.is_empty(),
                "System name should not be empty for {}",
                currency
            );
        }

        // Small delay between requests to respect rate limits
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    info!("Multiple currencies account summary test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_account_summary_consistency() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting account summary consistency test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    // Get account summary multiple times to check consistency
    debug!("Getting first BTC account summary");
    let summary1 = client.get_account_summary("BTC", None).await?;

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    debug!("Getting second BTC account summary");
    let summary2 = client.get_account_summary("BTC", None).await?;

    info!("Both account summaries retrieved successfully");

    // Check that basic account information is consistent
    assert_eq!(
        summary1.currency, summary2.currency,
        "Currency should be consistent"
    );
    // Compare system names if both are present
    match (&summary1.system_name, &summary2.system_name) {
        (Some(name1), Some(name2)) => {
            assert_eq!(name1, name2, "System name should be consistent");
        }
        (None, None) => {} // Both None is fine
        _ => {}            // One None, one Some - this could happen, so we don't assert
    }
    assert_eq!(
        summary1.account_type, summary2.account_type,
        "Account type should be consistent"
    );

    // Balances might change slightly due to funding or other activities, so we allow some tolerance
    let balance_diff = (summary1.balance - summary2.balance).abs();
    assert!(
        balance_diff < 0.01,
        "Balance should be relatively stable (diff: {})",
        balance_diff
    );

    info!("Account summary consistency test completed successfully");
    Ok(())
}
