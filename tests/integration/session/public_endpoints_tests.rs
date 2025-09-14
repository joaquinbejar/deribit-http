//! Integration tests for public endpoints

#[cfg(test)]
mod tests {
    use deribit_http::DeribitHttpClient;
    use mockito::Server;

    async fn create_mock_client() -> (mockito::ServerGuard, DeribitHttpClient) {
        let server = Server::new_async().await;
        let _config = deribit_http::config::HttpConfig {
            base_url: url::Url::parse(&server.url()).unwrap(),
            timeout: std::time::Duration::from_secs(30),
            user_agent: "test-agent".to_string(),
            max_retries: 3,
            testnet: false,
            credentials: None,
        };
        let client = DeribitHttpClient::default();
        (server, client)
    }

    #[tokio::test]
    async fn test_get_currencies() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_currencies")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": [{"currency": "BTC", "currency_long": "Bitcoin", "min_confirmations": 1}]}"#)
            .create_async()
            .await;

        let result = client.get_currencies().await;
        assert!(result.is_ok());
        let currencies = result.unwrap();
        assert_eq!(currencies.len(), 1);
        assert_eq!(currencies[0].currency, "BTC");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_index() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_index")
            .match_query(mockito::Matcher::UrlEncoded(
                "currency".into(),
                "BTC".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"BTC": 50000.0}}"#)
            .create_async()
            .await;

        let result = client.get_index("BTC").await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_index_price() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_index_price")
            .match_query(mockito::Matcher::UrlEncoded(
                "index_name".into(),
                "btc_usd".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"result": {"index_price": 50000.0, "estimated_delivery_price": 50000.0}}"#,
            )
            .create_async()
            .await;

        let result = client.get_index_price("btc_usd").await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_index_price_names() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_index_price_names")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": ["btc_usd", "eth_usd"]}"#)
            .create_async()
            .await;

        let result = client.get_index_price_names().await;
        assert!(result.is_ok());
        let names = result.unwrap();
        assert_eq!(names.len(), 2);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_book_summary_by_currency() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_book_summary_by_currency")
            .match_query(mockito::Matcher::UrlEncoded("currency".into(), "BTC".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": [{"instrument_name": "BTC-PERPETUAL", "bid_price": 49000.0, "ask_price": 49100.0}]}"#)
            .create_async()
            .await;

        let result = client.get_book_summary_by_currency("BTC", None).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_instrument() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_instrument")
            .match_query(mockito::Matcher::UrlEncoded("instrument_name".into(), "BTC-PERPETUAL".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"instrument_name": "BTC-PERPETUAL", "kind": "future", "currency": "BTC"}}"#)
            .create_async()
            .await;

        let result = client.get_instrument("BTC-PERPETUAL").await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_book_summary_by_instrument() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_book_summary_by_instrument")
            .match_query(mockito::Matcher::UrlEncoded("instrument_name".into(), "BTC-PERPETUAL".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"instrument_name": "BTC-PERPETUAL", "bid_price": 49000.0, "ask_price": 49100.0}}"#)
            .create_async()
            .await;

        let result = client.get_book_summary_by_instrument("BTC-PERPETUAL").await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_contract_size() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_contract_size")
            .match_query(mockito::Matcher::UrlEncoded(
                "instrument_name".into(),
                "BTC-PERPETUAL".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": 10.0}"#)
            .create_async()
            .await;

        let result = client.get_contract_size("BTC-PERPETUAL").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10.0);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_server_time() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_time")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": 1609459200000}"#)
            .create_async()
            .await;

        let result = client.get_server_time().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1609459200000);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_test_connection() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": "ok"}"#)
            .create_async()
            .await;

        let result = client.test_connection().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "ok");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_status() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/status")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"locked_currencies": []}}"#)
            .create_async()
            .await;

        let result = client.get_status().await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_ticker() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/ticker")
            .match_query(mockito::Matcher::UrlEncoded(
                "instrument_name".into(),
                "BTC-PERPETUAL".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"instrument_name": "BTC-PERPETUAL", "last_price": 50000.0}}"#)
            .create_async()
            .await;

        let result = client.get_ticker("BTC-PERPETUAL").await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_order_book() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_order_book")
            .match_query(mockito::Matcher::UrlEncoded(
                "instrument_name".into(),
                "BTC-PERPETUAL".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"result": {"instrument_name": "BTC-PERPETUAL", "bids": [], "asks": []}}"#,
            )
            .create_async()
            .await;

        let result = client.get_order_book("BTC-PERPETUAL", None).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_instruments() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_instruments")
            .match_query(mockito::Matcher::UrlEncoded(
                "currency".into(),
                "BTC".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": [{"instrument_name": "BTC-PERPETUAL", "kind": "future"}]}"#)
            .create_async()
            .await;

        let result = client.get_instruments("BTC", None, Some(false)).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_last_trades() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_last_trades")
            .match_query(mockito::Matcher::UrlEncoded(
                "instrument_name".into(),
                "BTC-PERPETUAL".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"trades": [{"trade_id": "123", "price": 50000.0}]}}"#)
            .create_async()
            .await;

        let result = client
            .get_last_trades("BTC-PERPETUAL", None, Some(false))
            .await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_historical_volatility() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_historical_volatility")
            .match_query(mockito::Matcher::UrlEncoded(
                "currency".into(),
                "BTC".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": [[1609459200000, 0.5]]}"#)
            .create_async()
            .await;

        let result = client.get_historical_volatility("BTC").await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_funding_chart_data() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_funding_chart_data")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("instrument_name".into(), "BTC-PERPETUAL".into()),
                mockito::Matcher::UrlEncoded("length".into(), "8h".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"current_interest": 0.001, "data": []}}"#)
            .create_async()
            .await;

        let result = client.get_funding_chart_data("BTC-PERPETUAL", "8h").await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_delivery_prices() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_delivery_prices")
            .match_query(mockito::Matcher::UrlEncoded(
                "index_name".into(),
                "btc_usd".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"result": {"data": [{"date": "2021-01-01", "delivery_price": 50000.0}]}}"#,
            )
            .create_async()
            .await;

        let result = client.get_delivery_prices("btc_usd", None, None).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_expirations() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_expirations")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("currency".into(), "BTC".into()),
                mockito::Matcher::UrlEncoded("kind".into(), "future".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"expirations": ["2024-12-27"]}}"#)
            .create_async()
            .await;

        let result = client.get_expirations("BTC", "future", None).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_error_handling() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//public/get_currencies")
            .with_status(500)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": {"code": 500, "message": "Internal server error"}}"#)
            .create_async()
            .await;

        let result = client.get_currencies().await;
        assert!(result.is_err());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_network_error_handling() {
        let _config = deribit_http::config::HttpConfig {
            base_url: url::Url::parse("http://invalid-url-that-does-not-exist.com").unwrap(),
            timeout: std::time::Duration::from_secs(30),
            user_agent: "test-agent".to_string(),
            max_retries: 3,
            testnet: false,
            credentials: None,
        };
        let client = DeribitHttpClient::default();

        let result = client.get_currencies().await;
        assert!(result.is_err());
    }
}
