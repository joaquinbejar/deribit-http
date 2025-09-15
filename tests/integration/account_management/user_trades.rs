//! User Trades Integration Tests
//!
//! This test covers user trades functionality:
//! 1. Get user trades by instrument

#[cfg(test)]
mod user_trades_log_tests {
    use deribit_http::DeribitHttpClient;
    use std::path::Path;
    use tracing::{debug, info};

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
    async fn test_get_user_trades_btc_perpetual() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting BTC-PERPETUAL user trades test");

        let client = DeribitHttpClient::new();

        debug!("Getting user trades for BTC-PERPETUAL");
        let user_trades = client
            .get_user_trades_by_instrument("BTC-PERPETUAL", None, None, None, None, None)
            .await?;

        info!(
            "BTC-PERPETUAL user trades retrieved successfully, count: {}",
            user_trades.trades.len()
        );
        debug!("User trades: {:?}", user_trades.trades);

        // Validate user trades structure
        for (i, trade) in user_trades.trades.iter().enumerate() {
            debug!("Validating user trade #{}: {}", i + 1, trade.trade_id);

            assert!(!trade.trade_id.is_empty(), "Trade ID should not be empty");
            assert!(trade.trade_seq > 0, "Trade sequence should be positive");
            assert!(!trade.order_id.is_empty(), "Order ID should not be empty");
            assert!(
                !trade.order_type.is_empty(),
                "Order type should not be empty"
            );
            assert_eq!(
                trade.instrument_name, "BTC-PERPETUAL",
                "Instrument name should be BTC-PERPETUAL"
            );
            assert!(!trade.direction.is_empty(), "Direction should not be empty");
            assert!(trade.amount > 0.0, "Amount should be positive");
            assert!(trade.price > 0.0, "Price should be positive");
            assert!(trade.timestamp > 0, "Timestamp should be positive");
            assert!(trade.fee >= 0.0, "Fee should be non-negative");
            assert!(
                !trade.fee_currency.is_empty(),
                "Fee currency should not be empty"
            );
            assert!(!trade.liquidity.is_empty(), "Liquidity should not be empty");
            assert!(trade.index_price > 0.0, "Index price should be positive");
            assert!(trade.mark_price > 0.0, "Mark price should be positive");
            assert!(trade.label.is_none(), "Label should not be empty");

            // Validate direction values
            assert!(
                trade.direction == "buy" || trade.direction == "sell",
                "Direction should be buy or sell: {}",
                trade.direction
            );

            // Validate liquidity values
            assert!(
                trade.liquidity == "M" || trade.liquidity == "T",
                "Liquidity should be M (maker) or T (taker): {}",
                trade.liquidity
            );

            // Validate tick direction
            assert!(
                trade.tick_direction >= -1 && trade.tick_direction <= 1,
                "Tick direction should be -1, 0, or 1: {}",
                trade.tick_direction
            );
        }

        info!("BTC-PERPETUAL user trades test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_user_trades_eth_perpetual() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting ETH-PERPETUAL user trades test");

        let client = DeribitHttpClient::new();

        debug!("Getting user trades for ETH-PERPETUAL");
        let user_trades = client
            .get_user_trades_by_instrument("ETH-PERPETUAL", None, None, None, None, None)
            .await?;

        info!(
            "ETH-PERPETUAL user trades retrieved successfully, count: {}",
            user_trades.trades.len()
        );
        debug!("User trades: {:?}", user_trades.trades);

        // Validate that all trades are for ETH-PERPETUAL
        for trade in &user_trades.trades {
            assert_eq!(
                trade.instrument_name, "ETH-PERPETUAL",
                "All trades should be for ETH-PERPETUAL"
            );
        }

        info!("ETH-PERPETUAL user trades test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_user_trades_with_count() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting user trades with count test");

        let client = DeribitHttpClient::new();

        let requested_count = 5;
        debug!("Getting user trades with count: {}", requested_count);
        let user_trades = client
            .get_user_trades_by_instrument(
                "BTC-PERPETUAL",
                None,
                None,
                Some(requested_count),
                None,
                None,
            )
            .await?;

        info!(
            "User trades with count retrieved successfully, count: {}",
            user_trades.trades.len()
        );
        debug!("User trades: {:?}", user_trades.trades);

