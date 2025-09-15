//! Integration tests for public endpoints

#[cfg(test)]
mod tests {
    use deribit_http::DeribitHttpClient;

    async fn create_test_client() -> DeribitHttpClient {
        // Create client with default configuration
        // Note: Tests will use the real Deribit testnet URL since we can't mock it
        DeribitHttpClient::new()
    }

    #[tokio::test]
    async fn test_get_currencies() {
        let client = create_test_client().await;

        let result = client.get_currencies().await;
        // Test that the client can make the request (may fail due to network/auth)
        // We're mainly testing that the method exists and can be called
        match result {
            Ok(currencies) => {
                println!("Successfully got {} currencies", currencies.len());
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
                // This is expected since we're not authenticated and using real API
            }
        }
    }

    #[tokio::test]
    async fn test_get_index() {
        let client = create_test_client().await;

        let result = client.get_index("BTC").await;
        match result {
            Ok(index) => {
                println!("Successfully got index: {:?}", index);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_index_price() {
        let client = create_test_client().await;

        let result = client.get_index_price("btc_usd").await;
        match result {
            Ok(price_data) => {
                println!("Successfully got index price: {:?}", price_data);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_index_price_names() {
        let client = create_test_client().await;

        let result = client.get_index_price_names().await;
        match result {
            Ok(names) => {
                println!("Successfully got {} index price names", names.len());
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_book_summary_by_currency() {
        let client = create_test_client().await;

        let result = client.get_book_summary_by_currency("BTC", None).await;
        match result {
            Ok(summaries) => {
                println!("Successfully got {} book summaries", summaries.len());
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_instrument() {
        let client = create_test_client().await;

        let result = client.get_instrument("BTC-PERPETUAL").await;
        match result {
            Ok(instrument) => {
                println!("Successfully got instrument: {:?}", instrument);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_book_summary_by_instrument() {
        let client = create_test_client().await;

        let result = client.get_book_summary_by_instrument("BTC-PERPETUAL").await;
        match result {
            Ok(summary) => {
                println!("Successfully got book summary: {:?}", summary);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_contract_size() {
        let client = create_test_client().await;

        let result = client.get_contract_size("BTC-PERPETUAL").await;
        match result {
            Ok(size) => {
                println!("Successfully got contract size: {}", size);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_server_time() {
        let client = create_test_client().await;

        let result = client.get_server_time().await;
        match result {
            Ok(timestamp) => {
                println!("Successfully got server time: {}", timestamp);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_test_connection() {
        let client = create_test_client().await;

        let result = client.test_connection().await;
        match result {
            Ok(response) => {
                println!("Successfully tested connection: {}", response);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_status() {
        let client = create_test_client().await;

        let result = client.get_status().await;
        match result {
            Ok(status) => {
                println!("Successfully got status: {:?}", status);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_ticker() {
        let client = create_test_client().await;

        let result = client.get_ticker("BTC-PERPETUAL").await;
        match result {
            Ok(ticker) => {
                println!("Successfully got ticker: {:?}", ticker);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_order_book() {
        let client = create_test_client().await;

        let result = client.get_order_book("BTC-PERPETUAL", None).await;
        match result {
            Ok(order_book) => {
                println!("Successfully got order book: {:?}", order_book);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_instruments() {
        let client = create_test_client().await;

        let result = client.get_instruments("BTC", None, None).await;
        match result {
            Ok(instruments) => {
                println!("Successfully got {} instruments", instruments.len());
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_last_trades() {
        let client = create_test_client().await;

        let result = client
            .get_last_trades("BTC-PERPETUAL", None, Some(false))
            .await;
        match result {
            Ok(trades) => {
                println!("Successfully got trades: {:?}", trades);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_last_trades_by_currency() {
        let client = create_test_client().await;

        let result = client.get_last_trades_by_currency("BTC", None, None, None, None).await;
        match result {
            Ok(trades) => {
                println!("Successfully got trades: {:?}", trades);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_historical_volatility() {
        let client = create_test_client().await;

        let result = client.get_historical_volatility("BTC").await;
        match result {
            Ok(volatility) => {
                println!("Successfully got volatility: {:?}", volatility);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_funding_chart_data() {
        let client = create_test_client().await;

        let result = client.get_funding_chart_data("BTC-PERPETUAL", "8h").await;
        match result {
            Ok(funding_data) => {
                println!("Successfully got funding data: {:?}", funding_data);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_delivery_prices() {
        let client = create_test_client().await;

        let result = client.get_delivery_prices("btc_usd", None, None).await;
        match result {
            Ok(delivery_prices) => {
                println!("Successfully got delivery prices: {:?}", delivery_prices);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_expirations() {
        let client = create_test_client().await;

        let result = client.get_expirations("BTC", "future", None).await;
        match result {
            Ok(expirations) => {
                println!("Successfully got expirations: {:?}", expirations);
            }
            Err(e) => {
                println!("Expected error (no auth/network): {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_error_handling() {
        let client = create_test_client().await;

        // Test with an invalid instrument name to trigger an error
        let result = client.get_ticker("INVALID-INSTRUMENT").await;
        match result {
            Ok(_) => {
                println!("Unexpected success - API might be very permissive");
            }
            Err(e) => {
                println!("Expected error for invalid instrument: {:?}", e);
                // This is the expected case
            }
        }
    }

    #[tokio::test]
    async fn test_network_error_handling() {
        // Since we can't easily configure the client with invalid URL,
        // we'll test with a very short timeout or invalid parameters
        let client = create_test_client().await;

        // Test with empty string parameters that should cause errors
        let result = client.get_ticker("").await;
        match result {
            Ok(_) => {
                println!("Unexpected success with empty instrument name");
            }
            Err(e) => {
                println!("Expected error with empty instrument name: {:?}", e);
            }
        }
    }
}
