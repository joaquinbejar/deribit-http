/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 7/3/26
******************************************************************************/
//! Unit tests for combo books endpoints

use deribit_http::{DeribitHttpClient, HttpConfig};
use serde_json::json;
use url::Url;

/// Helper function to create a test client with mock server
fn create_test_client(server: &mockito::Server) -> DeribitHttpClient {
    let mut server_url = server.url();
    if server_url.ends_with('/') {
        server_url.pop();
    }
    let config = HttpConfig {
        base_url: Url::parse(&server_url).expect("Invalid mock server URL"),
        ..Default::default()
    };
    DeribitHttpClient::with_config(config)
}

/// Helper function for private endpoint tests
fn create_auth_test_client(server: &mockito::ServerGuard) -> DeribitHttpClient {
    unsafe {
        std::env::set_var("DERIBIT_CLIENT_ID", "test_client_id");
        std::env::set_var("DERIBIT_CLIENT_SECRET", "test_client_secret");
        std::env::set_var("DERIBIT_TESTNET", "true");
    }

    let config = HttpConfig {
        base_url: Url::parse(&format!("{}/api/v2", server.url())).unwrap(),
        ..Default::default()
    };

    DeribitHttpClient::with_config(config)
}

async fn create_auth_mock(server: &mut mockito::Server) -> mockito::Mock {
    server
        .mock("GET", "/api/v2/public/auth?grant_type=client_credentials&client_id=test_client_id&client_secret=test_client_secret")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "access_token": "test_access_token",
                "expires_in": 3600,
                "refresh_token": "test_refresh_token",
                "scope": "read",
                "state": "",
                "token_type": "bearer"
            }
        }"#)
        .create_async()
        .await
}

// =========================================================================
// Public Combo Endpoints Tests
// =========================================================================

#[tokio::test]
async fn test_get_combo_details_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "state_timestamp": 1650620605150_u64,
            "state": "active",
            "legs": [
                {"instrument_name": "BTC-PERPETUAL", "amount": -1},
                {"instrument_name": "BTC-29APR22", "amount": 1}
            ],
            "id": "BTC-FS-29APR22_PERP",
            "instrument_id": 27,
            "creation_timestamp": 1650620575000_u64
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"//public/get_combo_details\?combo_id=.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_combo_details("BTC-FS-29APR22_PERP").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let combo = result.unwrap();
    assert_eq!(combo.id, "BTC-FS-29APR22_PERP");
    assert_eq!(combo.instrument_id, 27);
    assert!(combo.is_active());
    assert_eq!(combo.leg_count(), 2);
    assert_eq!(combo.legs[0].instrument_name, "BTC-PERPETUAL");
    assert_eq!(combo.legs[0].amount, -1);
}

#[tokio::test]
async fn test_get_combo_details_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"//public/get_combo_details\?combo_id=.*".to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc": "2.0", "error": {"code": 11050, "message": "combo_not_found"}, "id": 1}"#)
        .create_async()
        .await;

    let result = client.get_combo_details("INVALID-COMBO").await;

    mock.assert_async().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_combo_ids_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [
            "BTC-CS-29APR22-39300_39600",
            "BTC-FS-29APR22_PERP"
        ],
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"//public/get_combo_ids\?currency=BTC.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_combo_ids("BTC", Some("active")).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let combo_ids = result.unwrap();
    assert_eq!(combo_ids.len(), 2);
    assert_eq!(combo_ids[0], "BTC-CS-29APR22-39300_39600");
    assert_eq!(combo_ids[1], "BTC-FS-29APR22_PERP");
}

#[tokio::test]
async fn test_get_combo_ids_empty() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [],
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"//public/get_combo_ids\?currency=ETH.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_combo_ids("ETH", None).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let combo_ids = result.unwrap();
    assert!(combo_ids.is_empty());
}

#[tokio::test]
async fn test_get_combos_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": [
            {
                "state_timestamp": 1650636265101_u64,
                "state": "active",
                "legs": [
                    {"instrument_name": "BTC-29APR22-39300-C", "amount": 1},
                    {"instrument_name": "BTC-29APR22-39600-C", "amount": -1}
                ],
                "id": "BTC-CS-29APR22-39300_39600",
                "instrument_id": 28,
                "creation_timestamp": 1650636235000_u64
            },
            {
                "state_timestamp": 1650620605150_u64,
                "state": "active",
                "legs": [
                    {"instrument_name": "BTC-PERPETUAL", "amount": -1},
                    {"instrument_name": "BTC-29APR22", "amount": 1}
                ],
                "id": "BTC-FS-29APR22_PERP",
                "instrument_id": 27,
                "creation_timestamp": 1650620575000_u64
            }
        ],
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"//public/get_combos\?currency=BTC.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let result = client.get_combos("BTC").await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let combos = result.unwrap();
    assert_eq!(combos.len(), 2);
    assert_eq!(combos[0].id, "BTC-CS-29APR22-39300_39600");
    assert_eq!(combos[1].id, "BTC-FS-29APR22_PERP");
}

