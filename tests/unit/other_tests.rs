use deribit_http::model::OptionType;
use deribit_http::model::instrument::{Instrument, InstrumentKind, InstrumentType};
use deribit_http::model::other::*;
use deribit_http::model::ticker::{TickerData, TickerStats};
use serde::{Deserialize, Serialize};
use serde_json;

// Helper function to create mock ticker data
fn create_mock_ticker_data() -> TickerData {
    TickerData {
        instrument_name: "BTC-PERPETUAL".to_string(),
        last_price: Some(50000.0),
        mark_price: 50500.0,
        best_bid_price: Some(49950.0),
        best_ask_price: Some(50050.0),
        best_bid_amount: 10.0,
        best_ask_amount: 15.0,
        volume: Some(1000.0),
        volume_usd: Some(50000000.0),
        open_interest: Some(5000.0),
        high: Some(51000.0),
        low: Some(49000.0),
        price_change: Some(500.0),
        price_change_percentage: Some(1.0),
        bid_iv: Some(0.75),
        ask_iv: Some(0.8),
        mark_iv: Some(0.77),
        timestamp: 1640995200000,
        state: "open".to_string(),
        settlement_price: None,
        stats: TickerStats {
            volume: 1000.0,
            volume_usd: Some(50000000.0),
            price_change: Some(500.0),
            high: Some(51000.0),
            low: Some(49000.0),
        },
        greeks: None,
        index_price: Some(50000.0),
        min_price: Some(45000.0),
        max_price: Some(55000.0),
        interest_rate: Some(0.05),
        underlying_price: Some(50000.0),
        underlying_index: Some("btc_usd".to_string()),
        estimated_delivery_price: Some(50100.0),
    }
}

// Helper function to create mock instrument
fn create_mock_instrument() -> Instrument {
    Instrument {
        instrument_name: "BTC-PERPETUAL".to_string(),
        price_index: Some("btc_usd".to_string()),
        kind: Some(InstrumentKind::Future),
        currency: Some("BTC".to_string()),
        is_active: Some(true),
        expiration_timestamp: None,
        strike: None,
        option_type: None,
        tick_size: Some(0.5),
        min_trade_amount: Some(1.0),
        contract_size: Some(1.0),
        settlement_period: Some("perpetual".to_string()),
        instrument_type: Some(InstrumentType::Linear),
        quote_currency: Some("USD".to_string()),
        settlement_currency: Some("BTC".to_string()),
        creation_timestamp: Some(1640995200000),
        max_leverage: Some(100.0),
        maker_commission: Some(0.0001),
        taker_commission: Some(0.0005),
        instrument_id: Some(12345),
        base_currency: Some("BTC".to_string()),
        counter_currency: Some("USD".to_string()),
    }
}

#[cfg(test)]
mod delivery_price_data_tests {
    use super::*;

    #[test]
    fn test_delivery_price_data_creation() {
        let delivery_data = DeliveryPriceData {
            date: "2021-12-25".to_string(),
            delivery_price: 50000.0,
        };

        assert_eq!(delivery_data.date, "2021-12-25");
        assert_eq!(delivery_data.delivery_price, 50000.0);
    }

    #[test]
    fn test_delivery_price_data_serialization() {
        let delivery_data = DeliveryPriceData {
            date: "2021-12-25".to_string(),
            delivery_price: 50000.0,
        };

        let serialized = serde_json::to_string(&delivery_data).unwrap();
        assert!(serialized.contains("\"date\":\"2021-12-25\""));
        assert!(serialized.contains("\"delivery_price\":50000.0"));
    }

    #[test]
    fn test_delivery_price_data_deserialization() {
        let json = r#"{"date":"2021-12-25","delivery_price":50000.0}"#;
        let delivery_data: DeliveryPriceData = serde_json::from_str(json).unwrap();

        assert_eq!(delivery_data.date, "2021-12-25");
        assert_eq!(delivery_data.delivery_price, 50000.0);
    }

    #[test]
    fn test_delivery_price_data_clone() {
        let delivery_data = DeliveryPriceData {
            date: "2021-12-25".to_string(),
            delivery_price: 50000.0,
        };

        let cloned = delivery_data.clone();
        assert_eq!(delivery_data.date, cloned.date);
        assert_eq!(delivery_data.delivery_price, cloned.delivery_price);
    }

    #[test]
    fn test_delivery_price_data_debug_display() {
        let delivery_data = DeliveryPriceData {
            date: "2021-12-25".to_string(),
            delivery_price: 50000.0,
        };

        let debug_str = format!("{:?}", delivery_data);
        let display_str = format!("{}", delivery_data);

        assert!(!debug_str.is_empty());
        assert!(!display_str.is_empty());
    }
}

#[cfg(test)]
mod greeks_tests {
    use super::*;

