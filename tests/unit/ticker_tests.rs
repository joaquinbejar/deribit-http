use deribit_http::model::ticker::{Ticker, TickerData, TickerStats};
use deribit_http::model::instrument::InstrumentKind;
use deribit_http::model::other::Greeks;
use serde_json;

// Helper functions to create mock data
fn create_mock_ticker_stats() -> TickerStats {
    TickerStats {
        volume: 1000.0,
        volume_usd: Some(50000.0),
        price_change: Some(100.0),
        high: Some(51000.0),
        low: Some(49000.0),
    }
}

fn create_mock_greeks() -> Greeks {
    Greeks {
        delta: Some(0.5),
        gamma: Some(0.01),
        vega: Some(0.1),
        theta: Some(-0.05),
        rho: Some(0.02),
    }
}

fn create_mock_ticker_data() -> TickerData {
    TickerData {
        instrument_name: "BTC-PERPETUAL".to_string(),
        last_price: Some(50000.0),
        mark_price: 50100.0,
        best_bid_price: Some(49950.0),
        best_ask_price: Some(50050.0),
        best_bid_amount: 1.5,
        best_ask_amount: 2.0,
        volume: Some(1000.0),
        volume_usd: Some(50000000.0),
        open_interest: Some(5000.0),
        high: Some(51000.0),
        low: Some(49000.0),
        price_change: Some(500.0),
        price_change_percentage: Some(1.0),
        bid_iv: Some(0.8),
        ask_iv: Some(0.82),
        mark_iv: Some(0.81),
        timestamp: 1640995200000,
        state: "open".to_string(),
        settlement_price: None,
        stats: create_mock_ticker_stats(),
        greeks: Some(create_mock_greeks()),
        index_price: Some(49900.0),
        min_price: Some(45000.0),
        max_price: Some(55000.0),
        interest_rate: Some(0.05),
        underlying_price: Some(49950.0),
        underlying_index: Some("btc_usd".to_string()),
        estimated_delivery_price: Some(50000.0),
    }
}

fn create_mock_ticker() -> Ticker {
    Ticker {
        instrument_name: "BTC-PERPETUAL".to_string(),
        timestamp: 1640995200000,
        best_bid_price: Some(49950.0),
        best_bid_amount: Some(1.5),
        best_ask_price: Some(50050.0),
        best_ask_amount: Some(2.0),
        last_price: Some(50000.0),
        mark_price: Some(50100.0),
        index_price: Some(49900.0),
        open_interest: 5000.0,
        volume_24h: 1000.0,
        volume_usd_24h: 50000000.0,
        price_change_24h: 500.0,
        high_24h: Some(51000.0),
        low_24h: Some(49000.0),
        underlying_price: Some(49950.0),
        underlying_index: Some("btc_usd".to_string()),
        instrument_kind: Some(InstrumentKind::Future),
        current_funding: Some(0.0001),
        funding_8h: Some(0.0008),
        iv: Some(0.81),
        greeks: Some(create_mock_greeks()),
        interest_rate: Some(0.05),
    }
}

// Tests for TickerStats
#[test]
fn test_ticker_stats_creation() {
    let stats = create_mock_ticker_stats();
    assert_eq!(stats.volume, 1000.0);
    assert_eq!(stats.volume_usd, Some(50000.0));
    assert_eq!(stats.price_change, Some(100.0));
    assert_eq!(stats.high, Some(51000.0));
    assert_eq!(stats.low, Some(49000.0));
}

#[test]
fn test_ticker_stats_serialization() {
    let stats = create_mock_ticker_stats();
    let serialized = serde_json::to_string(&stats).unwrap();
    assert!(serialized.contains("volume"));
    assert!(serialized.contains("1000"));
    assert!(serialized.contains("volume_usd"));
    assert!(serialized.contains("50000"));
}

#[test]
fn test_ticker_stats_deserialization() {
    let json = r#"{
        "volume": 1000.0,
        "volume_usd": 50000.0,
        "price_change": 100.0,
        "high": 51000.0,
        "low": 49000.0
    }"#;
    
    let deserialized: TickerStats = serde_json::from_str(json).unwrap();
    assert_eq!(deserialized.volume, 1000.0);
    assert_eq!(deserialized.volume_usd, Some(50000.0));
    assert_eq!(deserialized.price_change, Some(100.0));
}