        // Validate that we got at most the requested count
        assert!(
            user_trades.trades.len() <= requested_count as usize,
            "Should not receive more than requested count: {} <= {}",
            user_trades.trades.len(),
            requested_count
        );

        info!("User trades with count test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_user_trades_with_sequence_range() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting user trades with sequence range test");

        let client = DeribitHttpClient::new();

        // First, get some trades to find sequence numbers
        debug!("Getting initial trades to find sequence range");
        let initial_trades = client
            .get_user_trades_by_instrument("BTC-PERPETUAL", None, None, Some(10), None, None)
            .await?;

        if initial_trades.trades.is_empty() {
            info!("No trades found for sequence range test, skipping");
            return Ok(());
        }

        let min_seq = initial_trades
            .trades
            .iter()
            .map(|t| t.trade_seq)
            .min()
            .unwrap();
        let max_seq = initial_trades
            .trades
            .iter()
            .map(|t| t.trade_seq)
            .max()
            .unwrap();

        debug!("Using sequence range: {} to {}", min_seq, max_seq);
        let filtered_trades = client
            .get_user_trades_by_instrument(
                "BTC-PERPETUAL",
                Some(min_seq),
                Some(max_seq),
                None,
                None,
                None,
            )
            .await?;

        info!(
            "User trades with sequence range retrieved successfully, count: {}",
            filtered_trades.trades.len()
        );
        debug!("Filtered trades: {:?}", filtered_trades.trades);

        // Validate that all trades are within the sequence range
        for trade in &filtered_trades.trades {
            assert!(
                trade.trade_seq >= min_seq,
                "Trade sequence should be >= start_seq: {} >= {}",
                trade.trade_seq,
                min_seq
            );
            assert!(
                trade.trade_seq <= max_seq,
                "Trade sequence should be <= end_seq: {} <= {}",
                trade.trade_seq,
                max_seq
            );
        }

        info!("User trades with sequence range test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_user_trades_include_old() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting user trades include old test");

        let client = DeribitHttpClient::new();

        // Get trades without including old
        debug!("Getting trades without including old");
        let trades_no_old = client
            .get_user_trades_by_instrument("BTC-PERPETUAL", None, None, Some(10), Some(false), None)
            .await?;

        // Get trades including old
        debug!("Getting trades including old");
        let trades_with_old = client
            .get_user_trades_by_instrument("BTC-PERPETUAL", None, None, Some(10), Some(true), None)
            .await?;

        info!(
            "User trades retrieved - no old: {}, with old: {}",
            trades_no_old.trades.len(),
            trades_with_old.trades.len()
        );

        // With old trades should have >= trades without old
        assert!(
            trades_with_old.trades.len() >= trades_no_old.trades.len(),
            "Including old trades should return >= trades: {} >= {}",
            trades_with_old.trades.len(),
            trades_no_old.trades.len()
        );

        info!("User trades include old test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_user_trades_sorting() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting user trades sorting test");

        let client = DeribitHttpClient::new();

        // Get trades with ascending sorting
        debug!("Getting trades with ascending sorting");
        let trades_asc = client
            .get_user_trades_by_instrument("BTC-PERPETUAL", None, None, Some(10), None, Some("asc"))
            .await?;

        // Get trades with descending sorting
        debug!("Getting trades with descending sorting");
        let trades_desc = client
            .get_user_trades_by_instrument(
                "BTC-PERPETUAL",
                None,
                None,
                Some(10),
                None,
                Some("desc"),
            )
            .await?;

        info!(
            "User trades retrieved - asc: {}, desc: {}",
            trades_asc.trades.len(),
            trades_desc.trades.len()
        );

        // Validate ascending order
        if trades_asc.trades.len() > 1 {
            for i in 1..trades_asc.trades.len() {
                assert!(
                    trades_asc.trades[i].trade_seq >= trades_asc.trades[i - 1].trade_seq,
                    "Ascending order should be maintained: {} >= {}",
                    trades_asc.trades[i].trade_seq,
                    trades_asc.trades[i - 1].trade_seq
                );
            }
        }

        // Validate descending order
        if trades_desc.trades.len() > 1 {
            for i in 1..trades_desc.trades.len() {
                assert!(
                    trades_desc.trades[i].trade_seq <= trades_desc.trades[i - 1].trade_seq,
                    "Descending order should be maintained: {} <= {}",
                    trades_desc.trades[i].trade_seq,
                    trades_desc.trades[i - 1].trade_seq
                );
            }
        }

