//! Unit tests for block trade models.

use deribit_http::model::block_trade::{
    BlockTrade, BlockTradeItem, BlockTradeRequest, BlockTradeResult, BlockTradeRole,
    BlockTradeSignature, BlockTradeTradeInfo, ExecuteBlockTradeRequest, GetBlockTradesRequest,
    SimulateBlockTradeRequest, TradeDirection, VerifyBlockTradeRequest,
};

#[test]
fn test_block_trade_role_serialization() {
    let maker = BlockTradeRole::Maker;
    let taker = BlockTradeRole::Taker;

    assert_eq!(serde_json::to_string(&maker).unwrap(), r#""maker""#);
    assert_eq!(serde_json::to_string(&taker).unwrap(), r#""taker""#);
}

#[test]
fn test_block_trade_role_deserialization() {
    let maker: BlockTradeRole = serde_json::from_str(r#""maker""#).unwrap();
    let taker: BlockTradeRole = serde_json::from_str(r#""taker""#).unwrap();

    assert_eq!(maker, BlockTradeRole::Maker);
    assert_eq!(taker, BlockTradeRole::Taker);
}

#[test]
fn test_block_trade_role_display() {
    assert_eq!(BlockTradeRole::Maker.to_string(), "maker");
    assert_eq!(BlockTradeRole::Taker.to_string(), "taker");
}

#[test]
fn test_trade_direction_serialization() {
    let buy = TradeDirection::Buy;
    let sell = TradeDirection::Sell;

    assert_eq!(serde_json::to_string(&buy).unwrap(), r#""buy""#);
    assert_eq!(serde_json::to_string(&sell).unwrap(), r#""sell""#);
}

#[test]
fn test_trade_direction_deserialization() {
    let buy: TradeDirection = serde_json::from_str(r#""buy""#).unwrap();
    let sell: TradeDirection = serde_json::from_str(r#""sell""#).unwrap();

    assert_eq!(buy, TradeDirection::Buy);
    assert_eq!(sell, TradeDirection::Sell);
}

#[test]
fn test_trade_direction_display() {
    assert_eq!(TradeDirection::Buy.to_string(), "buy");
    assert_eq!(TradeDirection::Sell.to_string(), "sell");
}

#[test]
fn test_block_trade_item_new() {
    let item = BlockTradeItem::new("BTC-PERPETUAL", 50000.0, Some(100.0), TradeDirection::Buy);

    assert_eq!(item.instrument_name, "BTC-PERPETUAL");
    assert!((item.price - 50000.0).abs() < f64::EPSILON);
    assert_eq!(item.amount, Some(100.0));
    assert_eq!(item.direction, TradeDirection::Buy);
}

#[test]
fn test_block_trade_item_serialization() {
    let item = BlockTradeItem::new("ETH-PERPETUAL", 3000.0, Some(50.0), TradeDirection::Sell);

    let json = serde_json::to_string(&item).unwrap();
    assert!(json.contains("ETH-PERPETUAL"));
    assert!(json.contains("3000"));
    assert!(json.contains("sell"));
    assert!(json.contains("50"));
}

#[test]
fn test_block_trade_item_without_amount() {
    let item = BlockTradeItem::new("BTC-28MAR25-100000-C", 0.05, None, TradeDirection::Buy);

    let json = serde_json::to_string(&item).unwrap();
    assert!(json.contains("BTC-28MAR25-100000-C"));
    assert!(!json.contains("amount")); // amount should be skipped when None
}

#[test]
fn test_execute_block_trade_request_serialization() {
    let request = ExecuteBlockTradeRequest {
        timestamp: 1565172650935,
        nonce: "test_nonce_123".to_string(),
        role: BlockTradeRole::Maker,
        trades: vec![
            BlockTradeItem::new("BTC-PERPETUAL", 50000.0, Some(100.0), TradeDirection::Buy),
            BlockTradeItem::new("ETH-PERPETUAL", 3000.0, Some(50.0), TradeDirection::Sell),
        ],
        counterparty_signature: "1565172710935.1ESE83qh.signature123".to_string(),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("1565172650935"));
    assert!(json.contains("test_nonce_123"));
    assert!(json.contains("maker"));
    assert!(json.contains("BTC-PERPETUAL"));
    assert!(json.contains("ETH-PERPETUAL"));
    assert!(json.contains("signature123"));
}

#[test]
fn test_verify_block_trade_request_serialization() {
    let request = VerifyBlockTradeRequest {
        timestamp: 1565172650935,
        nonce: "verify_nonce".to_string(),
        role: BlockTradeRole::Taker,
        trades: vec![BlockTradeItem::new(
            "BTC-PERPETUAL",
            50000.0,
            Some(100.0),
            TradeDirection::Buy,
        )],
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("1565172650935"));
    assert!(json.contains("verify_nonce"));
    assert!(json.contains("taker"));
    assert!(json.contains("BTC-PERPETUAL"));
}

#[test]
fn test_simulate_block_trade_request_with_role() {
    let request = SimulateBlockTradeRequest {
        role: Some(BlockTradeRole::Maker),
        trades: vec![BlockTradeItem::new(
            "BTC-PERPETUAL",
            50000.0,
            Some(40.0),
            TradeDirection::Buy,
        )],
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("maker"));
    assert!(json.contains("BTC-PERPETUAL"));
}

#[test]
fn test_simulate_block_trade_request_without_role() {
    let request = SimulateBlockTradeRequest {
        role: None,
        trades: vec![BlockTradeItem::new(
            "BTC-PERPETUAL",
            50000.0,
            Some(40.0),
            TradeDirection::Buy,
        )],
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("role")); // role should be skipped when None
    assert!(json.contains("BTC-PERPETUAL"));
}

#[test]
fn test_get_block_trades_request_default() {
    let request = GetBlockTradesRequest::default();

    assert!(request.currency.is_none());
    assert!(request.count.is_none());
    assert!(request.continuation.is_none());
    assert!(request.start_timestamp.is_none());
    assert!(request.end_timestamp.is_none());
}

#[test]
fn test_get_block_trades_request_with_filters() {
    let request = GetBlockTradesRequest {
        currency: Some("BTC".to_string()),
        count: Some(10),
        continuation: None,
        start_timestamp: Some(1565172650935),
        end_timestamp: Some(1565172750935),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("BTC"));
    assert!(json.contains("10"));
    assert!(json.contains("1565172650935"));
    assert!(json.contains("1565172750935"));
}

#[test]
fn test_block_trade_signature_deserialization() {
    let json = r#"{"signature":"1565172710935.1ESE83qh.g6fbgRd4VWagaJz7xdi2WaV"}"#;
    let sig: BlockTradeSignature = serde_json::from_str(json).unwrap();

    assert_eq!(
        sig.signature,
        "1565172710935.1ESE83qh.g6fbgRd4VWagaJz7xdi2WaV"
    );
}

#[test]
fn test_block_trade_trade_info_deserialization() {
    let json = r#"{
        "trade_id": "92437",
        "trade_seq": 37,
        "timestamp": 1565089523719,
        "tick_direction": 3,
        "state": "filled",
        "price": 0.0001,
        "order_type": "limit",
        "order_id": "343062",
        "matching_id": null,
        "liquidity": "T",
        "iv": 0,
        "instrument_name": "BTC-9AUG19-10250-C",
        "index_price": 11738,
        "fee_currency": "BTC",
        "fee": 0.00025,
        "direction": "sell",
        "block_trade_id": "61",
        "amount": 10
    }"#;

    let trade_info: BlockTradeTradeInfo = serde_json::from_str(json).unwrap();

    assert_eq!(trade_info.trade_id, "92437");
    assert_eq!(trade_info.trade_seq, Some(37));
    assert_eq!(trade_info.timestamp, 1565089523719);
    assert_eq!(trade_info.tick_direction, Some(3));
    assert_eq!(trade_info.state, Some("filled".to_string()));
    assert!((trade_info.price - 0.0001).abs() < f64::EPSILON);
    assert_eq!(trade_info.order_type, Some("limit".to_string()));
    assert_eq!(trade_info.instrument_name, "BTC-9AUG19-10250-C");
    assert_eq!(trade_info.direction, TradeDirection::Sell);
    assert_eq!(trade_info.block_trade_id, Some("61".to_string()));
    assert!((trade_info.amount - 10.0).abs() < f64::EPSILON);
}

#[test]
fn test_block_trade_deserialization() {
    let json = r#"{
        "id": "61",
        "timestamp": 1565089523720,
        "trades": [
            {
                "trade_id": "92437",
                "timestamp": 1565089523719,
                "price": 0.0001,
                "instrument_name": "BTC-9AUG19-10250-C",
                "direction": "sell",
                "amount": 10
            }
        ],
        "broker_code": "ABC123",
        "broker_name": "Test Broker"
    }"#;

    let trade: BlockTrade = serde_json::from_str(json).unwrap();

    assert_eq!(trade.id, "61");
    assert_eq!(trade.timestamp, 1565089523720);
    assert_eq!(trade.trades.len(), 1);
    assert_eq!(trade.broker_code, Some("ABC123".to_string()));
    assert_eq!(trade.broker_name, Some("Test Broker".to_string()));
    assert!(trade.app_name.is_none());
}

#[test]
fn test_block_trade_result_deserialization() {
    let json = r#"{
        "id": "6165",
        "timestamp": 1590485535980,
        "trades": [
            {
                "trade_id": "48079573",
                "timestamp": 1590485535978,
                "price": 8900.0,
                "instrument_name": "BTC-PERPETUAL",
                "direction": "sell",
                "amount": 200000.0
            }
        ]
    }"#;

    let result: BlockTradeResult = serde_json::from_str(json).unwrap();

    assert_eq!(result.id, "6165");
    assert_eq!(result.timestamp, 1590485535980);
    assert_eq!(result.trades.len(), 1);
    assert_eq!(result.trades[0].trade_id, "48079573");
}

#[test]
fn test_block_trade_request_deserialization() {
    let json = r#"{
        "timestamp": 1711468813551,
        "nonce": "bt-468nha",
        "role": "maker",
        "broker_code": "jpqYKgg1"
    }"#;

    let request: BlockTradeRequest = serde_json::from_str(json).unwrap();

    assert_eq!(request.timestamp, 1711468813551);
    assert_eq!(request.nonce, "bt-468nha");
    assert_eq!(request.role, BlockTradeRole::Maker);
    assert_eq!(request.broker_code, Some("jpqYKgg1".to_string()));
    assert!(request.trades.is_none());
}

#[test]
fn test_block_trade_with_multiple_trades() {
    let json = r#"{
        "id": "6165",
        "timestamp": 1590485535980,
        "trades": [
            {
                "trade_id": "48079573",
                "timestamp": 1590485535978,
                "price": 8900.0,
                "instrument_name": "BTC-PERPETUAL",
                "direction": "sell",
                "amount": 200000.0,
                "block_trade_id": "6165"
            },
            {
                "trade_id": "48079574",
                "timestamp": 1590485535979,
                "price": 0.0133,
                "instrument_name": "BTC-28MAY20-9000-C",
                "direction": "sell",
                "amount": 5.0,
                "block_trade_id": "6165",
                "iv": 62.44
            }
        ]
    }"#;

    let trade: BlockTrade = serde_json::from_str(json).unwrap();

    assert_eq!(trade.id, "6165");
    assert_eq!(trade.trades.len(), 2);
    assert_eq!(trade.trades[0].instrument_name, "BTC-PERPETUAL");
    assert_eq!(trade.trades[1].instrument_name, "BTC-28MAY20-9000-C");
    assert_eq!(trade.trades[1].iv, Some(62.44));
}

#[test]
fn test_block_trade_item_deserialization() {
    let json = r#"{
        "instrument_name": "BTC-PERPETUAL",
        "price": 50000.0,
        "amount": 100.0,
        "direction": "buy"
    }"#;

    let item: BlockTradeItem = serde_json::from_str(json).unwrap();

    assert_eq!(item.instrument_name, "BTC-PERPETUAL");
    assert!((item.price - 50000.0).abs() < f64::EPSILON);
    assert_eq!(item.amount, Some(100.0));
    assert_eq!(item.direction, TradeDirection::Buy);
}

#[test]
fn test_block_trade_role_equality() {
    assert_eq!(BlockTradeRole::Maker, BlockTradeRole::Maker);
    assert_eq!(BlockTradeRole::Taker, BlockTradeRole::Taker);
    assert_ne!(BlockTradeRole::Maker, BlockTradeRole::Taker);
}

#[test]
fn test_trade_direction_equality() {
    assert_eq!(TradeDirection::Buy, TradeDirection::Buy);
    assert_eq!(TradeDirection::Sell, TradeDirection::Sell);
    assert_ne!(TradeDirection::Buy, TradeDirection::Sell);
}

#[test]
fn test_block_trade_signature_equality() {
    let sig1 = BlockTradeSignature {
        signature: "test_signature".to_string(),
    };
    let sig2 = BlockTradeSignature {
        signature: "test_signature".to_string(),
    };
    let sig3 = BlockTradeSignature {
        signature: "different_signature".to_string(),
    };

    assert_eq!(sig1, sig2);
    assert_ne!(sig1, sig3);
}
