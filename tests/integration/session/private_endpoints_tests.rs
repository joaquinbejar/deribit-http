//! Integration tests for private endpoints

use deribit_base::prelude::{BuyOrderRequest, EditOrderRequest, SellOrderRequest};
use deribit_http::DeribitHttpClient;
use mockito::Server;

#[cfg(test)]
mod private_endpoints_tests {
    use super::*;

    async fn create_mock_client() -> (mockito::ServerGuard, DeribitHttpClient) {
        let server = Server::new_async().await;
        let config = deribit_http::config::HttpConfig {
            base_url: url::Url::parse(&server.url()).unwrap(),
            timeout: std::time::Duration::from_secs(30),
            user_agent: "test-agent".to_string(),
            max_retries: 3,
            testnet: false,
            credentials: None,
        };
        let client = DeribitHttpClient::with_config(config).unwrap();
        (server, client)
    }

    #[tokio::test]
    async fn test_get_subaccounts() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "/private/get_subaccounts")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"result": [{"id": 123, "username": "test_sub", "email": "test@example.com"}]}"#,
            )
            .create_async()
            .await;

        let result = client.get_subaccounts(None).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_subaccounts_with_portfolio() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "/private/get_subaccounts")
            .match_query(mockito::Matcher::UrlEncoded("with_portfolio".into(), "true".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": [{"id": 123, "username": "test_sub", "email": "test@example.com", "portfolio": {}}]}"#)
            .create_async()
            .await;

        let result = client.get_subaccounts(Some(true)).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_transaction_log() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/get_transaction_log")
            .match_query(mockito::Matcher::UrlEncoded(
                "currency".into(),
                "BTC".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"logs": [{"id": 1, "type": "deposit", "amount": 0.1}]}}"#)
            .create_async()
            .await;

        let result = client
            .get_transaction_log("BTC", None, None, None, None)
            .await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_deposits() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/get_deposits")
            .match_query(mockito::Matcher::UrlEncoded("currency".into(), "BTC".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"data": [{"transaction_id": "123", "amount": 0.1, "currency": "BTC"}]}}"#)
            .create_async()
            .await;

        let result = client.get_deposits("BTC", None, None).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_withdrawals() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/get_withdrawals")
            .match_query(mockito::Matcher::UrlEncoded("currency".into(), "BTC".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"data": [{"transaction_id": "456", "amount": 0.05, "currency": "BTC"}]}}"#)
            .create_async()
            .await;

        let result = client.get_withdrawals("BTC", None, None).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_submit_transfer_to_subaccount() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/submit_transfer_to_subaccount")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("currency".into(), "BTC".into()),
                mockito::Matcher::UrlEncoded("amount".into(), "0.001".into()),
                mockito::Matcher::UrlEncoded("destination".into(), "123".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"id": "transfer_123", "type": "subaccount"}}"#)
            .create_async()
            .await;

        let result = client
            .submit_transfer_to_subaccount("BTC", 0.001, 123)
            .await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_submit_transfer_to_user() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/submit_transfer_to_user")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("currency".into(), "ETH".into()),
                mockito::Matcher::UrlEncoded("amount".into(), "0.1".into()),
                mockito::Matcher::UrlEncoded("destination".into(), "user123".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"id": "transfer_456", "type": "user"}}"#)
            .create_async()
            .await;

        let result = client.submit_transfer_to_user("ETH", 0.1, "user123").await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_buy_order() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "/private/buy")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("instrument_name".into(), "BTC-PERPETUAL".into()),
                mockito::Matcher::UrlEncoded("amount".into(), "10".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"order": {"order_id": "order_123", "order_state": "open"}}}"#)
            .create_async()
            .await;

        let request = BuyOrderRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            amount: 10.0,
            type_: None,
            label: None,
            price: None,
            time_in_force: None,
            post_only: None,
            reduce_only: None,
        };

        let result = client.buy_order(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_sell_order() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/sell")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("instrument_name".into(), "BTC-PERPETUAL".into()),
                mockito::Matcher::UrlEncoded("amount".into(), "5".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"order": {"order_id": "order_456", "order_state": "open"}}}"#)
            .create_async()
            .await;

        let request = SellOrderRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            amount: 5.0,
            type_: None,
            label: None,
            price: None,
            time_in_force: None,
            post_only: None,
            reduce_only: None,
        };

        let result = client.sell_order(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_cancel_order() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/cancel")
            .match_query(mockito::Matcher::UrlEncoded(
                "order_id".into(),
                "order_123".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"order_id": "order_123", "order_state": "cancelled"}}"#)
            .create_async()
            .await;

        let result = client.cancel_order("order_123").await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_cancel_all() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/cancel_all")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": 5}"#)
            .create_async()
            .await;

        let result = client.cancel_all().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_cancel_all_by_currency() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/cancel_all_by_currency")
            .match_query(mockito::Matcher::UrlEncoded(
                "currency".into(),
                "BTC".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": 3}"#)
            .create_async()
            .await;

        let result = client.cancel_all_by_currency("BTC").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_cancel_all_by_instrument() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/cancel_all_by_instrument")
            .match_query(mockito::Matcher::UrlEncoded(
                "instrument_name".into(),
                "BTC-PERPETUAL".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": 2}"#)
            .create_async()
            .await;

        let result = client.cancel_all_by_instrument("BTC-PERPETUAL").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_account_summary() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "/private/get_account_summary")
            .match_query(mockito::Matcher::UrlEncoded(
                "currency".into(),
                "BTC".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"balance": 1.5, "available_funds": 1.2, "currency": "BTC"}}"#)
            .create_async()
            .await;

        let result = client.get_account_summary("BTC", None).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_positions() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/get_positions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": [{"instrument_name": "BTC-PERPETUAL", "size": 10.0, "direction": "buy"}]}"#)
            .create_async()
            .await;

        let result = client.get_positions(None, None, None).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_edit_order() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/edit")
            .match_query(mockito::Matcher::UrlEncoded(
                "order_id".into(),
                "order_123".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"order": {"order_id": "order_123", "order_state": "open"}}}"#)
            .create_async()
            .await;

        let request = EditOrderRequest {
            order_id: "order_123".to_string(),
            amount: None,
            price: None,
            post_only: None,
            reduce_only: None,
            time_in_force: None,
        };

        let result = client.edit_order(request).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_open_orders() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/get_open_orders")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": [{"order_id": "order_123", "order_state": "open", "instrument_name": "BTC-PERPETUAL"}]}"#)
            .create_async()
            .await;

        let result = client.get_open_orders(None, None).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_open_orders_by_currency() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/get_open_orders_by_currency")
            .match_query(mockito::Matcher::UrlEncoded(
                "currency".into(),
                "BTC".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": [{"order_id": "order_456", "order_state": "open"}]}"#)
            .create_async()
            .await;

        let result = client.get_open_orders_by_currency("BTC", None, None).await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_open_orders_by_instrument() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/get_open_orders_by_instrument")
            .match_query(mockito::Matcher::UrlEncoded(
                "instrument_name".into(),
                "BTC-PERPETUAL".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": [{"order_id": "order_789", "order_state": "open"}]}"#)
            .create_async()
            .await;

        let result = client
            .get_open_orders_by_instrument("BTC-PERPETUAL", None)
            .await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_order_state() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/get_order_state")
            .match_query(mockito::Matcher::UrlEncoded("order_id".into(), "order_123".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"order_id": "order_123", "order_state": "filled", "filled_amount": 10.0}}"#)
            .create_async()
            .await;

        let result = client.get_order_state("order_123").await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_user_trades_by_instrument() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "//private/get_user_trades_by_instrument")
            .match_query(mockito::Matcher::UrlEncoded("instrument_name".into(), "BTC-PERPETUAL".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"trades": [{"trade_id": "trade_123", "price": 50000.0, "amount": 1.0}]}}"#)
            .create_async()
            .await;

        let result = client
            .get_user_trades_by_instrument("BTC-PERPETUAL", None, None, None, None, None)
            .await;
        assert!(result.is_ok());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_error_handling_unauthorized() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "/private/get_account_summary")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": {"code": 401, "message": "Unauthorized"}}"#)
            .create_async()
            .await;

        let result = client.get_account_summary("BTC", None).await;
        assert!(result.is_err());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_error_handling_invalid_instrument() {
        let (mut server, client) = create_mock_client().await;

        let mock = server
            .mock("GET", "/private/buy")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": {"code": 10009, "message": "Invalid instrument"}}"#)
            .create_async()
            .await;

        let request = BuyOrderRequest {
            instrument_name: "INVALID-INSTRUMENT".to_string(),
            amount: 10.0,
            type_: None,
            label: None,
            price: None,
            time_in_force: None,
            post_only: None,
            reduce_only: None,
        };

        let result = client.buy_order(request).await;
        assert!(result.is_err());

        mock.assert_async().await;
    }
}
