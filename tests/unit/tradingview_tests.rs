use deribit_http::model::tradingview::TradingViewChartData;
use serde_json;

// Helper function to create mock data
fn create_mock_chart_data() -> TradingViewChartData {
    let mut chart_data = TradingViewChartData::new();
    chart_data.add_candle(
        1640995200000, // timestamp
        50000.0,       // open
        51000.0,       // high
        49500.0,       // low
        50500.0,       // close
        1000.0,        // volume
        50250000.0,    // cost
    );
    chart_data.add_candle(
        1640995260000, // timestamp + 60s
        50500.0,       // open
        50800.0,       // high
        50200.0,       // low
        50300.0,       // close
        800.0,         // volume
        40240000.0,    // cost
    );
    chart_data
}

fn create_empty_chart_data() -> TradingViewChartData {
    TradingViewChartData {
        status: "ok".to_string(),
        ticks: Vec::new(),
        open: Vec::new(),
        high: Vec::new(),
        low: Vec::new(),
        close: Vec::new(),
        volume: Vec::new(),
        cost: Vec::new(),
    }
}

// Tests for TradingViewChartData creation
#[test]
fn test_trading_view_chart_data_new() {
    let chart_data = TradingViewChartData::new();
    assert_eq!(chart_data.status, "ok");
    assert!(chart_data.ticks.is_empty());
    assert!(chart_data.open.is_empty());
    assert!(chart_data.high.is_empty());
    assert!(chart_data.low.is_empty());
    assert!(chart_data.close.is_empty());
    assert!(chart_data.volume.is_empty());
    assert!(chart_data.cost.is_empty());
}

#[test]
fn test_trading_view_chart_data_default() {
    let chart_data = TradingViewChartData::default();
    assert_eq!(chart_data.status, "ok");
    assert!(chart_data.ticks.is_empty());
    assert!(chart_data.open.is_empty());
    assert!(chart_data.high.is_empty());
    assert!(chart_data.low.is_empty());
    assert!(chart_data.close.is_empty());
    assert!(chart_data.volume.is_empty());
    assert!(chart_data.cost.is_empty());
}

#[test]
fn test_trading_view_chart_data_creation() {
    let chart_data = create_empty_chart_data();
    assert_eq!(chart_data.status, "ok");
    assert_eq!(chart_data.ticks.len(), 0);
    assert_eq!(chart_data.open.len(), 0);
    assert_eq!(chart_data.high.len(), 0);
    assert_eq!(chart_data.low.len(), 0);
    assert_eq!(chart_data.close.len(), 0);
    assert_eq!(chart_data.volume.len(), 0);
    assert_eq!(chart_data.cost.len(), 0);
}

// Tests for add_candle method
#[test]
fn test_add_single_candle() {
    let mut chart_data = TradingViewChartData::new();
    chart_data.add_candle(
        1640995200000, // timestamp
        50000.0,       // open
        51000.0,       // high
        49500.0,       // low
        50500.0,       // close
        1000.0,        // volume
        50250000.0,    // cost
    );

    assert_eq!(chart_data.ticks.len(), 1);
    assert_eq!(chart_data.open.len(), 1);
    assert_eq!(chart_data.high.len(), 1);
    assert_eq!(chart_data.low.len(), 1);
    assert_eq!(chart_data.close.len(), 1);
    assert_eq!(chart_data.volume.len(), 1);
    assert_eq!(chart_data.cost.len(), 1);

    assert_eq!(chart_data.ticks[0], 1640995200000);
    assert_eq!(chart_data.open[0], 50000.0);
    assert_eq!(chart_data.high[0], 51000.0);
    assert_eq!(chart_data.low[0], 49500.0);
    assert_eq!(chart_data.close[0], 50500.0);
    assert_eq!(chart_data.volume[0], 1000.0);
    assert_eq!(chart_data.cost[0], 50250000.0);
}

#[test]
fn test_add_multiple_candles() {
    let chart_data = create_mock_chart_data();

    assert_eq!(chart_data.ticks.len(), 2);
    assert_eq!(chart_data.open.len(), 2);
    assert_eq!(chart_data.high.len(), 2);
    assert_eq!(chart_data.low.len(), 2);
    assert_eq!(chart_data.close.len(), 2);
    assert_eq!(chart_data.volume.len(), 2);
    assert_eq!(chart_data.cost.len(), 2);

    // First candle
    assert_eq!(chart_data.ticks[0], 1640995200000);
    assert_eq!(chart_data.open[0], 50000.0);
    assert_eq!(chart_data.high[0], 51000.0);
    assert_eq!(chart_data.low[0], 49500.0);
    assert_eq!(chart_data.close[0], 50500.0);
    assert_eq!(chart_data.volume[0], 1000.0);
    assert_eq!(chart_data.cost[0], 50250000.0);

    // Second candle
    assert_eq!(chart_data.ticks[1], 1640995260000);
    assert_eq!(chart_data.open[1], 50500.0);
    assert_eq!(chart_data.high[1], 50800.0);
    assert_eq!(chart_data.low[1], 50200.0);
    assert_eq!(chart_data.close[1], 50300.0);
    assert_eq!(chart_data.volume[1], 800.0);
    assert_eq!(chart_data.cost[1], 40240000.0);
}