#[test]
fn test_ticker_stats_clone() {
    let stats = create_mock_ticker_stats();
    let cloned = stats.clone();
    assert_eq!(stats.volume, cloned.volume);
    assert_eq!(stats.volume_usd, cloned.volume_usd);
}

// Tests for TickerData
#[test]
fn test_ticker_data_creation() {
    let ticker_data = create_mock_ticker_data();
    assert_eq!(ticker_data.instrument_name, "BTC-PERPETUAL");
    assert_eq!(ticker_data.last_price, Some(50000.0));
    assert_eq!(ticker_data.mark_price, 50100.0);
    assert_eq!(ticker_data.best_bid_price, Some(49950.0));
    assert_eq!(ticker_data.best_ask_price, Some(50050.0));
}

#[test]
fn test_ticker_data_serialization() {
    let ticker_data = create_mock_ticker_data();
    let serialized = serde_json::to_string(&ticker_data).unwrap();
    assert!(serialized.contains("BTC-PERPETUAL"));
    assert!(serialized.contains("mark_price"));
    assert!(serialized.contains("50100"));
    assert!(serialized.contains("timestamp"));
}

#[test]
fn test_ticker_data_deserialization() {
    let json = r#"{
        "instrument_name": "BTC-PERPETUAL",
        "last_price": 50000.0,
        "mark_price": 50100.0,
        "best_bid_price": 49950.0,
        "best_ask_price": 50050.0,
        "best_bid_amount": 1.5,
        "best_ask_amount": 2.0,
        "timestamp": 1640995200000,
        "state": "open",
        "stats": {
            "volume": 1000.0,
            "volume_usd": 50000.0,
            "price_change": 100.0,
            "high": 51000.0,
            "low": 49000.0
        }
    }"#;
    
    let deserialized: TickerData = serde_json::from_str(json).unwrap();
    assert_eq!(deserialized.instrument_name, "BTC-PERPETUAL");
    assert_eq!(deserialized.mark_price, 50100.0);
    assert_eq!(deserialized.stats.volume, 1000.0);
}

#[test]
fn test_ticker_data_clone() {
    let ticker_data = create_mock_ticker_data();
    let cloned = ticker_data.clone();
    assert_eq!(ticker_data.instrument_name, cloned.instrument_name);
    assert_eq!(ticker_data.mark_price, cloned.mark_price);
}

// Tests for Ticker
#[test]
fn test_ticker_creation() {
    let ticker = create_mock_ticker();
    assert_eq!(ticker.instrument_name, "BTC-PERPETUAL");
    assert_eq!(ticker.timestamp, 1640995200000);
    assert_eq!(ticker.best_bid_price, Some(49950.0));
    assert_eq!(ticker.best_ask_price, Some(50050.0));
    assert_eq!(ticker.open_interest, 5000.0);
}

#[test]
fn test_ticker_serialization() {
    let ticker = create_mock_ticker();
    let serialized = serde_json::to_string(&ticker).unwrap();
    assert!(serialized.contains("BTC-PERPETUAL"));
    assert!(serialized.contains("timestamp"));
    assert!(serialized.contains("open_interest"));
    assert!(serialized.contains("5000"));
}

#[test]
fn test_ticker_deserialization() {
    let json = r#"{
        "instrument_name": "BTC-PERPETUAL",
        "timestamp": 1640995200000,
        "best_bid_price": 49950.0,
        "best_ask_price": 50050.0,
        "open_interest": 5000.0,
        "volume_24h": 1000.0,
        "volume_usd_24h": 50000000.0,
        "price_change_24h": 500.0
    }"#;
    
    let deserialized: Ticker = serde_json::from_str(json).unwrap();
    assert_eq!(deserialized.instrument_name, "BTC-PERPETUAL");
    assert_eq!(deserialized.timestamp, 1640995200000);
    assert_eq!(deserialized.open_interest, 5000.0);
}

#[test]
fn test_ticker_clone() {
    let ticker = create_mock_ticker();
    let cloned = ticker.clone();
    assert_eq!(ticker.instrument_name, cloned.instrument_name);
    assert_eq!(ticker.timestamp, cloned.timestamp);
    assert_eq!(ticker.open_interest, cloned.open_interest);
}

