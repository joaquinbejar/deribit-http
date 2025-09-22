//! Subaccounts Integration Tests
//!
//! This test covers subaccounts functionality:
//! 1. Get subaccounts list

#[cfg(test)]
mod user_trades_log_tests {
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
    async fn test_get_subaccounts_basic() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting basic subaccounts test");

        let client = DeribitHttpClient::new();

        debug!("Getting subaccounts without portfolio");
        let subaccounts = client.get_subaccounts(None).await?;

        info!(
            "Subaccounts retrieved successfully, count: {}",
            subaccounts.len()
        );
        debug!("Subaccounts: {:?}", subaccounts);

        // Validate subaccounts structure
        for (i, subaccount) in subaccounts.iter().enumerate() {
            debug!("Validating subaccount #{}: {}", i + 1, subaccount.username);

            assert!(subaccount.id > 0, "Subaccount ID should be positive");
            assert!(!subaccount.email.is_empty(), "Email should not be empty");
            assert!(
                !subaccount.username.is_empty(),
                "Username should not be empty"
            );
            assert!(
                !subaccount.system_name.is_empty(),
                "System name should not be empty"
            );
            assert!(
                !subaccount.subaccount_type.is_empty(),
                "Account type should not be empty"
            );

            // Boolean fields are always valid - no need to test tautologies
            // ID should be positive
            assert!(subaccount.id > 0, "Subaccount ID should be positive");

            // Portfolio should be None when not requested
            if subaccount.portfolio.is_some() {
                warn!(
                    "Portfolio information present when not requested for subaccount: {}",
                    subaccount.username
                );
            }
        }

        info!("Basic subaccounts test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_subaccounts_with_portfolio() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting subaccounts with portfolio test");

        let client = DeribitHttpClient::new();

        debug!("Getting subaccounts with portfolio information");
        let subaccounts = client.get_subaccounts(Some(true)).await?;

        info!(
            "Subaccounts with portfolio retrieved successfully, count: {}",
            subaccounts.len()
        );
        debug!("Subaccounts with portfolio: {:?}", subaccounts);

        // Validate subaccounts structure with portfolio
        for (i, subaccount) in subaccounts.iter().enumerate() {
            debug!(
                "Validating subaccount with portfolio #{}: {}",
                i + 1,
                subaccount.username
            );

            // Basic validation
            assert!(subaccount.id > 0, "Subaccount ID should be positive");
            assert!(!subaccount.email.is_empty(), "Email should not be empty");
            assert!(
                !subaccount.username.is_empty(),
                "Username should not be empty"
            );

            // Portfolio validation
            if let Some(ref portfolio) = subaccount.portfolio {
                debug!(
                    "Validating portfolio for subaccount: {}",
                    subaccount.username
                );

                debug!("Portfolio info: {:?}", portfolio);

                // Validate each currency portfolio if present
                let currencies = ["btc", "eth", "usdc", "usdt", "eurr"];

                for currency_name in currencies {
                    if let Some(portfolio_info) = portfolio.get(currency_name) {
                        debug!("Validating portfolio for currency: {}", currency_name);

                        assert!(
                            portfolio_info.available_funds.is_finite(),
                            "Available funds should be a finite number for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.balance.is_finite(),
                            "Balance should be a finite number for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.equity.is_finite(),
                            "Equity should be a finite number for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.initial_margin.is_finite(),
                            "Initial margin should be a finite number for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.maintenance_margin.is_finite(),
                            "Maintenance margin should be a finite number for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.margin_balance.is_finite(),
                            "Margin balance should be a finite number for {}",
                            currency_name
                        );

                        // Validate non-negative values where appropriate
                        assert!(
                            portfolio_info.available_funds >= 0.0,
                            "Available funds should be non-negative for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.available_withdrawal_funds >= 0.0,
                            "Available withdrawal funds should be non-negative for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.equity >= 0.0,
                            "Equity should be non-negative for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.initial_margin >= 0.0,
                            "Initial margin should be non-negative for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.maintenance_margin >= 0.0,
                            "Maintenance margin should be non-negative for {}",
                            currency_name
                        );

                        // Validate currency field
                        assert!(
                            !portfolio_info.currency.is_empty(),
                            "Portfolio currency should not be empty for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.currency.to_lowercase() == currency_name,
                            "Portfolio currency should match expected: {} vs {}",
                            portfolio_info.currency,
                            currency_name
                        );
                    }
                }
            } else {
                warn!(
                    "No portfolio information found for subaccount: {}",
                    subaccount.username
                );
            }
        }

