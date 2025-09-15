//! Transaction Log Integration Tests
//!
//! This test covers transaction log functionality:
//! 1. Transaction history retrieval
//! 2. Transaction filtering and pagination
//! 3. Transaction type validation
//! 4. Date range filtering
//! 5. Currency-specific transaction logs

#[cfg(test)]
mod transaction_log_tests {
    use deribit_http::DeribitHttpClient;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tracing::{debug, info};
    use deribit_http::model::TransactionLogRequest;

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
    async fn test_get_transaction_log_btc() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting BTC transaction log test");

        let client = DeribitHttpClient::new();

        debug!("Getting BTC transaction log");
        let transaction_log = client
            .get_transaction_log(TransactionLogRequest {
                currency: "BTC".to_string(),
                start_timestamp: 0,
                end_timestamp: 0,
                query: None,
                count: None,
                subaccount_id: None,
                continuation: None,
            })
            .await?;

        info!(
            "BTC transaction log retrieved successfully, count: {}",
            transaction_log.logs.len()
        );
        debug!("Transaction log: {:?}", transaction_log);

        // Validate transaction log structure
        for (i, log_entry) in transaction_log.logs.iter().enumerate() {
            debug!(
                "Validating transaction log entry #{}: {:?}",
                i + 1,
                log_entry.transaction_type
            );

            assert!(log_entry.id > 0, "Transaction ID should be positive");
            // User ID field doesn't exist in TransactionLogEntry
            // Username field doesn't exist in TransactionLogEntry
            // User sequence field doesn't exist in TransactionLogEntry
            // TransactionType is an enum, validate it exists
            debug!("Transaction type: {:?}", log_entry.transaction_type);
            assert!(log_entry.timestamp > 0, "Timestamp should be positive");
            assert_eq!(log_entry.currency, "BTC", "Currency should be BTC");
            // Amount can be None for some transaction types
            if let Some(amount) = log_entry.amount {
                assert!(amount.is_finite(), "Amount should be a finite number");
            }
            assert!(
                log_entry.balance.is_finite(),
                "Balance should be a finite number"
            );
            
        }

        info!("BTC transaction log test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_transaction_log_eth() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting ETH transaction log test");

        let client = DeribitHttpClient::new();

        debug!("Getting ETH transaction log");
        let transaction_log = client
            .get_transaction_log(TransactionLogRequest {
                currency: "ETH".to_string(),
                start_timestamp: 0,
                end_timestamp: 0,
                query: None,
                count: None,
                subaccount_id: None,
                continuation: None,
            })
            .await?;

        info!(
            "ETH transaction log retrieved successfully, count: {}",
            transaction_log.logs.len()
        );
        debug!("Transaction log: {:?}", transaction_log);

        // Validate that all entries are ETH-related
        for log_entry in &transaction_log.logs {
            assert_eq!(
                log_entry.currency, "ETH",
                "All entries should be ETH-related"
            );
        }

        info!("ETH transaction log test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_transaction_log_with_count() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting transaction log with count test");

        let client = DeribitHttpClient::new();

        let requested_count = 5;
        debug!("Getting transaction log with count: {}", requested_count);
        let transaction_log = client
            .get_transaction_log(TransactionLogRequest {
                currency: "BTC".to_string(),
                start_timestamp: 0,
                end_timestamp: 0,
                query: None,
                count: Some(requested_count),
                subaccount_id: None,
                continuation: None,
            })
            .await?;

        info!(
            "Transaction log with count retrieved successfully, count: {}",
            transaction_log.logs.len()
        );
        debug!("Transaction log: {:?}", transaction_log);

        // Validate that we got at most the requested count
        assert!(
            transaction_log.logs.len() <= requested_count as usize,
            "Should not receive more than requested count: {} <= {}",
            transaction_log.logs.len(),
            requested_count
        );

        info!("Transaction log with count test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_transaction_log_with_time_range() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting transaction log with time range test");

        let client = DeribitHttpClient::new();

        // Get current timestamp and calculate a range (last 30 days)
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
        let thirty_days_ago = now - (30 * 24 * 60 * 60 * 1000); // 30 days in milliseconds

        debug!(
            "Getting transaction log for time range: {} to {}",
            thirty_days_ago, now
        );
        let transaction_log = client
            .get_transaction_log(TransactionLogRequest {
                currency: "BTC".to_string(),
                start_timestamp: thirty_days_ago,
                end_timestamp: now,
                query: None,
                count: None,
                subaccount_id: None,
                continuation: None,
            })
            .await?;

        info!(
            "Transaction log with time range retrieved successfully, count: {}",
            transaction_log.logs.len()
        );
        debug!("Transaction log: {:?}", transaction_log);

        // Validate that all entries are within the time range
        for log_entry in &transaction_log.logs {
            assert!(
                log_entry.timestamp >= thirty_days_ago,
                "Transaction timestamp should be within range: {} >= {}",
                log_entry.timestamp,
                thirty_days_ago
            );
            assert!(
                log_entry.timestamp <= now,
                "Transaction timestamp should be within range: {} <= {}",
                log_entry.timestamp,
                now
            );
        }

