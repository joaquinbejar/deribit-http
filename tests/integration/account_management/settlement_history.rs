//! Settlement history integration tests
//!
//! Tests for private/get_settlement_history_by_currency and
//! private/get_settlement_history_by_instrument endpoints.

#[cfg(test)]
mod settlement_history_tests {
    use deribit_http::DeribitHttpClient;
    use tokio::time::{Duration, Instant};
    use tracing::info;

    /// Test get_settlement_history_by_currency endpoint behavior
    ///
    /// Returns settlement, delivery, and bankruptcy events for a currency.
    #[tokio::test]
    #[serial_test::serial]
    #[ignore = "Requires authentication"]
    async fn test_get_settlement_history_by_currency() -> Result<(), Box<dyn std::error::Error>> {
        let client = DeribitHttpClient::new();

        info!("Testing get_settlement_history_by_currency");
        let start_time = Instant::now();

        let result = client
            .get_settlement_history_by_currency("BTC", None, Some(10), None, None)
            .await;
        let elapsed = start_time.elapsed();

        match &result {
            Ok(response) => {
                info!(
                    "Get settlement history by currency succeeded in {:?}: {} settlements found",
                    elapsed,
                    response.settlements.len()
                );
                if let Some(continuation) = &response.continuation {
                    info!("Continuation token: {}", continuation);
                }
            }
            Err(e) => {
                info!(
                    "Get settlement history by currency failed in {:?}: {:?}",
                    elapsed, e
                );
            }
        }

        assert!(
            elapsed < Duration::from_secs(30),
            "Request took too long: {:?}",
            elapsed
        );

        info!("test_get_settlement_history_by_currency completed");
        Ok(())
    }

    /// Test get_settlement_history_by_instrument endpoint behavior
    ///
    /// Returns settlement, delivery, and bankruptcy events for an instrument.
    #[tokio::test]
    #[serial_test::serial]
    #[ignore = "Requires authentication"]
    async fn test_get_settlement_history_by_instrument() -> Result<(), Box<dyn std::error::Error>> {
        let client = DeribitHttpClient::new();

        info!("Testing get_settlement_history_by_instrument");
        let start_time = Instant::now();

        let result = client
            .get_settlement_history_by_instrument("BTC-PERPETUAL", None, Some(10), None, None)
            .await;
        let elapsed = start_time.elapsed();

        match &result {
            Ok(response) => {
                info!(
                    "Get settlement history by instrument succeeded in {:?}: {} settlements found",
                    elapsed,
                    response.settlements.len()
                );
                if let Some(continuation) = &response.continuation {
                    info!("Continuation token: {}", continuation);
                }
            }
            Err(e) => {
                info!(
                    "Get settlement history by instrument failed in {:?}: {:?}",
                    elapsed, e
                );
            }
        }

        assert!(
            elapsed < Duration::from_secs(30),
            "Request took too long: {:?}",
            elapsed
        );

        info!("test_get_settlement_history_by_instrument completed");
        Ok(())
    }
}
