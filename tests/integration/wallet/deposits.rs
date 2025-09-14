//! Deposits Integration Tests
//!
//! This test covers deposits functionality:
//! 1. Get deposits for different currencies

#[cfg(test)]
mod deposits_tests {
    use tracing::{debug, info};
    use deribit_http::DeribitHttpClient;
    use std::path::Path;

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
    
    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_deposits_btc() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting BTC deposits test

        let client = DeribitHttpClient::new();

        debug!("Getting BTC deposits");
        let deposits_response = client.get_deposits("BTC", None, None).await?;

        info!(
            "BTC deposits retrieved successfully, count: {}, total: {}",
            deposits_response.data.len(),
            deposits_response.count
        );
        debug!("Deposits response: {:?}", deposits_response);

        // Validate deposits response structure
        assert!(
            deposits_response.count >= deposits_response.data.len() as u32,
            "Count should be >= data length: {} >= {}",
            deposits_response.count,
            deposits_response.data.len()
        );

        // Validate individual deposits
        for (i, deposit) in deposits_response.data.iter().enumerate() {
            debug!(
                "Validating deposit #{}: {:?}",
                i + 1,
                deposit.transaction_id
            );

            assert!(
                !deposit.address.is_empty(),
                "Deposit address should not be empty"
            );
            assert!(deposit.amount > 0.0, "Deposit amount should be positive");
            assert_eq!(deposit.currency, "BTC", "Currency should be BTC");
            assert!(
                deposit.received_timestamp > 0,
                "Received timestamp should be positive"
            );
            assert!(
                deposit.transaction_id.is_some(),
                "Transaction ID should be present"
            );
            if let Some(tx_id) = &deposit.transaction_id {
                assert!(!tx_id.is_empty(), "Transaction ID should not be empty");
            }
            if let Some(updated_ts) = deposit.updated_timestamp {
                assert!(updated_ts > 0, "Updated timestamp should be positive");
            }

            // Validate state values
            let valid_states = ["completed", "pending", "cancelled", "rejected"];
            assert!(
                valid_states.iter().any(|&s| deposit.state.contains(s)),
                "State should be valid: {}",
                deposit.state
            );

            // Validate timestamps order
            if let Some(updated_ts) = deposit.updated_timestamp {
                assert!(
                    updated_ts >= deposit.received_timestamp,
                    "Updated timestamp should be >= received timestamp: {:?} >= {}",
                    deposit.updated_timestamp,
                    deposit.received_timestamp
                );
            }
        }

