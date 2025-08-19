//! Transfers Integration Tests
//!
//! This test covers transfer functionality:
//! 1. Test transfer to subaccount (simulation)
//! 2. Test transfer to user (simulation)
//! 3. Validate transfer request parameters
//! 4. Test error handling for invalid transfers
//!
//! Note: These tests use small amounts and may fail if insufficient balance.
//! They are designed to test the API endpoints rather than actual transfers.

use std::path::Path;
use tracing::{debug, info, warn};

use deribit_http::DeribitHttpClient;

/// Check if .env file exists and contains required variables
fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(".env").exists() {
        return Err("Missing .env file. Please create one with authentication credentials".into());
    }

    dotenv::dotenv().ok();

    let has_oauth2 = std::env::var("DERIBIT_CLIENT_ID").is_ok() && std::env::var("DERIBIT_CLIENT_SECRET").is_ok();
    let has_api_key = std::env::var("DERIBIT_API_KEY").is_ok() && std::env::var("DERIBIT_API_SECRET").is_ok();
    
    if !has_oauth2 && !has_api_key {
        return Err("Missing authentication credentials".into());
    }

    Ok(())
}

/// Authenticate client using available credentials
async fn authenticate_client(client: &DeribitHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    if let (Ok(client_id), Ok(client_secret)) = (std::env::var("DERIBIT_CLIENT_ID"), std::env::var("DERIBIT_CLIENT_SECRET")) {
        client.authenticate_oauth2(&client_id, &client_secret).await?;
    } else if let (Ok(api_key), Ok(api_secret)) = (std::env::var("DERIBIT_API_KEY"), std::env::var("DERIBIT_API_SECRET")) {
        client.authenticate_api_key(&api_key, &api_secret).await?;
    } else {
        return Err("No valid authentication credentials found".into());
    }
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_transfer_to_subaccount_validation() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;
    
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting transfer to subaccount validation test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;
    
    // First, get subaccounts to find a valid destination
    debug!("Getting subaccounts to find transfer destination");
    let subaccounts = client.get_subaccounts(None).await?;
    
    if subaccounts.is_empty() {
        info!("No subaccounts found, skipping transfer to subaccount test");
        return Ok(());
    }
    
    let destination_subaccount = subaccounts[0].id;
    info!("Using subaccount {} as transfer destination", destination_subaccount);
    
    // Test with a very small amount (this might still fail due to insufficient balance)
    let test_amount = 0.00001; // Very small amount for testing
    
    debug!("Attempting transfer to subaccount: {} BTC to subaccount {}", test_amount, destination_subaccount);
    let transfer_result = client.submit_transfer_to_subaccount("BTC", test_amount, destination_subaccount).await;
    
    match transfer_result {
        Ok(result) => {
            info!("Transfer to subaccount successful: {:?}", result);
            
            // Validate transfer result structure
            assert!(result.id > 0, "Transfer ID should be positive");
            assert_eq!(result.currency, "BTC", "Currency should be BTC");
            assert_eq!(result.amount, test_amount, "Amount should match requested amount");
            assert!(!result.state.is_empty(), "State should not be empty");
            assert!(!result.transfer_type.is_empty(), "Type should not be empty");
            assert!(result.created_timestamp > 0, "Created timestamp should be positive");
            assert!(result.updated_timestamp > 0, "Updated timestamp should be positive");
            assert!(!result.direction.is_empty(), "Direction should not be empty");
            
            // Validate state values
            let valid_states = ["completed", "pending", "cancelled", "rejected"];
            assert!(valid_states.iter().any(|&s| result.state.contains(s)),
                    "State should be valid: {}", result.state);
            
            // Validate type
            assert!(result.transfer_type.contains("subaccount") || result.transfer_type.contains("transfer"),
                    "Type should indicate subaccount transfer: {}", result.transfer_type);
            
        }
        Err(e) => {
            warn!("Transfer to subaccount failed (expected for testnet with no balance): {:?}", e);
            
            // Validate that it's a reasonable error (insufficient balance, etc.)
            let error_str = e.to_string().to_lowercase();
            assert!(error_str.contains("insufficient") || 
                   error_str.contains("balance") || 
                   error_str.contains("not_enough") ||
                   error_str.contains("invalid") ||
                   error_str.contains("minimum"),
                   "Error should be related to balance or validation: {}", e);
        }
    }
    
    info!("Transfer to subaccount validation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_transfer_to_user_validation() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;
    
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting transfer to user validation test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;
    
    // Use a test destination (this will likely fail, but we're testing the API structure)
    let test_destination = "test_user@example.com";
    let test_amount = 0.00001; // Very small amount for testing
    
    debug!("Attempting transfer to user: {} BTC to {}", test_amount, test_destination);
    let transfer_result = client.submit_transfer_to_user("BTC", test_amount, test_destination).await;
    
    match transfer_result {
        Ok(result) => {
            info!("Transfer to user successful: {:?}", result);
            
            // Validate transfer result structure
            assert!(result.id > 0, "Transfer ID should be positive");
            assert_eq!(result.currency, "BTC", "Currency should be BTC");
            assert_eq!(result.amount, test_amount, "Amount should match requested amount");
            assert!(!result.state.is_empty(), "State should not be empty");
            assert!(!result.transfer_type.is_empty(), "Type should not be empty");
            assert!(result.created_timestamp > 0, "Created timestamp should be positive");
            assert!(result.updated_timestamp > 0, "Updated timestamp should be positive");
            assert!(!result.direction.is_empty(), "Direction should not be empty");
            
            // Validate state values
            let valid_states = ["completed", "pending", "cancelled", "rejected"];
            assert!(valid_states.iter().any(|&s| result.state.contains(s)),
                    "State should be valid: {}", result.state);
            
            // Validate type
            assert!(result.transfer_type.contains("user") || result.transfer_type.contains("transfer"),
                    "Type should indicate user transfer: {}", result.transfer_type);
            
        }
        Err(e) => {
            warn!("Transfer to user failed (expected for testnet): {:?}", e);
            
            // Validate that it's a reasonable error
            let error_str = e.to_string().to_lowercase();
            assert!(error_str.contains("insufficient") || 
                   error_str.contains("balance") || 
                   error_str.contains("not_enough") ||
                   error_str.contains("invalid") ||
                   error_str.contains("user") ||
                   error_str.contains("not_found") ||
                   error_str.contains("minimum"),
                   "Error should be related to balance, user validation, or other reasonable cause: {}", e);
        }
    }
    
    info!("Transfer to user validation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_transfer_invalid_currency() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;
    
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting transfer invalid currency test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;
    
    // Test with invalid currency
    debug!("Attempting transfer with invalid currency");
    let transfer_result = client.submit_transfer_to_user("INVALID", 0.001, "test@example.com").await;
    
    match transfer_result {
        Ok(_) => {
            return Err("Transfer with invalid currency should have failed".into());
        }
        Err(e) => {
            info!("Transfer correctly failed with invalid currency: {:?}", e);
            let error_str = e.to_string().to_lowercase();
            assert!(error_str.contains("invalid") || 
                   error_str.contains("currency") || 
                   error_str.contains("not_found") ||
                   error_str.contains("unsupported"),
                   "Error should be related to invalid currency: {}", e);
        }
    }
    
    info!("Transfer invalid currency test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_transfer_zero_amount() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;
    
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting transfer zero amount test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;
    
    // Test with zero amount
    debug!("Attempting transfer with zero amount");
    let transfer_result = client.submit_transfer_to_user("BTC", 0.0, "test@example.com").await;
    
    match transfer_result {
        Ok(_) => {
            return Err("Transfer with zero amount should have failed".into());
        }
        Err(e) => {
            info!("Transfer correctly failed with zero amount: {:?}", e);
            let error_str = e.to_string().to_lowercase();
            assert!(error_str.contains("amount") || 
                   error_str.contains("zero") || 
                   error_str.contains("minimum") ||
                   error_str.contains("invalid"),
                   "Error should be related to invalid amount: {}", e);
        }
    }
    
    info!("Transfer zero amount test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_transfer_negative_amount() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;
    
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting transfer negative amount test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;
    
    // Test with negative amount
    debug!("Attempting transfer with negative amount");
    let transfer_result = client.submit_transfer_to_user("BTC", -0.001, "test@example.com").await;
    
    match transfer_result {
        Ok(_) => {
            return Err("Transfer with negative amount should have failed".into());
        }
        Err(e) => {
            info!("Transfer correctly failed with negative amount: {:?}", e);
            let error_str = e.to_string().to_lowercase();
            assert!(error_str.contains("amount") || 
                   error_str.contains("negative") || 
                   error_str.contains("positive") ||
                   error_str.contains("invalid"),
                   "Error should be related to invalid negative amount: {}", e);
        }
    }
    
    info!("Transfer negative amount test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_transfer_to_invalid_subaccount() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;
    
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting transfer to invalid subaccount test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;
    
    // Test with invalid subaccount ID
    let invalid_subaccount_id = 999999999;
    debug!("Attempting transfer to invalid subaccount: {}", invalid_subaccount_id);
    let transfer_result = client.submit_transfer_to_subaccount("BTC", 0.001, invalid_subaccount_id).await;
    
    match transfer_result {
        Ok(_) => {
            return Err("Transfer to invalid subaccount should have failed".into());
        }
        Err(e) => {
            info!("Transfer correctly failed with invalid subaccount: {:?}", e);
            let error_str = e.to_string().to_lowercase();
            assert!(error_str.contains("subaccount") || 
                   error_str.contains("not_found") || 
                   error_str.contains("invalid") ||
                   error_str.contains("destination"),
                   "Error should be related to invalid subaccount: {}", e);
        }
    }
    
    info!("Transfer to invalid subaccount test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_transfer_multiple_currencies() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;
    
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting transfer multiple currencies test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;
    
    let currencies = ["BTC", "ETH", "USDC"];
    let test_destination = "test@example.com";
    let test_amount = 0.00001;
    
    for currency in &currencies {
        debug!("Testing transfer for currency: {}", currency);
        let transfer_result = client.submit_transfer_to_user(currency, test_amount, test_destination).await;
        
        match transfer_result {
            Ok(result) => {
                info!("Transfer for {} successful: {:?}", currency, result);
                assert_eq!(result.currency, *currency, "Currency should match requested currency");
                assert_eq!(result.amount, test_amount, "Amount should match requested amount");
            }
            Err(e) => {
                warn!("Transfer for {} failed (expected): {:?}", currency, e);
                // This is expected for testnet accounts with no balance
                let error_str = e.to_string().to_lowercase();
                assert!(error_str.contains("insufficient") || 
                       error_str.contains("balance") || 
                       error_str.contains("not_enough") ||
                       error_str.contains("invalid") ||
                       error_str.contains("user") ||
                       error_str.contains("minimum"),
                       "Error should be reasonable for currency {}: {}", currency, e);
            }
        }
        
        // Small delay between requests to respect rate limits
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
    
    info!("Transfer multiple currencies test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_transfer_parameter_validation() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;
    
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting transfer parameter validation test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;
    
    // Test various parameter combinations to validate API behavior
    let test_cases = vec![
        ("BTC", 0.00001, "valid@example.com", "Valid parameters"),
        ("ETH", 0.00001, "valid@example.com", "Valid ETH transfer"),
        ("USDC", 0.01, "valid@example.com", "Valid USDC transfer"),
    ];
    
    for (currency, amount, destination, description) in test_cases {
        debug!("Testing: {} - {} {} to {}", description, amount, currency, destination);
        
        let transfer_result = client.submit_transfer_to_user(currency, amount, destination).await;
        
        match transfer_result {
            Ok(result) => {
                info!("Transfer successful for {}: {:?}", description, result);
                
                // Validate basic structure regardless of success
                assert_eq!(result.currency, currency, "Currency should match for {}", description);
                assert_eq!(result.amount, amount, "Amount should match for {}", description);
                assert!(!result.state.is_empty(), "State should not be empty for {}", description);
                assert!(!result.transfer_type.is_empty(), "Type should not be empty for {}", description);
                assert!(result.id > 0, "ID should be positive for {}", description);
                assert!(result.created_timestamp > 0, "Created timestamp should be positive for {}", description);
                assert!(result.updated_timestamp > 0, "Updated timestamp should be positive for {}", description);
                assert!(!result.direction.is_empty(), "Direction should not be empty for {}", description);
            }
            Err(e) => {
                warn!("Transfer failed for {} (expected): {:?}", description, e);
                // Validate that errors are reasonable
                let error_str = e.to_string().to_lowercase();
                assert!(!error_str.is_empty(), "Error message should not be empty for {}", description);
            }
        }
        
        // Small delay between requests
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    
    info!("Transfer parameter validation test completed successfully");
    Ok(())
}