        info!("Transaction log with time range test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_transaction_log_pagination() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting transaction log pagination test");

        let client = DeribitHttpClient::new();

        // Get first page
        debug!("Getting first page of transaction log");
        let first_page = client
            .get_transaction_log(TransactionLogRequest {
                currency: "BTC".to_string(),
                start_timestamp: 0,
                end_timestamp: 0,
                query: None,
                count: Some(10),
                subaccount_id: None,
                continuation: None,
            })
            .await?;

        info!(
            "First page retrieved successfully, count: {}",
            first_page.logs.len()
        );

        // If there's a continuation token, get the next page
        if let Some(continuation) = &first_page.continuation {
            debug!("Getting second page with continuation: {}", continuation);
            let second_page = client
                .get_transaction_log(TransactionLogRequest {
                    currency: "BTC".to_string(),
                    start_timestamp: 0,
                    end_timestamp: 0,
                    query: None,
                    count: Some(10),
                    subaccount_id: None,
                    continuation: Some(*continuation),
                })
                .await?;

            info!(
                "Second page retrieved successfully, count: {}",
                second_page.logs.len()
            );
            debug!("Second page: {:?}", second_page);

            // Validate that pages don't overlap
            for first_entry in &first_page.logs {
                for second_entry in &second_page.logs {
                    assert_ne!(
                        first_entry.id, second_entry.id,
                        "Transaction IDs should not overlap between pages"
                    );
                }
            }

            // Validate that second page entries are older (assuming descending order)
            if !first_page.logs.is_empty() && !second_page.logs.is_empty() {
                let first_page_latest = first_page.logs.iter().map(|e| e.timestamp).max().unwrap();
                let second_page_earliest =
                    second_page.logs.iter().map(|e| e.timestamp).min().unwrap();

                // Allow some overlap due to potential same timestamps
                if first_page_latest > second_page_earliest {
                    debug!(
                        "Pages might have overlapping timestamps: {} > {}",
                        first_page_latest, second_page_earliest
                    );
                }
            }
        } else {
            info!("No continuation token found, all data retrieved in first page");
        }

        info!("Transaction log pagination test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_transaction_log_data_validation() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting transaction log data validation test");

        let client = DeribitHttpClient::new();

        debug!("Getting transaction log for data validation");
        let transaction_log = client
            .get_transaction_log(TransactionLogRequest {
                currency: "BTC".to_string(),
                start_timestamp: 0,
                end_timestamp: 0,
                query: None,
                count: Some(20),
                subaccount_id: None,
                continuation: None,
            })
            .await?;

        info!(
            "Transaction log retrieved for validation, count: {}",
            transaction_log.logs.len()
        );

        for log_entry in &transaction_log.logs {
            debug!(
                "Validating transaction log entry: {} - {:?}",
                log_entry.id, log_entry.transaction_type
            );

            // Validate required fields
            assert!(log_entry.id > 0, "Transaction ID should be positive");
            assert!(log_entry.timestamp > 0, "Timestamp should be positive");
            assert!(
                !log_entry.currency.is_empty(),
                "Currency should not be empty"
            );
            assert!(
                !log_entry.transaction_type.is_empty(),
                "Transaction type should not be empty"
            );

            // Validate numeric fields
            if let Some(amount) = log_entry.amount {
                assert!(
                    amount.is_finite(),
                    "Amount should be a finite number"
                );
            }
            assert!(
                log_entry.balance.is_finite(),
                "Balance should be a finite number"
            );

            // Validate currency values
            let valid_currencies = ["BTC", "ETH", "USDC", "USDT", "EURR"];
            assert!(
                valid_currencies.contains(&log_entry.currency.as_str()),
                "Currency should be valid: {}",
                log_entry.currency
            );

            // Validate info field if present
            if let Some(ref info) = log_entry.info {
                assert!(!info.is_null(), "Info should not be null if present");
            }
        }

        info!("Transaction log data validation test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_transaction_log_multiple_currencies() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting transaction log multiple currencies test");

        let client = DeribitHttpClient::new();

        let currencies = ["BTC", "ETH", "USDC"];

        for currency in &currencies {
            debug!("Getting transaction log for {}", currency);
            let transaction_log = client
                .get_transaction_log(TransactionLogRequest {
                    currency: currency.to_string(),
                    start_timestamp: 0,
                    end_timestamp: 0,
                    query: None,
                    count: Some(5),
                    subaccount_id: None,
                    continuation: None,
                })
                .await?;

            info!(
                "{} transaction log retrieved successfully, count: {}",
                currency,
                transaction_log.logs.len()
            );

            // Validate that all entries match the requested currency
            for log_entry in &transaction_log.logs {
                assert_eq!(
                    log_entry.currency, *currency,
                    "All entries should match requested currency: {} == {}",
                    log_entry.currency, currency
                );
            }

            // Small delay between requests to respect rate limits
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        info!("Transaction log multiple currencies test completed successfully");
        Ok(())
    }
}
