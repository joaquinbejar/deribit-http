/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 7/3/26
******************************************************************************/
//! Integration tests for Block RFQ endpoints
//!
//! These tests require authentication and are ignored by default.
//! Run with: cargo test --test integration -- --ignored

#[allow(unused_imports)]
use deribit_http::DeribitHttpClient;
#[allow(unused_imports)]
use deribit_http::model::response::{
    BlockRfqQuote, BlockRfqRole, BlockRfqState, BlockRfqTradesResponse, BlockRfqsResponse,
};

#[allow(dead_code)]
fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path("tests/integration/.env")?;
    Ok(())
}

/// Test public get_block_rfq_trades endpoint
#[tokio::test]
#[ignore = "Integration test - requires network"]
async fn test_get_block_rfq_trades() {
    let client = DeribitHttpClient::new();

    let result: Result<BlockRfqTradesResponse, _> = client
        .get_block_rfq_trades(Some("BTC"), Some(10), None)
        .await;

    // This is a public endpoint, should work without auth
    assert!(
        result.is_ok(),
        "get_block_rfq_trades failed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    println!("Found {} Block RFQ trades", response.len());
}

/// Test private get_block_rfqs endpoint
#[tokio::test]
#[ignore = "Requires authentication"]
#[serial_test::serial]
async fn test_get_block_rfqs() {
    check_env_file().expect("Missing .env file");
    let client = DeribitHttpClient::new();

    let result: Result<BlockRfqsResponse, _> = client
        .get_block_rfqs(Some(10), None, None, None, None, None)
        .await;

    assert!(result.is_ok(), "get_block_rfqs failed: {:?}", result.err());

    let response: BlockRfqsResponse = result.unwrap();
    println!("Found {} Block RFQs", response.len());
}

/// Test private get_block_rfq_quotes endpoint
#[tokio::test]
#[ignore = "Requires authentication"]
#[serial_test::serial]
async fn test_get_block_rfq_quotes() {
    check_env_file().expect("Missing .env file");
    let client = DeribitHttpClient::new();

    let result: Result<Vec<BlockRfqQuote>, _> = client.get_block_rfq_quotes(None, None, None).await;

    assert!(
        result.is_ok(),
        "get_block_rfq_quotes failed: {:?}",
        result.err()
    );

    let quotes: Vec<BlockRfqQuote> = result.unwrap();
    println!("Found {} open Block RFQ quotes", quotes.len());
}

/// Test private cancel_all_block_rfq_quotes endpoint
#[tokio::test]
#[ignore = "Requires authentication - may cancel active quotes"]
#[serial_test::serial]
async fn test_cancel_all_block_rfq_quotes() {
    check_env_file().expect("Missing .env file");
    let client = DeribitHttpClient::new();

    let result: Result<Vec<BlockRfqQuote>, _> = client.cancel_all_block_rfq_quotes().await;

    assert!(
        result.is_ok(),
        "cancel_all_block_rfq_quotes failed: {:?}",
        result.err()
    );

    let cancelled: Vec<BlockRfqQuote> = result.unwrap();
    println!("Cancelled {} Block RFQ quotes", cancelled.len());
}

/// Test get_block_rfqs with state filter
#[tokio::test]
#[ignore = "Requires authentication"]
#[serial_test::serial]
async fn test_get_block_rfqs_with_state_filter() {
    check_env_file().expect("Missing .env file");
    let client = DeribitHttpClient::new();

    let result: Result<BlockRfqsResponse, _> = client
        .get_block_rfqs(Some(20), Some(BlockRfqState::Open), None, None, None, None)
        .await;

    assert!(result.is_ok(), "get_block_rfqs failed: {:?}", result.err());

    let response: BlockRfqsResponse = result.unwrap();
    println!("Found {} open Block RFQs", response.len());

    // Verify all returned RFQs are open
    for rfq in &response.block_rfqs {
        assert!(rfq.is_open(), "Expected open state, got {:?}", rfq.state);
    }
}

/// Test get_block_rfqs with role filter
#[tokio::test]
#[ignore = "Requires authentication"]
#[serial_test::serial]
async fn test_get_block_rfqs_with_role_filter() {
    check_env_file().expect("Missing .env file");
    let client = DeribitHttpClient::new();

    // Test as maker
    let maker_result: Result<BlockRfqsResponse, _> = client
        .get_block_rfqs(Some(10), None, Some(BlockRfqRole::Maker), None, None, None)
        .await;

    assert!(
        maker_result.is_ok(),
        "get_block_rfqs (maker) failed: {:?}",
        maker_result.err()
    );

    // Test as taker
    let taker_result: Result<BlockRfqsResponse, _> = client
        .get_block_rfqs(Some(10), None, Some(BlockRfqRole::Taker), None, None, None)
        .await;

    assert!(
        taker_result.is_ok(),
        "get_block_rfqs (taker) failed: {:?}",
        taker_result.err()
    );
}
