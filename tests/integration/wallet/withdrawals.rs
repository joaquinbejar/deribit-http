//! Withdrawals Integration Tests
//!
//! This test covers withdrawals functionality:
//! 1. Get withdrawals for different currencies
//! 2. Test withdrawal pagination
//! 3. Test withdrawal filtering
//! 4. Validate withdrawal data structure

use std::path::Path;
use tracing::{debug, info};

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
async fn test_get_withdrawals_btc() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting BTC withdrawals test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting BTC withdrawals");
    let withdrawals_response = client.get_withdrawals("BTC", None, None).await?;

    info!(
        "BTC withdrawals retrieved successfully, count: {}, total: {}",
        withdrawals_response.data.len(),
        withdrawals_response.count
    );
    debug!("Withdrawals response: {:?}", withdrawals_response);

    // Validate withdrawals response structure
    assert!(
        withdrawals_response.count >= withdrawals_response.data.len() as u32,
        "Count should be >= data length: {} >= {}",
        withdrawals_response.count,
        withdrawals_response.data.len()
    );

    // Validate individual withdrawals
    for (i, withdrawal) in withdrawals_response.data.iter().enumerate() {
        debug!(
            "Validating withdrawal #{}: {:?}",
            i + 1,
            withdrawal.transaction_id
        );

        assert!(
            !withdrawal.address.is_empty(),
            "Withdrawal address should not be empty"
        );
        assert!(
            withdrawal.amount > 0.0,
            "Withdrawal amount should be positive"
        );
        assert_eq!(withdrawal.currency, "BTC", "Currency should be BTC");
        assert!(
            withdrawal.created_timestamp > 0,
            "Created timestamp should be positive"
        );
        assert!(!withdrawal.state.is_empty(), "State should not be empty");
        assert!(
            withdrawal.transaction_id.is_some(),
            "Transaction ID should be present"
        );
        if let Some(ref tx_id) = withdrawal.transaction_id {
            assert!(!tx_id.is_empty(), "Transaction ID should not be empty");
        }
        if let Some(updated_ts) = withdrawal.updated_timestamp {
            assert!(updated_ts > 0, "Updated timestamp should be positive");
        }
        assert!(withdrawal.fee >= 0.0, "Fee should be non-negative");
        assert!(withdrawal.id > 0, "Withdrawal ID should be positive");

        // Validate state values
        let valid_states = [
            "completed",
            "pending",
            "cancelled",
            "rejected",
            "confirming",
            "unconfirmed",
        ];
        assert!(
            valid_states.iter().any(|&s| withdrawal.state.contains(s)),
            "State should be valid: {}",
            withdrawal.state
        );

        // Validate timestamps order
        if let Some(updated_ts) = withdrawal.updated_timestamp {
            assert!(
                updated_ts >= withdrawal.created_timestamp,
                "Updated timestamp should be >= created timestamp: {:?} >= {}",
                withdrawal.updated_timestamp,
                withdrawal.created_timestamp
            );
        }

        // Validate priority (priority is a string, not a number)
        assert!(
            !withdrawal.priority.is_empty(),
            "Priority should not be empty"
        );
    }

    info!("BTC withdrawals test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_withdrawals_eth() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting ETH withdrawals test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting ETH withdrawals");
    let withdrawals_response = client.get_withdrawals("ETH", None, None).await?;

    info!(
        "ETH withdrawals retrieved successfully, count: {}, total: {}",
        withdrawals_response.data.len(),
        withdrawals_response.count
    );
    debug!("Withdrawals response: {:?}", withdrawals_response);

    // Validate that all withdrawals are ETH-related
    for withdrawal in &withdrawals_response.data {
        assert_eq!(
            withdrawal.currency, "ETH",
            "All withdrawals should be ETH-related"
        );
    }

    info!("ETH withdrawals test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_withdrawals_with_count() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting withdrawals with count test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    let requested_count = 5;
    debug!("Getting withdrawals with count: {}", requested_count);
    let withdrawals_response = client
        .get_withdrawals("BTC", Some(requested_count), None)
        .await?;

    info!(
        "Withdrawals with count retrieved successfully, count: {}, total: {}",
        withdrawals_response.data.len(),
        withdrawals_response.count
    );
    debug!("Withdrawals response: {:?}", withdrawals_response);

    // Validate that we got at most the requested count
    assert!(
        withdrawals_response.data.len() <= requested_count as usize,
        "Should not receive more than requested count: {} <= {}",
        withdrawals_response.data.len(),
        requested_count
    );

    info!("Withdrawals with count test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_withdrawals_with_offset() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting withdrawals with offset test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    // Get first page
    debug!("Getting first page of withdrawals");
    let first_page = client.get_withdrawals("BTC", Some(5), Some(0)).await?;

    info!(
        "First page retrieved successfully, count: {}",
        first_page.data.len()
    );

    if first_page.data.len() >= 5 {
        // Get second page with offset
        debug!("Getting second page with offset: 5");
        let second_page = client.get_withdrawals("BTC", Some(5), Some(5)).await?;

        info!(
            "Second page retrieved successfully, count: {}",
            second_page.data.len()
        );
        debug!("Second page: {:?}", second_page);

        // Validate that pages don't overlap
        for first_withdrawal in &first_page.data {
            for second_withdrawal in &second_page.data {
                assert_ne!(
                    first_withdrawal.transaction_id, second_withdrawal.transaction_id,
                    "Transaction IDs should not overlap between pages"
                );
                assert_ne!(
                    first_withdrawal.id, second_withdrawal.id,
                    "Withdrawal IDs should not overlap between pages"
                );
            }
        }
    } else {
        info!("Not enough withdrawals for offset test, skipping second page");
    }

    info!("Withdrawals with offset test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_withdrawals_data_validation() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting withdrawals data validation test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting withdrawals for data validation");
    let withdrawals_response = client.get_withdrawals("BTC", Some(20), None).await?;

    info!(
        "Withdrawals retrieved for validation, count: {}",
        withdrawals_response.data.len()
    );

    for withdrawal in &withdrawals_response.data {
        debug!(
            "Validating withdrawal: {:?} - {}",
            withdrawal.transaction_id, withdrawal.currency
        );

        // Validate required string fields
        assert!(
            !withdrawal.address.is_empty(),
            "Address should not be empty"
        );
        assert!(
            !withdrawal.currency.is_empty(),
            "Currency should not be empty"
        );
        assert!(!withdrawal.state.is_empty(), "State should not be empty");
        assert!(
            withdrawal.transaction_id.is_some(),
            "Transaction ID should be present"
        );
        if let Some(ref tx_id) = withdrawal.transaction_id {
            assert!(!tx_id.is_empty(), "Transaction ID should not be empty");
        }

        // Validate numeric fields
        assert!(withdrawal.amount > 0.0, "Amount should be positive");
        assert!(withdrawal.amount.is_finite(), "Amount should be finite");
        assert!(withdrawal.fee >= 0.0, "Fee should be non-negative");
        assert!(withdrawal.fee.is_finite(), "Fee should be finite");
        assert!(
            !withdrawal.priority.is_empty(),
            "Priority should not be empty"
        );
        assert!(withdrawal.id > 0, "Withdrawal ID should be positive");
        assert!(
            withdrawal.created_timestamp > 0,
            "Created timestamp should be positive"
        );
        if let Some(updated_ts) = withdrawal.updated_timestamp {
            assert!(updated_ts > 0, "Updated timestamp should be positive");
        }

        // Validate currency values
        let valid_currencies = ["BTC", "ETH", "USDC", "USDT", "EURR"];
        assert!(
            valid_currencies.contains(&withdrawal.currency.as_str()),
            "Currency should be valid: {}",
            withdrawal.currency
        );

        // Validate state values
        let valid_states = [
            "completed",
            "pending",
            "cancelled",
            "rejected",
            "confirming",
            "unconfirmed",
            "requesting",
        ];
        assert!(
            valid_states.iter().any(|&s| withdrawal.state.contains(s)),
            "State should be valid: {}",
            withdrawal.state
        );

        // Validate address format (basic check)
        assert!(
            withdrawal.address.len() >= 10,
            "Address should be at least 10 characters: {}",
            withdrawal.address
        );

        // Validate transaction ID format (basic check)
        if let Some(ref tx_id) = withdrawal.transaction_id {
            assert!(
                tx_id.len() >= 10,
                "Transaction ID should be at least 10 characters: {}",
                tx_id
            );
        }

        // Validate timestamp relationship
        if let Some(updated_ts) = withdrawal.updated_timestamp {
            assert!(
                updated_ts >= withdrawal.created_timestamp,
                "Updated timestamp should be >= created timestamp"
            );
        }

        // Note: confirmation_id field doesn't exist in Withdrawal struct
        // Validation removed as it's not part of the actual structure
    }

    info!("Withdrawals data validation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_withdrawals_multiple_currencies() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting withdrawals multiple currencies test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    let currencies = ["BTC", "ETH", "USDC"];

    for currency in &currencies {
        debug!("Getting withdrawals for {}", currency);
        let withdrawals_response = client.get_withdrawals(currency, Some(5), None).await?;

        info!(
            "{} withdrawals retrieved successfully, count: {}, total: {}",
            currency,
            withdrawals_response.data.len(),
            withdrawals_response.count
        );

        // Validate that all withdrawals match the requested currency
        for withdrawal in &withdrawals_response.data {
            assert_eq!(
                withdrawal.currency, *currency,
                "All withdrawals should match requested currency: {} == {}",
                withdrawal.currency, currency
            );
        }

        // Small delay between requests to respect rate limits
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    info!("Withdrawals multiple currencies test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_withdrawals_consistency() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting withdrawals consistency test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    // Get withdrawals multiple times to check consistency
    debug!("Getting first set of withdrawals");
    let withdrawals1 = client.get_withdrawals("BTC", Some(10), None).await?;

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    debug!("Getting second set of withdrawals");
    let withdrawals2 = client.get_withdrawals("BTC", Some(10), None).await?;

    info!("Both withdrawal sets retrieved successfully");

    // Check that the total count is consistent (might vary slightly due to new withdrawals)
    let count_diff = (withdrawals1.count as i32 - withdrawals2.count as i32).abs();
    assert!(
        count_diff <= 5,
        "Withdrawal count should be relatively stable (diff: {})",
        count_diff
    );

    // Check that common withdrawals have consistent data
    for with1 in &withdrawals1.data {
        if let Some(with2) = withdrawals2
            .data
            .iter()
            .find(|w| w.transaction_id == with1.transaction_id)
        {
            assert_eq!(
                with1.address, with2.address,
                "Address should be consistent for {:?}",
                with1.transaction_id
            );
            assert_eq!(
                with1.amount, with2.amount,
                "Amount should be consistent for {:?}",
                with1.transaction_id
            );
            assert_eq!(
                with1.currency, with2.currency,
                "Currency should be consistent for {:?}",
                with1.transaction_id
            );
            assert_eq!(
                with1.fee, with2.fee,
                "Fee should be consistent for {:?}",
                with1.transaction_id
            );
            assert_eq!(
                with1.id, with2.id,
                "ID should be consistent for {:?}",
                with1.transaction_id
            );
            assert_eq!(
                with1.created_timestamp, with2.created_timestamp,
                "Created timestamp should be consistent for {:?}",
                with1.transaction_id
            );

            // State might change, but updated timestamp should be >= previous
            if let (Some(ts1), Some(ts2)) = (with1.updated_timestamp, with2.updated_timestamp) {
                assert!(
                    ts2 >= ts1,
                    "Updated timestamp should not go backwards for {:?}: {:?} >= {:?}",
                    with1.transaction_id,
                    with2.updated_timestamp,
                    with1.updated_timestamp
                );
            }
        }
    }

    info!("Withdrawals consistency test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_withdrawals_state_analysis() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting withdrawals state analysis test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting withdrawals for state analysis");
    let withdrawals_response = client.get_withdrawals("BTC", Some(50), None).await?;

    info!(
        "Withdrawals retrieved for state analysis, count: {}",
        withdrawals_response.data.len()
    );

    // Analyze withdrawal states
    let mut state_counts = std::collections::HashMap::new();
    for withdrawal in &withdrawals_response.data {
        *state_counts.entry(withdrawal.state.clone()).or_insert(0) += 1;
    }

    info!("Withdrawal state distribution: {:?}", state_counts);

    // Validate that we have reasonable state distribution
    for (state, count) in &state_counts {
        debug!("State '{}' appears {} times", state, count);
        assert!(*count > 0, "State count should be positive");

        // Validate state names
        let valid_states = [
            "completed",
            "pending",
            "cancelled",
            "rejected",
            "confirming",
            "unconfirmed",
            "requesting",
        ];
        assert!(
            valid_states.iter().any(|&s| state.contains(s)),
            "State should be valid: {}",
            state
        );
    }

    // Analyze fee distribution
    let fees: Vec<f64> = withdrawals_response.data.iter().map(|w| w.fee).collect();
    if !fees.is_empty() {
        let min_fee = fees.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_fee = fees.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let avg_fee = fees.iter().sum::<f64>() / fees.len() as f64;

        info!(
            "Fee analysis - min: {}, max: {}, avg: {}",
            min_fee, max_fee, avg_fee
        );

        assert!(min_fee >= 0.0, "Minimum fee should be non-negative");
        assert!(max_fee >= min_fee, "Maximum fee should be >= minimum fee");
        assert!(avg_fee >= 0.0, "Average fee should be non-negative");
    }

    info!("Withdrawals state analysis test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_withdrawals_empty_result() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting withdrawals empty result test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    // Try to get withdrawals with a very high offset to potentially get empty results
    debug!("Getting withdrawals with high offset to test empty results");
    let withdrawals_response = client.get_withdrawals("BTC", Some(10), Some(10000)).await?;

    info!(
        "Withdrawals with high offset retrieved successfully, count: {}, total: {}",
        withdrawals_response.data.len(),
        withdrawals_response.count
    );

    // Validate empty result structure
    if withdrawals_response.data.is_empty() {
        info!("Empty withdrawals result received as expected");
        assert_eq!(withdrawals_response.data.len(), 0, "Data should be empty");
        // Count might still be > 0 as it represents total available withdrawals
    } else {
        info!("Non-empty withdrawals result received, which is also valid");
        // Validate the structure even if not empty
        for withdrawal in &withdrawals_response.data {
            assert_eq!(withdrawal.currency, "BTC", "Currency should be BTC");
        }
    }

    info!("Withdrawals empty result test completed successfully");
    Ok(())
}