    #[test]
    fn test_greeks_creation_with_all_values() {
        let greeks = Greeks {
            delta: Some(0.5),
            gamma: Some(0.02),
            vega: Some(0.1),
            theta: Some(-0.05),
            rho: Some(0.03),
        };

        assert_eq!(greeks.delta, Some(0.5));
        assert_eq!(greeks.gamma, Some(0.02));
        assert_eq!(greeks.vega, Some(0.1));
        assert_eq!(greeks.theta, Some(-0.05));
        assert_eq!(greeks.rho, Some(0.03));
    }

    #[test]
    fn test_greeks_creation_with_none_values() {
        let greeks = Greeks {
            delta: None,
            gamma: None,
            vega: None,
            theta: None,
            rho: None,
        };

        assert_eq!(greeks.delta, None);
        assert_eq!(greeks.gamma, None);
        assert_eq!(greeks.vega, None);
        assert_eq!(greeks.theta, None);
        assert_eq!(greeks.rho, None);
    }

    #[test]
    fn test_greeks_serialization_with_values() {
        let greeks = Greeks {
            delta: Some(0.5),
            gamma: Some(0.02),
            vega: None,
            theta: Some(-0.05),
            rho: None,
        };

        let serialized = serde_json::to_string(&greeks).unwrap();
        assert!(serialized.contains("\"delta\":0.5"));
        assert!(serialized.contains("\"gamma\":0.02"));
        assert!(serialized.contains("\"theta\":-0.05"));
        // None values should be skipped in serialization
        assert!(!serialized.contains("\"vega\""));
        assert!(!serialized.contains("\"rho\""));
    }

    #[test]
    fn test_greeks_deserialization() {
        let json = r#"{"delta":0.5,"gamma":0.02,"theta":-0.05}"#;
        let greeks: Greeks = serde_json::from_str(json).unwrap();

        assert_eq!(greeks.delta, Some(0.5));
        assert_eq!(greeks.gamma, Some(0.02));
        assert_eq!(greeks.vega, None);
        assert_eq!(greeks.theta, Some(-0.05));
        assert_eq!(greeks.rho, None);
    }

    #[test]
    fn test_greeks_clone() {
        let greeks = Greeks {
            delta: Some(0.5),
            gamma: Some(0.02),
            vega: Some(0.1),
            theta: Some(-0.05),
            rho: Some(0.03),
        };

        let cloned = greeks.clone();
        assert_eq!(greeks.delta, cloned.delta);
        assert_eq!(greeks.gamma, cloned.gamma);
        assert_eq!(greeks.vega, cloned.vega);
        assert_eq!(greeks.theta, cloned.theta);
        assert_eq!(greeks.rho, cloned.rho);
    }
}

#[cfg(test)]
mod option_instrument_tests {
    use super::*;

    #[test]
    fn test_option_instrument_creation() {
        let instrument = create_mock_instrument();
        let ticker = create_mock_ticker_data();

        let option_instrument = OptionInstrument {
            instrument: instrument.clone(),
            ticker: ticker.clone(),
        };

        assert_eq!(
            option_instrument.instrument.instrument_name,
            instrument.instrument_name
        );
        assert_eq!(
            option_instrument.ticker.instrument_name,
            ticker.instrument_name
        );
    }

    #[test]
    fn test_option_instrument_serialization() {
        let instrument = create_mock_instrument();
        let ticker = create_mock_ticker_data();

        let option_instrument = OptionInstrument { instrument, ticker };

        let serialized = serde_json::to_string(&option_instrument).unwrap();
        assert!(serialized.contains("\"instrument\""));
        assert!(serialized.contains("\"ticker\""));
    }

    #[test]
    fn test_option_instrument_deserialization() {
        let instrument = create_mock_instrument();
        let ticker = create_mock_ticker_data();

        let option_instrument = OptionInstrument { instrument, ticker };

        let serialized = serde_json::to_string(&option_instrument).unwrap();
        let deserialized: OptionInstrument = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            option_instrument.instrument.instrument_name,
            deserialized.instrument.instrument_name
        );
        assert_eq!(
            option_instrument.ticker.instrument_name,
            deserialized.ticker.instrument_name
        );
    }

    #[test]
    fn test_option_instrument_clone() {
        let instrument = create_mock_instrument();
        let ticker = create_mock_ticker_data();

        let option_instrument = OptionInstrument { instrument, ticker };

        let cloned = option_instrument.clone();
        assert_eq!(
            option_instrument.instrument.instrument_name,
            cloned.instrument.instrument_name
        );
        assert_eq!(
            option_instrument.ticker.instrument_name,
            cloned.ticker.instrument_name
        );
    }
}

#[cfg(test)]
mod option_instrument_pair_tests {
    use super::*;