#[tokio::test]
async fn test_get_combos_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_test_client(&server);

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"//public/get_combos\?currency=.*".to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc": "2.0", "error": {"code": 10001, "message": "invalid_currency"}, "id": 1}"#)
        .create_async()
        .await;

    let result = client.get_combos("INVALID").await;

    mock.assert_async().await;
    assert!(result.is_err());
}

// =========================================================================
// Private Combo Endpoints Tests
// =========================================================================

#[tokio::test]
async fn test_create_combo_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_auth_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "state_timestamp": 1650960943922_u64,
            "state": "rfq",
            "legs": [
                {"instrument_name": "BTC-29APR22-37500-C", "amount": 1},
                {"instrument_name": "BTC-29APR22-37500-P", "amount": -1}
            ],
            "id": "BTC-REV-29APR22-37500",
            "instrument_id": 52,
            "creation_timestamp": 1650960943000_u64
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/api/v2/private/create_combo\?trades=.*".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let trades = vec![
        deribit_http::model::ComboTrade::buy("BTC-29APR22-37500-C", Some(1.0)),
        deribit_http::model::ComboTrade::sell("BTC-29APR22-37500-P", Some(1.0)),
    ];
    let result = client.create_combo(&trades).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let combo = result.unwrap();
    assert_eq!(combo.id, "BTC-REV-29APR22-37500");
    assert_eq!(combo.instrument_id, 52);
    assert!(combo.is_rfq());
    assert_eq!(combo.leg_count(), 2);
}

#[tokio::test]
async fn test_create_combo_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_auth_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/api/v2/private/create_combo\?trades=.*".to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc": "2.0", "error": {"code": 11051, "message": "invalid_combo_structure"}, "id": 1}"#)
        .create_async()
        .await;

    let trades = vec![deribit_http::model::ComboTrade::buy(
        "INVALID-INSTRUMENT",
        Some(1.0),
    )];
    let result = client.create_combo(&trades).await;

    mock.assert_async().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_leg_prices_success() {
    let mut server = mockito::Server::new_async().await;
    let client = create_auth_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "legs": [
                {"ratio": 1, "instrument_name": "BTC-1NOV24-67000-C", "price": 0.6001, "direction": "buy"},
                {"ratio": 1, "instrument_name": "BTC-1NOV24-66000-C", "price": 0.0001, "direction": "sell"}
            ],
            "amount": 2.0
        },
        "id": 1
    });

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(
                r"/api/v2/private/get_leg_prices\?legs=.*&price=.*".to_string(),
            ),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create_async()
        .await;

    let legs = vec![
        deribit_http::model::LegInput::buy("BTC-1NOV24-67000-C", 2.0),
        deribit_http::model::LegInput::sell("BTC-1NOV24-66000-C", 2.0),
    ];
    let result = client.get_leg_prices(&legs, 0.6).await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.amount, 2.0);
    assert_eq!(response.leg_count(), 2);
    assert_eq!(response.legs[0].instrument_name, "BTC-1NOV24-67000-C");
    assert_eq!(response.legs[0].price, 0.6001);
    assert_eq!(response.legs[0].direction, "buy");
    assert_eq!(response.legs[1].direction, "sell");
}

#[tokio::test]
async fn test_get_leg_prices_error() {
    let mut server = mockito::Server::new_async().await;
    let client = create_auth_test_client(&server);

    let _auth_mock = create_auth_mock(&mut server).await;

    let mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/api/v2/private/get_leg_prices\?legs=.*&price=.*".to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc": "2.0", "error": {"code": 11052, "message": "invalid_leg_structure"}, "id": 1}"#)
        .create_async()
        .await;

    let legs = vec![deribit_http::model::LegInput::buy(
        "INVALID-INSTRUMENT",
        2.0,
    )];
    let result = client.get_leg_prices(&legs, 0.6).await;

    mock.assert_async().await;
    assert!(result.is_err());
}
