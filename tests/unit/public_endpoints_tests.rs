use deribit_http::{DeribitHttpClient, HttpConfig};
use mockito;
use serde_json::json;
use tokio;
use url::Url;

/// Helper function to create a test client with mock server
fn create_test_client(server: &mockito::Server) -> DeribitHttpClient {
    let mut server_url = server.url();
    // Remove trailing slash to match real API behavior
    if server_url.ends_with('/') {
        server_url.pop();
    }
    let config = HttpConfig {
        base_url: Url::parse(&server_url).expect("Invalid mock server URL"),
        ..Default::default()
    };
    DeribitHttpClient::with_config(config)
}

#[tokio::test]
async fn test_get_currencies_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [
            {
                "currency": "BTC",
                "currency_long": "Bitcoin",
                "min_confirmations": 1,
                "min_withdrawal_fee": 0.0001,
                "disabled_deposit_address_creation": false,
                "coin_type": "CRYPTO",
                "fee_precision": 4,
                "withdrawal_fee": 0.0005,
                "withdrawal_priorities": [
                    {
                        "name": "very_low",
                        "value": 0.15
                    }
                ]
            }
        ],
        "id": 1
    });

    let mock = server
        .mock("GET", "//public/get_currencies")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_currencies().await;

    mock.assert_async().await;
    if let Err(e) = &result {
        println!("Error: {:?}", e);
    }
    assert!(result.is_ok());
    let currencies = result.unwrap();
    assert_eq!(currencies.len(), 1);
    assert_eq!(currencies[0].currency, "BTC");
    assert_eq!(currencies[0].currency_long, "Bitcoin");
}

#[tokio::test]
async fn test_get_currencies_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock = server
        .mock("GET", "//public/get_currencies")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32602,
                "message": "Invalid params"
            }
        }"#,
        )
        .create_async()
        .await;

    let result = client.get_currencies().await;

    mock.assert_async().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_index_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "btc": 45000.0,
            "edp": 45000.0
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "//public/get_index?currency=BTC")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_index("BTC").await;

    mock.assert_async().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_index_price_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "index_price": 45000.0,
            "estimated_delivery_price": 45000.0
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "//public/get_index_price?index_name=btc_usd")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_index_price("btc_usd").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let index_data = result.unwrap();
    assert_eq!(index_data.index_price, 45000.0);
}

#[tokio::test]
async fn test_get_index_price_names_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [
            "btc_usd",
            "eth_usd",
            "ada_usd"
        ],
        "id": 1
    });

    let mock = server
        .mock("GET", "//public/get_index_price_names")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_index_price_names().await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let names = result.unwrap();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"btc_usd".to_string()));
}

#[tokio::test]
async fn test_get_book_summary_by_currency_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock = server
        .mock("GET", "//public/get_book_summary_by_currency?currency=BTC")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
                "jsonrpc": "2.0",
                "id": 1,
                "result": [
                    {
                        "instrument_name": "BTC-PERPETUAL",
                        "base_currency": "BTC",
                        "quote_currency": "USD",
                        "volume": 1000.0,
                        "volume_usd": 45000000.0,
                        "open_interest": 500.0,
                        "mark_price": 45000.0,
                        "creation_timestamp": 1640995200000
                    }
                ]
            }"#,
        )
        .create_async()
        .await;

    let result = client.get_book_summary_by_currency("BTC", None).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let summaries = result.unwrap();
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].instrument_name, "BTC-PERPETUAL");
}

#[tokio::test]
async fn test_get_instrument_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "tick_size": 0.5,
            "taker_commission": 0.0005,
            "settlement_period": "perpetual",
            "quote_currency": "USD",
            "min_trade_amount": 10.0,
            "maker_commission": 0.0001,
            "kind": "future",
            "is_active": true,
            "instrument_name": "BTC-PERPETUAL",
            "expiration_timestamp": 32503680000000u64,
            "creation_timestamp": 1569888000000u64,
            "contract_size": 10.0,
            "base_currency": "BTC"
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "//public/get_instrument?instrument_name=BTC-PERPETUAL",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_instrument("BTC-PERPETUAL").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let instrument = result.unwrap();
    assert_eq!(instrument.instrument_name, "BTC-PERPETUAL");
    assert_eq!(instrument.base_currency, Some("BTC".to_string()));
}

#[tokio::test]
async fn test_get_server_time_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": 1640995200000u64,
        "id": 1
    });

    let mock = server
        .mock("GET", "//public/get_time")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_server_time().await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let timestamp = result.unwrap();
    assert_eq!(timestamp, 1640995200000u64);
}

#[tokio::test]
async fn test_test_connection_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "version": "1.0.0"
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "//public/test")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.test_connection().await;

    mock.assert_async().await;
    if let Err(e) = &result {
        println!("Error: {:?}", e);
    }
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response, "1.0.0");
}

#[tokio::test]
async fn test_get_ticker_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "instrument_name": "BTC-PERPETUAL",
            "best_bid_price": 44999.0,
            "best_ask_price": 45001.0,
            "best_bid_amount": 1.0,
            "best_ask_amount": 1.0,
            "mark_price": 45000.0,
            "last_price": 45000.0,
            "volume": 1000.0,
            "volume_usd": 45000000.0,
            "open_interest": 500.0,
            "timestamp": 1640995200000u64,
            "state": "open",
            "stats": {
                "volume": 1000.0,
                "volume_usd": 45000000.0
            }
        },
        "id": 1
    });

    let mock = server
        .mock("GET", "//public/ticker?instrument_name=BTC-PERPETUAL")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_ticker("BTC-PERPETUAL").await;

    mock.assert_async().await;
    if let Err(e) = &result {
        println!("Error: {:?}", e);
    }
    assert!(result.is_ok());
    let ticker = result.unwrap();
    assert_eq!(ticker.instrument_name, "BTC-PERPETUAL");
    assert_eq!(ticker.mark_price, 45000.0);
}

#[tokio::test]
async fn test_get_contract_size_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "contract_size": 10.0
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "//public/get_contract_size?instrument_name=BTC-PERPETUAL",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_contract_size("BTC-PERPETUAL").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let contract_size = result.unwrap();
    assert_eq!(contract_size, 10.0);
}

#[tokio::test]
async fn test_get_last_trades_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "trades": [
                {
                    "trade_id": "12345",
                    "instrument_name": "BTC-PERPETUAL",
                    "price": 45000.0,
                    "amount": 1.0,
                    "direction": "buy",
                    "timestamp": 1640995200000u64,
                    "index_price": 45000.0,
                    "trade_seq": 123,
                    "tick_direction": 1,
                    "iv": null
                }
            ],
            "has_more": false
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "//public/get_last_trades_by_instrument?instrument_name=BTC-PERPETUAL&count=10",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client
        .get_last_trades("BTC-PERPETUAL", Some(10), None)
        .await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let trades = result.unwrap();
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].instrument_name, "BTC-PERPETUAL");
    assert_eq!(trades[0].price, 45000.0);
}

#[tokio::test]
async fn test_get_order_book_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "instrument_name": "BTC-PERPETUAL",
            "bids": [[44999.0, 1.0]],
            "asks": [[45001.0, 1.0]],
            "timestamp": 1640995200000u64,
            "change_id": 12345
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            "//public/get_order_book?instrument_name=BTC-PERPETUAL&depth=5",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_order_book("BTC-PERPETUAL", Some(5)).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let order_book = result.unwrap();
    assert_eq!(order_book.instrument_name, "BTC-PERPETUAL");
    assert_eq!(order_book.bids.len(), 1);
    assert_eq!(order_book.asks.len(), 1);
}