    #[test]
    fn test_option_instrument_pair_with_both() {
        let instrument = create_mock_instrument();
        let ticker = create_mock_ticker_data();

        let call_option = OptionInstrument {
            instrument: instrument.clone(),
            ticker: ticker.clone(),
        };

        let put_option = OptionInstrument { instrument, ticker };

        let pair = OptionInstrumentPair {
            call: Some(call_option),
            put: Some(put_option),
        };

        assert!(pair.call.is_some());
        assert!(pair.put.is_some());
    }

    #[test]
    fn test_option_instrument_pair_with_call_only() {
        let instrument = create_mock_instrument();
        let ticker = create_mock_ticker_data();

        let call_option = OptionInstrument { instrument, ticker };

        let pair = OptionInstrumentPair {
            call: Some(call_option),
            put: None,
        };

        assert!(pair.call.is_some());
        assert!(pair.put.is_none());
    }

    #[test]
    fn test_option_instrument_pair_with_put_only() {
        let instrument = create_mock_instrument();
        let ticker = create_mock_ticker_data();

        let put_option = OptionInstrument { instrument, ticker };

        let pair = OptionInstrumentPair {
            call: None,
            put: Some(put_option),
        };

        assert!(pair.call.is_none());
        assert!(pair.put.is_some());
    }

    #[test]
    fn test_option_instrument_pair_empty() {
        let pair = OptionInstrumentPair {
            call: None,
            put: None,
        };

        assert!(pair.call.is_none());
        assert!(pair.put.is_none());
    }

    #[test]
    fn test_option_instrument_pair_serialization() {
        let instrument = create_mock_instrument();
        let ticker = create_mock_ticker_data();

        let call_option = OptionInstrument { instrument, ticker };

        let pair = OptionInstrumentPair {
            call: Some(call_option),
            put: None,
        };

        let serialized = serde_json::to_string(&pair).unwrap();
        assert!(serialized.contains("\"call\""));
        // None values should be skipped
        assert!(!serialized.contains("\"put\""));
    }

    #[test]
    fn test_option_instrument_pair_clone() {
        let instrument = create_mock_instrument();
        let ticker = create_mock_ticker_data();

        let call_option = OptionInstrument { instrument, ticker };

        let pair = OptionInstrumentPair {
            call: Some(call_option),
            put: None,
        };

        let cloned = pair.clone();
        assert_eq!(pair.call.is_some(), cloned.call.is_some());
        assert_eq!(pair.put.is_none(), cloned.put.is_none());
    }
}

#[cfg(test)]
mod parsed_option_with_ticker_tests {
    use super::*;

    #[test]
    fn test_parsed_option_with_ticker_creation() {
        let ticker = create_mock_ticker_data();

        let parsed_option = ParsedOptionWithTicker {
            instrument_name: "BTC-25DEC21-50000-C".to_string(),
            strike: 50000.0,
            option_type: OptionType::Call,
            expiry: "2021-12-25".to_string(),
            ticker,
        };

        assert_eq!(parsed_option.instrument_name, "BTC-25DEC21-50000-C");
        assert_eq!(parsed_option.strike, 50000.0);
        assert_eq!(parsed_option.option_type, OptionType::Call);
        assert_eq!(parsed_option.expiry, "2021-12-25");
    }

    #[test]
    fn test_parsed_option_with_ticker_put() {
        let ticker = create_mock_ticker_data();

        let parsed_option = ParsedOptionWithTicker {
            instrument_name: "BTC-25DEC21-45000-P".to_string(),
            strike: 45000.0,
            option_type: OptionType::Put,
            expiry: "2021-12-25".to_string(),
            ticker,
        };

        assert_eq!(parsed_option.option_type, OptionType::Put);
        assert_eq!(parsed_option.strike, 45000.0);
    }

    #[test]
    fn test_parsed_option_with_ticker_serialization() {
        let ticker = create_mock_ticker_data();

        let parsed_option = ParsedOptionWithTicker {
            instrument_name: "BTC-25DEC21-50000-C".to_string(),
            strike: 50000.0,
            option_type: OptionType::Call,
            expiry: "2021-12-25".to_string(),
            ticker,
        };

        let serialized = serde_json::to_string(&parsed_option).unwrap();
        assert!(serialized.contains("\"instrument_name\":\"BTC-25DEC21-50000-C\""));
        assert!(serialized.contains("\"strike\":50000.0"));
        assert!(serialized.contains("\"expiry\":\"2021-12-25\""));
    }

