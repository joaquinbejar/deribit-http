//! Move positions integration tests
//!
//! Tests for private/move_positions endpoint.

#[cfg(test)]
mod move_positions_tests {
    use deribit_http::DeribitHttpClient;
    use deribit_http::model::request::position::MovePositionTrade;
    use tokio::time::{Duration, Instant};
    use tracing::info;

    /// Test move_positions endpoint behavior
    ///
    /// Moves positions between subaccounts.
    /// Note: This test requires valid subaccount IDs and open positions.
    #[tokio::test]
    #[serial_test::serial]
    #[ignore = "Requires authentication and subaccounts with positions"]
    async fn test_move_positions() -> Result<(), Box<dyn std::error::Error>> {
        let client = DeribitHttpClient::new();

        info!("Testing move_positions");
        let start_time = Instant::now();

        // Note: These are placeholder values - real test would need actual subaccount IDs
        let trades = vec![MovePositionTrade::new("BTC-PERPETUAL", 10.0)];

        let result = client.move_positions("BTC", 1, 2, &trades).await;
        let elapsed = start_time.elapsed();

        match &result {
            Ok(results) => {
                info!(
                    "Move positions succeeded in {:?}: {} trades executed",
                    elapsed,
                    results.len()
                );
                for trade in results {
                    info!(
                        "  {} {} {} @ {} (source: {}, target: {})",
                        trade.direction,
                        trade.amount,
                        trade.instrument_name,
                        trade.price,
                        trade.source_uid,
                        trade.target_uid
                    );
                }
            }
            Err(e) => {
                info!("Move positions failed in {:?}: {:?}", elapsed, e);
            }
        }

        assert!(
            elapsed < Duration::from_secs(30),
            "Request took too long: {:?}",
            elapsed
        );

        info!("test_move_positions completed");
        Ok(())
    }
}