#[test]
fn test_add_candle_with_zero_values() {
    let mut chart_data = TradingViewChartData::new();
    chart_data.add_candle(0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    assert_eq!(chart_data.ticks.len(), 1);
    assert_eq!(chart_data.ticks[0], 0);
    assert_eq!(chart_data.open[0], 0.0);
    assert_eq!(chart_data.high[0], 0.0);
    assert_eq!(chart_data.low[0], 0.0);
    assert_eq!(chart_data.close[0], 0.0);
    assert_eq!(chart_data.volume[0], 0.0);
    assert_eq!(chart_data.cost[0], 0.0);
}

#[test]
fn test_add_candle_with_negative_values() {
    let mut chart_data = TradingViewChartData::new();
    chart_data.add_candle(
        1640995200000,
        -100.0,  // negative open
        100.0,   // positive high
        -200.0,  // negative low
        50.0,    // positive close
        -10.0,   // negative volume
        -1000.0, // negative cost
    );

    assert_eq!(chart_data.open[0], -100.0);
    assert_eq!(chart_data.high[0], 100.0);
    assert_eq!(chart_data.low[0], -200.0);
    assert_eq!(chart_data.close[0], 50.0);
    assert_eq!(chart_data.volume[0], -10.0);
    assert_eq!(chart_data.cost[0], -1000.0);
}

// Tests for serialization
#[test]
fn test_trading_view_chart_data_serialization() {
    let chart_data = create_mock_chart_data();
    let serialized = serde_json::to_string(&chart_data).unwrap();

    assert!(serialized.contains("status"));
    assert!(serialized.contains("ok"));
    assert!(serialized.contains("ticks"));
    assert!(serialized.contains("open"));
    assert!(serialized.contains("high"));
    assert!(serialized.contains("low"));
    assert!(serialized.contains("close"));
    assert!(serialized.contains("volume"));
    assert!(serialized.contains("cost"));
    assert!(serialized.contains("1640995200000"));
    assert!(serialized.contains("50000"));
    assert!(serialized.contains("51000"));
}

#[test]
fn test_trading_view_chart_data_serialization_empty() {
    let chart_data = TradingViewChartData::new();
    let serialized = serde_json::to_string(&chart_data).unwrap();

    assert!(serialized.contains("status"));
    assert!(serialized.contains("ok"));
    assert!(serialized.contains("ticks"));
    assert!(serialized.contains("[]"));
}

// Tests for deserialization
#[test]
fn test_trading_view_chart_data_deserialization() {
    let json = r#"{
        "status": "ok",
        "ticks": [1640995200000, 1640995260000],
        "open": [50000.0, 50500.0],
        "high": [51000.0, 50800.0],
        "low": [49500.0, 50200.0],
        "close": [50500.0, 50300.0],
        "volume": [1000.0, 800.0],
        "cost": [50250000.0, 40240000.0]
    }"#;

    let deserialized: TradingViewChartData = serde_json::from_str(json).unwrap();

    assert_eq!(deserialized.status, "ok");
    assert_eq!(deserialized.ticks.len(), 2);
    assert_eq!(deserialized.open.len(), 2);
    assert_eq!(deserialized.high.len(), 2);
    assert_eq!(deserialized.low.len(), 2);
    assert_eq!(deserialized.close.len(), 2);
    assert_eq!(deserialized.volume.len(), 2);
    assert_eq!(deserialized.cost.len(), 2);

    assert_eq!(deserialized.ticks[0], 1640995200000);
    assert_eq!(deserialized.open[0], 50000.0);
    assert_eq!(deserialized.high[0], 51000.0);
    assert_eq!(deserialized.low[0], 49500.0);
    assert_eq!(deserialized.close[0], 50500.0);
    assert_eq!(deserialized.volume[0], 1000.0);
    assert_eq!(deserialized.cost[0], 50250000.0);
}

#[test]
fn test_trading_view_chart_data_deserialization_empty() {
    let json = r#"{
        "status": "ok",
        "ticks": [],
        "open": [],
        "high": [],
        "low": [],
        "close": [],
        "volume": [],
        "cost": []
    }"#;

    let deserialized: TradingViewChartData = serde_json::from_str(json).unwrap();

    assert_eq!(deserialized.status, "ok");
    assert!(deserialized.ticks.is_empty());
    assert!(deserialized.open.is_empty());
    assert!(deserialized.high.is_empty());
    assert!(deserialized.low.is_empty());
    assert!(deserialized.close.is_empty());
    assert!(deserialized.volume.is_empty());
    assert!(deserialized.cost.is_empty());
}

#[test]
fn test_trading_view_chart_data_deserialization_error_status() {
    let json = r#"{
        "status": "error",
        "ticks": [],
        "open": [],
        "high": [],
        "low": [],
        "close": [],
        "volume": [],
        "cost": []
    }"#;

    let deserialized: TradingViewChartData = serde_json::from_str(json).unwrap();
    assert_eq!(deserialized.status, "error");
}