        info!("BTC deposits test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_deposits_eth() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting ETH deposits test

        let client = DeribitHttpClient::new();

        debug!("Getting ETH deposits");
        let deposits_response = client.get_deposits("ETH", None, None).await?;

        info!(
            "ETH deposits retrieved successfully, count: {}, total: {}",
            deposits_response.data.len(),
            deposits_response.count
        );
        debug!("Deposits response: {:?}", deposits_response);

        // Validate that all deposits are ETH-related
        for deposit in &deposits_response.data {
            assert_eq!(
                deposit.currency, "ETH",
                "All deposits should be ETH-related"
            );
        }

        info!("ETH deposits test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_deposits_with_count() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting deposits with count test

        let client = DeribitHttpClient::new();

        let requested_count = 5;
        debug!("Getting deposits with count: {}", requested_count);
        let deposits_response = client
            .get_deposits("BTC", Some(requested_count), None)
            .await?;

        info!(
            "Deposits with count retrieved successfully, count: {}, total: {}",
            deposits_response.data.len(),
            deposits_response.count
        );
        debug!("Deposits response: {:?}", deposits_response);

        // Validate that we got at most the requested count
        assert!(
            deposits_response.data.len() <= requested_count as usize,
            "Should not receive more than requested count: {} <= {}",
            deposits_response.data.len(),
            requested_count
        );

        info!("Deposits with count test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_deposits_with_offset() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting deposits with offset test

        let client = DeribitHttpClient::new();

        // Get first page
        debug!("Getting first page of deposits");
        let first_page = client.get_deposits("BTC", Some(5), Some(0)).await?;

        info!(
            "First page retrieved successfully, count: {}",
            first_page.data.len()
        );

        if first_page.data.len() >= 5 {
            // Get second page with offset
            debug!("Getting second page with offset: 5");
            let second_page = client.get_deposits("BTC", Some(5), Some(5)).await?;

            info!(
                "Second page retrieved successfully, count: {}",
                second_page.data.len()
            );
            debug!("Second page: {:?}", second_page);

            // Validate that pages don't overlap
            for first_deposit in &first_page.data {
                for second_deposit in &second_page.data {
                    assert_ne!(
                        first_deposit.transaction_id, second_deposit.transaction_id,
                        "Transaction IDs should not overlap between pages"
                    );
                }
            }
        } else {
            info!("Not enough deposits for offset test, skipping second page");
        }

        info!("Deposits with offset test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_deposits_data_validation() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting deposits data validation test

        let client = DeribitHttpClient::new();

        debug!("Getting deposits for data validation");
        let deposits_response = client.get_deposits("BTC", Some(20), None).await?;

        info!(
            "Deposits retrieved for validation, count: {}",
            deposits_response.data.len()
        );

        for deposit in &deposits_response.data {
            debug!(
                "Validating deposit: {:?} - {}",
                deposit.transaction_id, deposit.currency
            );

            // Validate required string fields
            assert!(!deposit.address.is_empty(), "Address should not be empty");
            assert!(!deposit.currency.is_empty(), "Currency should not be empty");
            assert!(!deposit.state.is_empty(), "State should not be empty");
            if let Some(tx_id) = &deposit.transaction_id {
                assert!(!tx_id.is_empty(), "Transaction ID should not be empty");
            }

            // Validate numeric fields
            assert!(deposit.amount > 0.0, "Amount should be positive");
            assert!(deposit.amount.is_finite(), "Amount should be finite");
            assert!(
                deposit.received_timestamp > 0,
                "Received timestamp should be positive"
            );
            if let Some(updated_ts) = deposit.updated_timestamp {
                assert!(updated_ts > 0, "Updated timestamp should be positive");
            }

            // Validate currency values
            let valid_currencies = ["BTC", "ETH", "USDC", "USDT", "EURR"];
            assert!(
                valid_currencies.contains(&deposit.currency.as_str()),
                "Currency should be valid: {}",
                deposit.currency
            );

            // Validate state values
            let valid_states = [
                "completed",
                "pending",
                "cancelled",
                "rejected",
                "confirming",
            ];
            assert!(
                valid_states.iter().any(|&s| deposit.state.contains(s)),
                "State should be valid: {}",
                deposit.state
            );

            // Validate address format (basic check)
            assert!(
                deposit.address.len() >= 10,
                "Address should be at least 10 characters: {}",
                deposit.address
            );

            // Validate transaction ID format (basic check)
            if let Some(tx_id) = &deposit.transaction_id {
                assert!(
                    tx_id.len() >= 10,
                    "Transaction ID should be at least 10 characters: {:?}",
                    deposit.transaction_id
                );
            }

            // Validate timestamp relationship
            if let Some(updated_ts) = deposit.updated_timestamp {
                assert!(
                    updated_ts >= deposit.received_timestamp,
                    "Updated timestamp should be >= received timestamp"
                );
            }
        }

        info!("Deposits data validation test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_deposits_multiple_currencies() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting deposits multiple currencies test

        let client = DeribitHttpClient::new();

        let currencies = ["BTC", "ETH", "USDC"];

        for currency in &currencies {
            debug!("Getting deposits for {}", currency);
            let deposits_response = client.get_deposits(currency, Some(5), None).await?;

            info!(
                "{} deposits retrieved successfully, count: {}, total: {}",
                currency,
                deposits_response.data.len(),
                deposits_response.count
            );

            // Validate that all deposits match the requested currency
            for deposit in &deposits_response.data {
                assert_eq!(
                    deposit.currency, *currency,
                    "All deposits should match requested currency: {} == {}",
                    deposit.currency, currency
                );
            }

            // Small delay between requests to respect rate limits
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        info!("Deposits multiple currencies test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_deposits_consistency() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting deposits consistency test

        let client = DeribitHttpClient::new();

        // Get deposits multiple times to check consistency
        debug!("Getting first set of deposits");
        let deposits1 = client.get_deposits("BTC", Some(10), None).await?;

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        debug!("Getting second set of deposits");
        let deposits2 = client.get_deposits("BTC", Some(10), None).await?;

        info!("Both deposit sets retrieved successfully");

        // Check that the total count is consistent (might vary slightly due to new deposits)
        let count_diff = (deposits1.count as i32 - deposits2.count as i32).abs();
        assert!(
            count_diff <= 5,
            "Deposit count should be relatively stable (diff: {})",
            count_diff
        );

        // Check that common deposits have consistent data
        for dep1 in &deposits1.data {
            if let Some(dep2) = deposits2
                .data
                .iter()
                .find(|d| d.transaction_id == dep1.transaction_id)
            {
                assert_eq!(
                    dep1.address, dep2.address,
                    "Address should be consistent for {:?}",
                    dep1.transaction_id
                );
                assert_eq!(
                    dep1.amount, dep2.amount,
                    "Amount should be consistent for {:?}",
                    dep1.transaction_id
                );
                assert_eq!(
                    dep1.currency, dep2.currency,
                    "Currency should be consistent for {:?}",
                    dep1.transaction_id
                );
                assert_eq!(
                    dep1.received_timestamp, dep2.received_timestamp,
                    "Received timestamp should be consistent for {:?}",
                    dep1.transaction_id
                );

                // State might change, but updated timestamp should be >= previous
                if let (Some(ts1), Some(ts2)) = (dep1.updated_timestamp, dep2.updated_timestamp) {
                    assert!(
                        ts2 >= ts1,
                        "Updated timestamp should not go backwards for {:?}: {:?} >= {:?}",
                        dep1.transaction_id,
                        dep2.updated_timestamp,
                        dep1.updated_timestamp
                    );
                }
            }
        }

        info!("Deposits consistency test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_deposits_empty_result() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        // Starting deposits empty result test

        let client = DeribitHttpClient::new();

        // Try to get deposits with a very high offset to potentially get empty results
        debug!("Getting deposits with high offset to test empty results");
        let deposits_response = client.get_deposits("BTC", Some(10), Some(10000)).await?;

        info!(
            "Deposits with high offset retrieved successfully, count: {}, total: {}",
            deposits_response.data.len(),
            deposits_response.count
        );

        // Validate empty result structure
        if deposits_response.data.is_empty() {
            info!("Empty deposits result received as expected");
            assert_eq!(deposits_response.data.len(), 0, "Data should be empty");
            // Count might still be > 0 as it represents total available deposits
        } else {
            info!("Non-empty deposits result received, which is also valid");
            // Validate the structure even if not empty
            for deposit in &deposits_response.data {
                assert_eq!(deposit.currency, "BTC", "Currency should be BTC");
            }
        }

        info!("Deposits empty result test completed successfully");
        Ok(())
    }
}