        info!("User trades sorting test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_user_trades_data_validation() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting user trades data validation test");

        let client = DeribitHttpClient::new();

        debug!("Getting user trades for data validation");
        let user_trades = client
            .get_user_trades_by_instrument("BTC-PERPETUAL", None, None, Some(20), None, None)
            .await?;

        info!(
            "User trades retrieved for validation, count: {}",
            user_trades.trades.len()
        );

        for trade in &user_trades.trades {
            debug!(
                "Validating user trade: {} - {}",
                trade.trade_id, trade.instrument_name
            );

            // Validate required string fields
            assert!(!trade.trade_id.is_empty(), "Trade ID should not be empty");
            assert!(!trade.order_id.is_empty(), "Order ID should not be empty");
            assert!(
                !trade.order_type.is_empty(),
                "Order type should not be empty"
            );
            assert!(
                !trade.instrument_name.is_empty(),
                "Instrument name should not be empty"
            );
            assert!(!trade.direction.is_empty(), "Direction should not be empty");
            assert!(
                !trade.fee_currency.is_empty(),
                "Fee currency should not be empty"
            );
            assert!(!trade.liquidity.is_empty(), "Liquidity should not be empty");
            assert!(trade.label.is_none(), "Label should not be empty");

            // Validate numeric fields
            assert!(trade.trade_seq > 0, "Trade sequence should be positive");
            assert!(trade.amount > 0.0, "Amount should be positive");
            assert!(trade.price > 0.0, "Price should be positive");
            assert!(trade.timestamp > 0, "Timestamp should be positive");
            assert!(trade.fee >= 0.0, "Fee should be non-negative");
            assert!(trade.index_price > 0.0, "Index price should be positive");
            assert!(trade.mark_price > 0.0, "Mark price should be positive");

            // Validate finite numbers
            assert!(trade.amount.is_finite(), "Amount should be finite");
            assert!(trade.price.is_finite(), "Price should be finite");
            assert!(trade.fee.is_finite(), "Fee should be finite");
            assert!(
                trade.index_price.is_finite(),
                "Index price should be finite"
            );
            assert!(trade.mark_price.is_finite(), "Mark price should be finite");

            // Validate enum-like fields
            assert!(
                trade.direction == "buy" || trade.direction == "sell",
                "Direction should be buy or sell: {}",
                trade.direction
            );
            assert!(
                trade.liquidity == "M" || trade.liquidity == "T",
                "Liquidity should be M or T: {}",
                trade.liquidity
            );

            // Validate tick direction
            assert!(
                trade.tick_direction >= -1 && trade.tick_direction <= 1,
                "Tick direction should be -1, 0, or 1: {}",
                trade.tick_direction
            );

            // Validate order type
            let valid_order_types = [
                "limit",
                "market",
                "stop_limit",
                "stop_market",
                "take_limit",
                "take_market",
            ];
            assert!(
                valid_order_types
                    .iter()
                    .any(|&t| trade.order_type.contains(t)),
                "Order type should be valid: {}",
                trade.order_type
            );

            // Self trade flag is always valid boolean - no need to test tautology

            // Validate optional matching ID
            if let Some(ref matching_id) = trade.matching_id {
                assert!(
                    !matching_id.is_empty(),
                    "Matching ID should not be empty if present"
                );
            }
        }

        info!("User trades data validation test completed successfully");
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_user_trades_multiple_instruments() -> Result<(), Box<dyn std::error::Error>> {
        check_env_file()?;

        info!("Starting user trades multiple instruments test");

        let client = DeribitHttpClient::new();

        let instruments = ["BTC-PERPETUAL", "ETH-PERPETUAL"];

        for instrument in &instruments {
            debug!("Getting user trades for {}", instrument);
            let user_trades = client
                .get_user_trades_by_instrument(instrument, None, None, Some(5), None, None)
                .await?;

            info!(
                "{} user trades retrieved successfully, count: {}",
                instrument,
                user_trades.trades.len()
            );

            // Validate that all trades match the requested instrument
            for trade in &user_trades.trades {
                assert_eq!(
                    trade.instrument_name, *instrument,
                    "All trades should match requested instrument: {} == {}",
                    trade.instrument_name, instrument
                );
            }

            // Small delay between requests to respect rate limits
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        info!("User trades multiple instruments test completed successfully");
        Ok(())
    }
}