    #[test]
    fn test_parsed_option_with_ticker_clone() {
        let ticker = create_mock_ticker_data();

        let parsed_option = ParsedOptionWithTicker {
            instrument_name: "BTC-25DEC21-50000-C".to_string(),
            strike: 50000.0,
            option_type: OptionType::Call,
            expiry: "2021-12-25".to_string(),
            ticker,
        };

        let cloned = parsed_option.clone();
        assert_eq!(parsed_option.instrument_name, cloned.instrument_name);
        assert_eq!(parsed_option.strike, cloned.strike);
        assert_eq!(parsed_option.option_type, cloned.option_type);
        assert_eq!(parsed_option.expiry, cloned.expiry);
    }
}

#[cfg(test)]
mod sort_direction_tests {
    use super::*;

    #[test]
    fn test_sort_direction_default() {
        let _default_sort = SortDirection::default();
        // Default test - just verify it compiles and runs
    }

    #[test]
    fn test_sort_direction_serialization() {
        let asc = SortDirection::Asc;
        let desc = SortDirection::Desc;
        let default = SortDirection::Default;

        let asc_serialized = serde_json::to_string(&asc).unwrap();
        let desc_serialized = serde_json::to_string(&desc).unwrap();
        let default_serialized = serde_json::to_string(&default).unwrap();

        assert_eq!(asc_serialized, "\"asc\"");
        assert_eq!(desc_serialized, "\"desc\"");
        assert_eq!(default_serialized, "\"default\"");
    }

    #[test]
    fn test_sort_direction_deserialization() {
        let asc_json = "\"asc\"";
        let desc_json = "\"desc\"";
        let default_json = "\"default\"";

        let _asc: SortDirection = serde_json::from_str(asc_json).unwrap();
        let _desc: SortDirection = serde_json::from_str(desc_json).unwrap();
        let _default: SortDirection = serde_json::from_str(default_json).unwrap();

        // Deserialization test - just verify it compiles and runs without errors
    }

    #[test]
    fn test_sort_direction_display() {
        let asc = SortDirection::Asc;
        let desc = SortDirection::Desc;
        let default = SortDirection::Default;

        assert_eq!(format!("{}", asc), "asc");
        assert_eq!(format!("{}", desc), "desc");
        assert_eq!(format!("{}", default), "default");
    }

    #[test]
    fn test_sort_direction_debug() {
        let asc = SortDirection::Asc;
        let debug_str = format!("{:?}", asc);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_sort_direction_clone() {
        let asc = SortDirection::Asc;
        let _cloned = asc.clone();
        // Clone test - just verify it compiles and runs

        let desc = SortDirection::Desc;
        let _cloned = desc.clone();
        // Clone test - just verify it compiles and runs

        let default = SortDirection::Default;
        let _cloned = default.clone();
        // Clone test - just verify it compiles and runs
    }

    #[test]
    fn test_sort_direction_deserialization_error() {
        let invalid_json = "\"invalid\"";
        let result: Result<SortDirection, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complex_option_structure() {
        let instrument = create_mock_instrument();
        let ticker = create_mock_ticker_data();

        let call_option = OptionInstrument {
            instrument: instrument.clone(),
            ticker: ticker.clone(),
        };

        let put_option = OptionInstrument { instrument, ticker };

        let pair = OptionInstrumentPair {
            call: Some(call_option),
            put: Some(put_option),
        };

        let serialized = serde_json::to_string(&pair).unwrap();
        let deserialized: OptionInstrumentPair = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.call.is_some());
        assert!(deserialized.put.is_some());
    }

    #[test]
    fn test_greeks_with_option_data() {
        let greeks = Greeks {
            delta: Some(0.5),
            gamma: Some(0.02),
            vega: Some(0.1),
            theta: Some(-0.05),
            rho: Some(0.03),
        };

        // Test that Greeks can be used in combination with other structures
        #[derive(Serialize, Deserialize)]
        struct OptionWithGreeks {
            name: String,
            greeks: Greeks,
        }

        let option_with_greeks = OptionWithGreeks {
            name: "BTC-25DEC21-50000-C".to_string(),
            greeks,
        };

        let serialized = serde_json::to_string(&option_with_greeks).unwrap();
        let deserialized: OptionWithGreeks = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.name, "BTC-25DEC21-50000-C");
        assert_eq!(deserialized.greeks.delta, Some(0.5));
    }

    #[test]
    fn test_delivery_price_collections() {
        let delivery_prices = vec![
            DeliveryPriceData {
                date: "2021-12-24".to_string(),
                delivery_price: 49000.0,
            },
            DeliveryPriceData {
                date: "2021-12-25".to_string(),
                delivery_price: 50000.0,
            },
        ];

        let serialized = serde_json::to_string(&delivery_prices).unwrap();
        let deserialized: Vec<DeliveryPriceData> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.len(), 2);
        assert_eq!(deserialized[0].date, "2021-12-24");
        assert_eq!(deserialized[1].delivery_price, 50000.0);
    }
}