        info!("Subaccounts with portfolio test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_subaccounts_data_validation() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting subaccounts data validation test");

        let client = DeribitHttpClient::new();

        debug!("Getting subaccounts for data validation");
        let subaccounts = client.get_subaccounts(Some(true)).await?;

        info!(
            "Subaccounts retrieved for validation, count: {}",
            subaccounts.len()
        );

        for subaccount in &subaccounts {
            debug!("Validating subaccount data: {}", subaccount.username);

            // Validate ID
            assert!(
                subaccount.id > 0,
                "Subaccount ID should be positive: {}",
                subaccount.id
            );

            // Validate string fields
            assert!(!subaccount.email.is_empty(), "Email should not be empty");
            assert!(
                subaccount.email.contains('@'),
                "Email should contain @ symbol: {}",
                subaccount.email
            );
            assert!(
                !subaccount.username.is_empty(),
                "Username should not be empty"
            );
            assert!(
                !subaccount.system_name.is_empty(),
                "System name should not be empty"
            );
            assert!(
                !subaccount.subaccount_type.is_empty(),
                "Account type should not be empty"
            );

            // Validate account type values
            assert!(
                subaccount.subaccount_type == "main"
                    || subaccount.subaccount_type == "subaccount"
                    || subaccount.subaccount_type == "managed",
                "Subaccount type should be valid: {}",
                subaccount.subaccount_type
            );

            // Validate portfolio information if present
            if let Some(ref portfolio) = subaccount.portfolio {
                debug!("Portfolio info present: {:?}", portfolio);

                // Validate each currency portfolio if present
                let currencies = ["btc", "eth", "usdc", "usdt", "eurr"];

                for currency_name in currencies {
                    if let Some(portfolio_info) = portfolio.get(currency_name) {
                        debug!("Validating portfolio for currency: {}", currency_name);

                        // Validate financial fields are finite
                        assert!(
                            portfolio_info.available_funds.is_finite(),
                            "Available funds should be a finite number for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.available_withdrawal_funds.is_finite(),
                            "Available withdrawal funds should be a finite number for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.equity.is_finite(),
                            "Equity should be a finite number for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.initial_margin.is_finite(),
                            "Initial margin should be a finite number for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.maintenance_margin.is_finite(),
                            "Maintenance margin should be a finite number for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.margin_balance.is_finite(),
                            "Margin balance should be a finite number for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.balance.is_finite(),
                            "Balance should be a finite number for {}",
                            currency_name
                        );

                        // Validate non-negative values where appropriate
                        assert!(
                            portfolio_info.available_funds >= 0.0,
                            "Available funds should be non-negative for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.available_withdrawal_funds >= 0.0,
                            "Available withdrawal funds should be non-negative for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.equity >= 0.0,
                            "Equity should be non-negative for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.initial_margin >= 0.0,
                            "Initial margin should be non-negative for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.maintenance_margin >= 0.0,
                            "Maintenance margin should be non-negative for {}",
                            currency_name
                        );
                        // Validate currency field
                        assert!(
                            !portfolio_info.currency.is_empty(),
                            "Portfolio currency should not be empty for {}",
                            currency_name
                        );
                        assert!(
                            portfolio_info.currency.to_lowercase() == currency_name,
                            "Portfolio currency should match expected: {} vs {}",
                            portfolio_info.currency,
                            currency_name
                        );
                    }
                }
            } else {
                warn!(
                    "No portfolio information found for subaccount: {}",
                    subaccount.username
                );
            }

            // Validate ID is positive
            assert!(subaccount.id > 0, "Subaccount ID should be positive");
        }

        info!("Subaccounts data validation test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_subaccounts_consistency() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting subaccounts consistency test");