// Tests for Ticker methods
#[test]
fn test_ticker_spread() {
    let ticker = create_mock_ticker();
    let spread = ticker.spread().unwrap();
    assert_eq!(spread, 100.0); // 50050.0 - 49950.0
}

#[test]
fn test_ticker_spread_none() {
    let mut ticker = create_mock_ticker();
    ticker.best_ask_price = None;
    assert!(ticker.spread().is_none());
    
    ticker.best_ask_price = Some(50050.0);
    ticker.best_bid_price = None;
    assert!(ticker.spread().is_none());
}

#[test]
fn test_ticker_mid_price() {
    let ticker = create_mock_ticker();
    let mid_price = ticker.mid_price().unwrap();
    assert_eq!(mid_price, 50000.0); // (50050.0 + 49950.0) / 2.0
}

#[test]
fn test_ticker_mid_price_none() {
    let mut ticker = create_mock_ticker();
    ticker.best_ask_price = None;
    assert!(ticker.mid_price().is_none());
    
    ticker.best_ask_price = Some(50050.0);
    ticker.best_bid_price = None;
    assert!(ticker.mid_price().is_none());
}

#[test]
fn test_ticker_spread_percentage() {
    let ticker = create_mock_ticker();
    let spread_percentage = ticker.spread_percentage().unwrap();
    assert_eq!(spread_percentage, 0.2); // (100.0 / 50000.0) * 100.0
}

#[test]
fn test_ticker_spread_percentage_none() {
    let mut ticker = create_mock_ticker();
    ticker.best_ask_price = None;
    assert!(ticker.spread_percentage().is_none());
}

#[test]
fn test_ticker_spread_percentage_zero_mid() {
    let mut ticker = create_mock_ticker();
    ticker.best_bid_price = Some(0.0);
    ticker.best_ask_price = Some(0.0);
    assert!(ticker.spread_percentage().is_none());
}

#[test]
fn test_ticker_has_valid_spread() {
    let ticker = create_mock_ticker();
    assert!(ticker.has_valid_spread());
    
    let mut ticker_no_bid = create_mock_ticker();
    ticker_no_bid.best_bid_price = None;
    assert!(!ticker_no_bid.has_valid_spread());
    
    let mut ticker_no_ask = create_mock_ticker();
    ticker_no_ask.best_ask_price = None;
    assert!(!ticker_no_ask.has_valid_spread());
}

// Edge cases and error handling
#[test]
fn test_ticker_with_minimal_data() {
    let ticker = Ticker {
        instrument_name: "TEST".to_string(),
        timestamp: 0,
        best_bid_price: None,
        best_bid_amount: None,
        best_ask_price: None,
        best_ask_amount: None,
        last_price: None,
        mark_price: None,
        index_price: None,
        open_interest: 0.0,
        volume_24h: 0.0,
        volume_usd_24h: 0.0,
        price_change_24h: 0.0,
        high_24h: None,
        low_24h: None,
        underlying_price: None,
        underlying_index: None,
        instrument_kind: None,
        current_funding: None,
        funding_8h: None,
        iv: None,
        greeks: None,
        interest_rate: None,
    };
    
    assert_eq!(ticker.instrument_name, "TEST");
    assert!(ticker.spread().is_none());
    assert!(ticker.mid_price().is_none());
    assert!(!ticker.has_valid_spread());
}

#[test]
fn test_ticker_stats_with_none_values() {
    let stats = TickerStats {
        volume: 0.0,
        volume_usd: None,
        price_change: None,
        high: None,
        low: None,
    };
    
    let serialized = serde_json::to_string(&stats).unwrap();
    assert!(serialized.contains("volume"));
    assert!(!serialized.contains("volume_usd"));
}

#[test]
fn test_ticker_data_with_expired_state() {
    let mut ticker_data = create_mock_ticker_data();
    ticker_data.state = "expired".to_string();
    ticker_data.settlement_price = Some(50500.0);
    
    assert_eq!(ticker_data.state, "expired");
    assert_eq!(ticker_data.settlement_price, Some(50500.0));
}

#[test]
fn test_ticker_negative_spread() {
    let mut ticker = create_mock_ticker();
    ticker.best_bid_price = Some(50100.0);
    ticker.best_ask_price = Some(50000.0);
    
    let spread = ticker.spread().unwrap();
    assert_eq!(spread, -100.0); // Negative spread (crossed market)
}