// Tests for cloning
#[test]
fn test_trading_view_chart_data_clone() {
    let chart_data = create_mock_chart_data();
    let cloned = chart_data.clone();

    assert_eq!(chart_data.status, cloned.status);
    assert_eq!(chart_data.ticks, cloned.ticks);
    assert_eq!(chart_data.open, cloned.open);
    assert_eq!(chart_data.high, cloned.high);
    assert_eq!(chart_data.low, cloned.low);
    assert_eq!(chart_data.close, cloned.close);
    assert_eq!(chart_data.volume, cloned.volume);
    assert_eq!(chart_data.cost, cloned.cost);
}

#[test]
fn test_trading_view_chart_data_clone_empty() {
    let chart_data = TradingViewChartData::new();
    let cloned = chart_data.clone();

    assert_eq!(chart_data.status, cloned.status);
    assert_eq!(chart_data.ticks.len(), cloned.ticks.len());
    assert_eq!(chart_data.open.len(), cloned.open.len());
    assert_eq!(chart_data.high.len(), cloned.high.len());
    assert_eq!(chart_data.low.len(), cloned.low.len());
    assert_eq!(chart_data.close.len(), cloned.close.len());
    assert_eq!(chart_data.volume.len(), cloned.volume.len());
    assert_eq!(chart_data.cost.len(), cloned.cost.len());
}

// Edge cases and boundary tests
#[test]
fn test_trading_view_chart_data_large_numbers() {
    let mut chart_data = TradingViewChartData::new();
    chart_data.add_candle(
        u64::MAX,
        f64::MAX,
        f64::MAX,
        f64::MIN,
        f64::MAX / 2.0,
        f64::MAX,
        f64::MAX,
    );

    assert_eq!(chart_data.ticks[0], u64::MAX);
    assert_eq!(chart_data.open[0], f64::MAX);
    assert_eq!(chart_data.high[0], f64::MAX);
    assert_eq!(chart_data.low[0], f64::MIN);
    assert_eq!(chart_data.close[0], f64::MAX / 2.0);
    assert_eq!(chart_data.volume[0], f64::MAX);
    assert_eq!(chart_data.cost[0], f64::MAX);
}

#[test]
fn test_trading_view_chart_data_consistency() {
    let mut chart_data = TradingViewChartData::new();

    // Add multiple candles
    for i in 0..10 {
        chart_data.add_candle(
            1640995200000 + (i * 60000), // timestamp increments
            50000.0 + i as f64,          // open increments
            51000.0 + i as f64,          // high increments
            49000.0 + i as f64,          // low increments
            50500.0 + i as f64,          // close increments
            1000.0 + i as f64,           // volume increments
            50000000.0 + i as f64,       // cost increments
        );
    }

    // Verify all arrays have the same length
    assert_eq!(chart_data.ticks.len(), 10);
    assert_eq!(chart_data.open.len(), 10);
    assert_eq!(chart_data.high.len(), 10);
    assert_eq!(chart_data.low.len(), 10);
    assert_eq!(chart_data.close.len(), 10);
    assert_eq!(chart_data.volume.len(), 10);
    assert_eq!(chart_data.cost.len(), 10);

    // Verify data consistency
    for i in 0..10 {
        assert_eq!(chart_data.ticks[i], 1640995200000 + (i as u64 * 60000));
        assert_eq!(chart_data.open[i], 50000.0 + i as f64);
        assert_eq!(chart_data.high[i], 51000.0 + i as f64);
        assert_eq!(chart_data.low[i], 49000.0 + i as f64);
        assert_eq!(chart_data.close[i], 50500.0 + i as f64);
        assert_eq!(chart_data.volume[i], 1000.0 + i as f64);
        assert_eq!(chart_data.cost[i], 50000000.0 + i as f64);
    }
}

#[test]
fn test_trading_view_chart_data_with_different_status() {
    let mut chart_data = TradingViewChartData::new();
    chart_data.status = "error".to_string();

    assert_eq!(chart_data.status, "error");

    let serialized = serde_json::to_string(&chart_data).unwrap();
    assert!(serialized.contains("error"));
    assert!(!serialized.contains("ok"));
}

#[test]
fn test_trading_view_chart_data_round_trip_serialization() {
    let original = create_mock_chart_data();
    let serialized = serde_json::to_string(&original).unwrap();
    let deserialized: TradingViewChartData = serde_json::from_str(&serialized).unwrap();

    assert_eq!(original.status, deserialized.status);
    assert_eq!(original.ticks, deserialized.ticks);
    assert_eq!(original.open, deserialized.open);
    assert_eq!(original.high, deserialized.high);
    assert_eq!(original.low, deserialized.low);
    assert_eq!(original.close, deserialized.close);
    assert_eq!(original.volume, deserialized.volume);
    assert_eq!(original.cost, deserialized.cost);
}