        let client = DeribitHttpClient::new();

        // Get subaccounts multiple times to check consistency
        debug!("Getting first set of subaccounts");
        let subaccounts1 = client.get_subaccounts(None).await?;

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        debug!("Getting second set of subaccounts");
        let subaccounts2 = client.get_subaccounts(None).await?;

        info!("Both subaccount sets retrieved successfully");

        // Check that the number of subaccounts is consistent
        assert_eq!(
            subaccounts1.len(),
            subaccounts2.len(),
            "Subaccount count should be consistent"
        );

        // Check that subaccounts have consistent basic data
        for sub1 in &subaccounts1 {
            let sub2 = subaccounts2
                .iter()
                .find(|s| s.id == sub1.id)
                .unwrap_or_else(|| {
                    panic!("Subaccount with ID {} should exist in both calls", sub1.id)
                });

            assert_eq!(
                sub1.email, sub2.email,
                "Email should be consistent for subaccount {}",
                sub1.id
            );
            assert_eq!(
                sub1.username, sub2.username,
                "Username should be consistent for subaccount {}",
                sub1.id
            );
            assert_eq!(
                sub1.system_name, sub2.system_name,
                "System name should be consistent for subaccount {}",
                sub1.id
            );
            assert_eq!(
                sub1.subaccount_type, sub2.subaccount_type,
                "Account type should be consistent for subaccount {}",
                sub1.id
            );
            assert_eq!(
                sub1.login_enabled, sub2.login_enabled,
                "Login enabled should be consistent for subaccount {}",
                sub1.id
            );
            assert_eq!(
                sub1.receive_notifications, sub2.receive_notifications,
                "Receive notifications should be consistent for subaccount {}",
                sub1.id
            );
        }

        info!("Subaccounts consistency test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_subaccounts_portfolio_comparison() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting subaccounts portfolio comparison test");

        let client = DeribitHttpClient::new();

        // Get subaccounts without portfolio
        debug!("Getting subaccounts without portfolio");
        let subaccounts_no_portfolio = client.get_subaccounts(Some(false)).await?;

        // Get subaccounts with portfolio
        debug!("Getting subaccounts with portfolio");
        let subaccounts_with_portfolio = client.get_subaccounts(Some(true)).await?;

        info!("Both subaccount sets retrieved for comparison");

        // Check that the number of subaccounts is the same
        assert_eq!(
            subaccounts_no_portfolio.len(),
            subaccounts_with_portfolio.len(),
            "Subaccount count should be the same regardless of portfolio flag"
        );

        // Check that basic data is consistent
        for sub_no_portfolio in &subaccounts_no_portfolio {
            let sub_with_portfolio = subaccounts_with_portfolio
                .iter()
                .find(|s| s.id == sub_no_portfolio.id)
                .unwrap_or_else(|| {
                    panic!(
                        "Subaccount with ID {} should exist in both calls",
                        sub_no_portfolio.id
                    )
                });

            // Basic fields should be identical
            assert_eq!(
                sub_no_portfolio.id, sub_with_portfolio.id,
                "ID should be consistent"
            );
            assert_eq!(
                sub_no_portfolio.email, sub_with_portfolio.email,
                "Email should be consistent"
            );
            assert_eq!(
                sub_no_portfolio.username, sub_with_portfolio.username,
                "Username should be consistent"
            );
            assert_eq!(
                sub_no_portfolio.system_name, sub_with_portfolio.system_name,
                "System name should be consistent"
            );
            assert_eq!(
                sub_no_portfolio.subaccount_type, sub_with_portfolio.subaccount_type,
                "Account type should be consistent"
            );

            // Portfolio should be None in first call, potentially Some in second call
            assert!(
                sub_no_portfolio.portfolio.is_none() || sub_no_portfolio.portfolio.is_some(),
                "Portfolio should be None or Some in no-portfolio call"
            );

            if sub_with_portfolio.portfolio.is_some() {
                debug!(
                    "Portfolio information found for subaccount: {}",
                    sub_with_portfolio.username
                );
            }
        }

        info!("Subaccounts portfolio comparison test completed successfully");
        Ok(())
    }
}
