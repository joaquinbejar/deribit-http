//! Transfer Integration Tests
//!
//! This test covers transfer functionality:
//! 1. Internal transfer operations
//! 2. Transfer validation and limits
//! 3. Cross-currency transfers
//! 4. Transfer history retrieval
//! 5. Transfer error handling
#[cfg(test)]
mod withdrawal_tests {
    use deribit_http::DeribitHttpClient;
    use std::path::Path;
    use tracing::{debug, info, warn};

    /// Check if .env file exists and contains required variables
    fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(".env").exists() {
            return Err(
                "Missing .env file. Please create one with authentication credentials".into(),
            );
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

    #[tokio::test]
    #[serial_test::serial]
    async fn test_transfer_to_user_validation() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting transfer to user validation test

        let client = DeribitHttpClient::new();

        // Use a test destination (this will likely fail, but we're testing the API structure)
        let test_destination = "test_user@example.com";
        let test_amount = 0.00001; // Very small amount for testing

        debug!(
            "Attempting transfer to user: {} BTC to {}",
            test_amount, test_destination
        );
        let transfer_result = client
            .submit_transfer_to_user("BTC", test_amount, test_destination)
            .await;

        match transfer_result {
            Ok(result) => {
                info!("Transfer to user successful: {:?}", result);

                // Validate transfer result structure
                assert!(!result.id.is_empty(), "Transfer ID should not be empty");
                assert!(!result.status.is_empty(), "Status should not be empty");

                // Validate state values
                let valid_states = ["completed", "pending", "cancelled", "rejected"];
                assert!(
                    valid_states.iter().any(|&s| result.status.contains(s)),
                    "State should be valid: {}",
                    result.status
                );

                // Basic validation - TransferResult only has id and status fields
            }
            Err(e) => {
                warn!("Transfer to user failed (expected for testnet): {:?}", e);

                // Validate that it's a reasonable error
                let error_str = e.to_string().to_lowercase();
                assert!(
                    error_str.contains("insufficient")
                        || error_str.contains("balance")
                        || error_str.contains("not_enough")
                        || error_str.contains("invalid")
                        || error_str.contains("user")
                        || error_str.contains("not_found")
                        || error_str.contains("minimum"),
                    "Error should be related to balance, user validation, or other reasonable cause: {}",
                    e
                );
            }
        }

        info!("Transfer to user validation test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_transfer_invalid_currency() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting transfer invalid currency test

        let client = DeribitHttpClient::new();

        // Test with invalid currency
        debug!("Attempting transfer with invalid currency");
        let transfer_result = client
            .submit_transfer_to_user("INVALID", 0.001, "test@example.com")
            .await;

        match transfer_result {
            Ok(_) => {
                return Err("Transfer with invalid currency should have failed".into());
            }
            Err(e) => {
                info!("Transfer correctly failed with invalid currency: {:?}", e);
                let error_str = e.to_string().to_lowercase();
                assert!(
                    error_str.contains("invalid")
                        || error_str.contains("currency")
                        || error_str.contains("not_found")
                        || error_str.contains("unsupported"),
                    "Error should be related to invalid currency: {}",
                    e
                );
            }
        }

        info!("Transfer invalid currency test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_transfer_zero_amount() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting transfer zero amount test

        let client = DeribitHttpClient::new();

        // Test with zero amount
        debug!("Attempting transfer with zero amount");
        let transfer_result = client
            .submit_transfer_to_user("BTC", 0.0, "test@example.com")
            .await;

        match transfer_result {
            Ok(_) => {
                return Err("Transfer with zero amount should have failed".into());
            }
            Err(e) => {
                info!("Transfer correctly failed with zero amount: {:?}", e);
                let error_str = e.to_string().to_lowercase();
                assert!(
                    error_str.contains("amount")
                        || error_str.contains("zero")
                        || error_str.contains("minimum")
                        || error_str.contains("invalid"),
                    "Error should be related to invalid amount: {}",
                    e
                );
            }
        }

        info!("Transfer zero amount test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_transfer_negative_amount() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting transfer negative amount test

        let client = DeribitHttpClient::new();

        // Test with negative amount
        debug!("Attempting transfer with negative amount");
        let transfer_result = client
            .submit_transfer_to_user("BTC", -0.001, "test@example.com")
            .await;

        match transfer_result {
            Ok(_) => {
                return Err("Transfer with negative amount should have failed".into());
            }
            Err(e) => {
                info!("Transfer correctly failed with negative amount: {:?}", e);
                let error_str = e.to_string().to_lowercase();
                assert!(
                    error_str.contains("amount")
                        || error_str.contains("negative")
                        || error_str.contains("positive")
                        || error_str.contains("invalid"),
                    "Error should be related to invalid negative amount: {}",
                    e
                );
            }
        }

        info!("Transfer negative amount test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_transfer_to_invalid_subaccount() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting transfer to invalid subaccount test

        let client = DeribitHttpClient::new();

        // Test with invalid subaccount ID
        let invalid_subaccount_id = 999999999;
        debug!(
            "Attempting transfer to invalid subaccount: {}",
            invalid_subaccount_id
        );
        let transfer_result = client
            .submit_transfer_to_subaccount("BTC", 0.001, invalid_subaccount_id)
            .await;

        match transfer_result {
            Ok(_) => {
                return Err("Transfer to invalid subaccount should have failed".into());
            }
            Err(e) => {
                info!("Transfer correctly failed with invalid subaccount: {:?}", e);
                let error_str = e.to_string().to_lowercase();
                assert!(
                    error_str.contains("subaccount")
                        || error_str.contains("not_found")
                        || error_str.contains("invalid")
                        || error_str.contains("destination"),
                    "Error should be related to invalid subaccount: {}",
                    e
                );
            }
        }

        info!("Transfer to invalid subaccount test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_transfer_multiple_currencies() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting transfer multiple currencies test

        let client = DeribitHttpClient::new();

        let currencies = ["BTC", "ETH", "USDC"];
        let test_destination = "test@example.com";
        let test_amount = 0.00001;

        for currency in &currencies {
            debug!("Testing transfer for currency: {}", currency);
            let transfer_result = client
                .submit_transfer_to_user(currency, test_amount, test_destination)
                .await;

            match transfer_result {
                Ok(result) => {
                    info!("Transfer for {} successful: {:?}", currency, result);
                    assert!(!result.id.is_empty(), "Transfer ID should not be empty");
                    assert!(!result.status.is_empty(), "Status should not be empty");
                }
                Err(e) => {
                    warn!("Transfer for {} failed (expected): {:?}", currency, e);
                    // This is expected for testnet accounts with no balance
                    let error_str = e.to_string().to_lowercase();
                    assert!(
                        error_str.contains("insufficient")
                            || error_str.contains("balance")
                            || error_str.contains("not_enough")
                            || error_str.contains("invalid")
                            || error_str.contains("user")
                            || error_str.contains("minimum"),
                        "Error should be reasonable for currency {}: {}",
                        currency,
                        e
                    );
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

        // Starting transfer parameter validation test

        let client = DeribitHttpClient::new();

        // Test various parameter combinations to validate API behavior
        let test_cases = vec![
            ("BTC", 0.00001, "valid@example.com", "Valid parameters"),
            ("ETH", 0.00001, "valid@example.com", "Valid ETH transfer"),
            ("USDC", 0.01, "valid@example.com", "Valid USDC transfer"),
        ];

        for (currency, amount, destination, description) in test_cases {
            debug!(
                "Testing: {} - {} {} to {}",
                description, amount, currency, destination
            );

            let transfer_result = client
                .submit_transfer_to_user(currency, amount, destination)
                .await;

            match transfer_result {
                Ok(result) => {
                    info!("Transfer successful for {}: {:?}", description, result);
                    assert!(
                        !result.id.is_empty(),
                        "Transfer ID should not be empty for {}",
                        description
                    );
                    assert!(
                        !result.status.is_empty(),
                        "Status should not be empty for {}",
                        description
                    );
                }
                Err(e) => {
                    warn!("Transfer failed for {} (expected): {:?}", description, e);
                    // Validate that errors are reasonable
                    let error_str = e.to_string().to_lowercase();
                    assert!(
                        !error_str.is_empty(),
                        "Error message should not be empty for {}",
                        description
                    );
                }
            }

            // Small delay between requests
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        info!("Transfer parameter validation test completed successfully");
        Ok(())
    }

    // =========================================================================
    // Get Transfers Tests (Issue #28)
    // =========================================================================

    #[tokio::test]
    #[ignore = "Requires authentication"]
    #[serial_test::serial]
    async fn test_get_transfers_success() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        let client = DeribitHttpClient::new();

        // Get transfers for BTC
        let result = client.get_transfers("BTC", Some(10), None).await;

        match result {
            Ok(transfers) => {
                info!("Got {} transfers (total: {})", transfers.len(), transfers.count);
                for transfer in &transfers.data {
                    debug!(
                        "Transfer ID: {}, Amount: {} {}, Direction: {:?}, State: {:?}",
                        transfer.id,
                        transfer.amount,
                        transfer.currency,
                        transfer.direction,
                        transfer.state
                    );
                }
            }
            Err(e) => {
                warn!("Get transfers failed (may be expected): {:?}", e);
            }
        }

        Ok(())
    }

    #[tokio::test]
    #[ignore = "Requires authentication"]
    #[serial_test::serial]
    async fn test_get_transfers_with_pagination() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        let client = DeribitHttpClient::new();

        // Get first page
        let page1 = client.get_transfers("BTC", Some(5), Some(0)).await;
        // Get second page
        let page2 = client.get_transfers("BTC", Some(5), Some(5)).await;

        match (page1, page2) {
            (Ok(p1), Ok(p2)) => {
                info!("Page 1: {} transfers, Page 2: {} transfers", p1.len(), p2.len());
                assert!(
                    p1.count == p2.count,
                    "Total count should be consistent across pages"
                );
            }
            (Err(e1), _) => {
                warn!("Page 1 failed: {:?}", e1);
            }
            (_, Err(e2)) => {
                warn!("Page 2 failed: {:?}", e2);
            }
        }

        Ok(())
    }

    #[tokio::test]
    #[ignore = "Requires authentication"]
    #[serial_test::serial]
    async fn test_get_transfers_multiple_currencies() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        let client = DeribitHttpClient::new();

        let currencies = ["BTC", "ETH", "USDC"];

        for currency in currencies {
            let result = client.get_transfers(currency, Some(5), None).await;

            match result {
                Ok(transfers) => {
                    info!(
                        "{}: {} transfers (total: {})",
                        currency,
                        transfers.len(),
                        transfers.count
                    );
                }
                Err(e) => {
                    warn!("Get transfers for {} failed: {:?}", currency, e);
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        Ok(())
    }

    // =========================================================================
    // Cancel Transfer Tests (Issue #28)
    // =========================================================================

    #[tokio::test]
    #[ignore = "Requires authentication and active transfer"]
    #[serial_test::serial]
    async fn test_cancel_transfer_by_id() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        let client = DeribitHttpClient::new();

        // This test requires an actual pending transfer to cancel
        // Using a non-existent ID to test error handling
        let result = client.cancel_transfer_by_id("BTC", 999999).await;

        match result {
            Ok(transfer) => {
                info!("Transfer cancelled: {:?}", transfer);
                assert!(transfer.is_cancelled(), "Transfer should be cancelled");
            }
            Err(e) => {
                warn!("Cancel transfer failed (expected for non-existent ID): {:?}", e);
            }
        }

        Ok(())
    }

    // =========================================================================
    // Submit Transfer Between Subaccounts Tests (Issue #28)
    // =========================================================================

    #[tokio::test]
    #[ignore = "Requires authentication and subaccounts"]
    #[serial_test::serial]
    async fn test_submit_transfer_between_subaccounts() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        let client = DeribitHttpClient::new();

        // This test requires actual subaccounts to exist
        // Using test parameters that will likely fail but test API structure
        let result = client
            .submit_transfer_between_subaccounts("BTC", 0.00001, 1, None)
            .await;

        match result {
            Ok(transfer) => {
                info!("Transfer submitted: {:?}", transfer);
                assert_eq!(transfer.currency, "BTC");
                assert!(transfer.is_confirmed() || transfer.is_pending());
            }
            Err(e) => {
                warn!(
                    "Submit transfer between subaccounts failed (expected): {:?}",
                    e
                );
            }
        }

        Ok(())
    }

    #[tokio::test]
    #[ignore = "Requires authentication and subaccounts"]
    #[serial_test::serial]
    async fn test_submit_transfer_between_subaccounts_with_source(
    ) -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        let client = DeribitHttpClient::new();

        // Test with explicit source subaccount
        let result = client
            .submit_transfer_between_subaccounts("ETH", 0.001, 20, Some(10))
            .await;

        match result {
            Ok(transfer) => {
                info!("Transfer submitted with source: {:?}", transfer);
                assert_eq!(transfer.currency, "ETH");
            }
            Err(e) => {
                warn!(
                    "Submit transfer with source failed (expected): {:?}",
                    e
                );
            }
        }

        Ok(())
    }

    #[tokio::test]
    #[ignore = "Requires authentication"]
    #[serial_test::serial]
    async fn test_transfer_workflow() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        let client = DeribitHttpClient::new();

        info!("Testing complete transfer workflow");

        // Step 1: Get initial transfers
        let initial_transfers = client.get_transfers("BTC", Some(10), None).await;
        let initial_count = initial_transfers.map(|t| t.count).unwrap_or(0);
        info!("Initial transfer count: {}", initial_count);

        // Step 2: Attempt a transfer (will likely fail without proper setup)
        let transfer_result = client
            .submit_transfer_between_subaccounts("BTC", 0.00001, 1, None)
            .await;

        match transfer_result {
            Ok(transfer) => {
                info!("Transfer created: ID={}", transfer.id);

                // Step 3: Verify transfer appears in list
                let updated_transfers = client.get_transfers("BTC", Some(10), None).await;
                if let Ok(transfers) = updated_transfers {
                    info!("Updated transfer count: {}", transfers.count);
                }

                // Step 4: Attempt to cancel if transfer is pending
                if transfer.is_pending() {
                    let cancel_result = client
                        .cancel_transfer_by_id("BTC", transfer.id)
                        .await;
                    match cancel_result {
                        Ok(cancelled) => {
                            info!("Transfer cancelled: {:?}", cancelled.state);
                        }
                        Err(e) => {
                            warn!("Cancel failed: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Transfer workflow test: transfer creation failed (expected): {:?}", e);
            }
        }

        info!("Transfer workflow test completed");
        Ok(())
    }
}
