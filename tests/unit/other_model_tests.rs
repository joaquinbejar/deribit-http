use deribit_http::model::OptionType;
use deribit_http::model::other::{
    DeliveryPriceData, Greeks, OptionInstrument, OptionInstrumentPair, ParsedOptionWithTicker,
    SortDirection,
};
use serde_json;

// Mock functions for dependencies
fn create_mock_instrument() -> deribit_http::model::instrument::Instrument {
    use deribit_http::model::instrument::{InstrumentKind, InstrumentType};

    deribit_http::model::instrument::Instrument {
        instrument_name: "BTC-25DEC21-50000-C".to_string(),
        price_index: Some("btc_usd".to_string()),
        kind: Some(InstrumentKind::Option),
        currency: Some("BTC".to_string()),
        is_active: Some(true),
        expiration_timestamp: Some(1640390400000),
        strike: Some(50000.0),
        option_type: Some(OptionType::Call),
        tick_size: Some(0.5),
        min_trade_amount: Some(10.0),
        contract_size: Some(1.0),
        settlement_period: Some("month".to_string()),
        instrument_type: Some(InstrumentType::Linear),
        quote_currency: Some("USD".to_string()),
        settlement_currency: Some("BTC".to_string()),
        creation_timestamp: Some(1234567890),
        max_leverage: Some(1.0),
        maker_commission: Some(0.0001),
        taker_commission: Some(0.0005),
        instrument_id: Some(1),
        base_currency: Some("BTC".to_string()),
        counter_currency: Some("USD".to_string()),
    }
}

fn create_mock_ticker() -> deribit_http::model::ticker::TickerData {
    use deribit_http::model::ticker::TickerStats;

    deribit_http::model::ticker::TickerData {
        instrument_name: "BTC-25DEC21-50000-C".to_string(),
        last_price: Some(50250.0),
        mark_price: 50500.0,
        best_bid_price: Some(50200.0),
        best_ask_price: Some(50300.0),
        best_bid_amount: 10.0,
        best_ask_amount: 15.0,
        volume: Some(100.0),
        volume_usd: Some(1000000.0),
        open_interest: Some(1000.0),
        high: Some(55000.0),
        low: Some(45000.0),
        price_change: Some(0.05),
        price_change_percentage: Some(0.1),
        bid_iv: Some(0.24),
        ask_iv: Some(0.25),
        mark_iv: Some(0.245),
        timestamp: 1640995200000,
        state: "open".to_string(),
        settlement_price: Some(50000.0),
        stats: TickerStats {
            volume: 100.0,
            volume_usd: Some(1000000.0),
            price_change: Some(0.05),
            high: Some(55000.0),
            low: Some(45000.0),
        },
        greeks: None,
        index_price: Some(50000.0),
        min_price: Some(45000.0),
        max_price: Some(55000.0),
        interest_rate: Some(0.05),
        underlying_price: Some(50000.0),
        underlying_index: Some("btc_usd".to_string()),
        estimated_delivery_price: Some(50000.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_price_data_serialization() {
        let data = DeliveryPriceData {
            date: "2021-12-25".to_string(),
            delivery_price: 50000.0,
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: DeliveryPriceData = serde_json::from_str(&serialized).unwrap();

        assert_eq!(data.date, deserialized.date);
        assert_eq!(data.delivery_price, deserialized.delivery_price);
    }

    #[test]
    fn test_delivery_price_data_clone() {
        let data = DeliveryPriceData {
            date: "2021-12-25".to_string(),
            delivery_price: 50000.0,
        };

        let cloned = data.clone();
        assert_eq!(data.date, cloned.date);
        assert_eq!(data.delivery_price, cloned.delivery_price);
    }

    #[test]
    fn test_greeks_serialization_with_all_values() {
        let greeks = Greeks {
            delta: Some(0.5),
            gamma: Some(0.01),
            vega: Some(0.1),
            theta: Some(-0.05),
            rho: Some(0.02),
        };

        let serialized = serde_json::to_string(&greeks).unwrap();
        let deserialized: Greeks = serde_json::from_str(&serialized).unwrap();

        assert_eq!(greeks.delta, deserialized.delta);
        assert_eq!(greeks.gamma, deserialized.gamma);
        assert_eq!(greeks.vega, deserialized.vega);
        assert_eq!(greeks.theta, deserialized.theta);
        assert_eq!(greeks.rho, deserialized.rho);
    }

    #[test]
    fn test_greeks_serialization_with_none_values() {
        let greeks = Greeks {
            delta: Some(0.5),
            gamma: None,
            vega: Some(0.1),
            theta: None,
            rho: None,
        };

        let serialized = serde_json::to_string(&greeks).unwrap();
        let deserialized: Greeks = serde_json::from_str(&serialized).unwrap();

        assert_eq!(greeks.delta, deserialized.delta);
        assert_eq!(greeks.gamma, deserialized.gamma);
        assert_eq!(greeks.vega, deserialized.vega);
        assert_eq!(greeks.theta, deserialized.theta);
        assert_eq!(greeks.rho, deserialized.rho);
    }

    #[test]
    fn test_greeks_clone() {
        let greeks = Greeks {
            delta: Some(0.5),
            gamma: Some(0.01),
            vega: Some(0.1),
            theta: Some(-0.05),
            rho: Some(0.02),
        };

        let cloned = greeks.clone();
        assert_eq!(greeks.delta, cloned.delta);
        assert_eq!(greeks.gamma, cloned.gamma);
        assert_eq!(greeks.vega, cloned.vega);
        assert_eq!(greeks.theta, cloned.theta);
        assert_eq!(greeks.rho, cloned.rho);
    }

    #[test]
    fn test_option_instrument_serialization() {
        let option_instrument = OptionInstrument {
            instrument: create_mock_instrument(),
            ticker: create_mock_ticker(),
        };

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
        let option_instrument = OptionInstrument {
            instrument: create_mock_instrument(),
            ticker: create_mock_ticker(),
        };

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

    #[test]
    fn test_option_instrument_pair_with_both_options() {
        let call_option = OptionInstrument {
            instrument: create_mock_instrument(),
            ticker: create_mock_ticker(),
        };

        let mut put_instrument = create_mock_instrument();
        put_instrument.option_type = Some(OptionType::Put);
        put_instrument.instrument_name = "BTC-25DEC21-50000-P".to_string();

        let mut put_ticker = create_mock_ticker();
        put_ticker.instrument_name = "BTC-25DEC21-50000-P".to_string();

        let put_option = OptionInstrument {
            instrument: put_instrument,
            ticker: put_ticker,
        };

        let pair = OptionInstrumentPair {
            call: Some(call_option),
            put: Some(put_option),
        };

        let serialized = serde_json::to_string(&pair).unwrap();
        let deserialized: OptionInstrumentPair = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.call.is_some());
        assert!(deserialized.put.is_some());
        assert_eq!(
            deserialized.call.as_ref().unwrap().instrument.option_type,
            Some(OptionType::Call)
        );
        assert_eq!(
            deserialized.put.as_ref().unwrap().instrument.option_type,
            Some(OptionType::Put)
        );
    }

    #[test]
    fn test_option_instrument_pair_with_only_call() {
        let call_option = OptionInstrument {
            instrument: create_mock_instrument(),
            ticker: create_mock_ticker(),
        };

        let pair = OptionInstrumentPair {
            call: Some(call_option),
            put: None,
        };

        let serialized = serde_json::to_string(&pair).unwrap();
        let deserialized: OptionInstrumentPair = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.call.is_some());
        assert!(deserialized.put.is_none());
    }

    #[test]
    fn test_parsed_option_with_ticker_serialization() {
        let parsed_option = ParsedOptionWithTicker {
            instrument_name: "BTC-25DEC21-50000-C".to_string(),
            strike: 50000.0,
            option_type: OptionType::Call,
            expiry: "25DEC21".to_string(),
            ticker: create_mock_ticker(),
        };

        let serialized = serde_json::to_string(&parsed_option).unwrap();
        assert!(serialized.contains("BTC-25DEC21-50000-C"));
        assert!(serialized.contains("50000"));
        assert!(serialized.contains("call"));
    }

    #[test]
    fn test_parsed_option_with_ticker_clone() {
        let parsed_option = ParsedOptionWithTicker {
            instrument_name: "BTC-25DEC21-50000-C".to_string(),
            strike: 50000.0,
            option_type: OptionType::Call,
            expiry: "25DEC21".to_string(),
            ticker: create_mock_ticker(),
        };

        let cloned = parsed_option.clone();
        assert_eq!(parsed_option.instrument_name, cloned.instrument_name);
        assert_eq!(parsed_option.strike, cloned.strike);
        assert_eq!(parsed_option.option_type, cloned.option_type);
        assert_eq!(parsed_option.expiry, cloned.expiry);
    }

    #[test]
    fn test_sort_direction_serialization() {
        let asc = SortDirection::Asc;
        let desc = SortDirection::Desc;
        let default = SortDirection::Default;

        assert_eq!(serde_json::to_string(&asc).unwrap(), "\"asc\"");
        assert_eq!(serde_json::to_string(&desc).unwrap(), "\"desc\"");
        assert_eq!(serde_json::to_string(&default).unwrap(), "\"default\"");
    }

    #[test]
    fn test_sort_direction_deserialization() {
        let asc: SortDirection = serde_json::from_str("\"asc\"").unwrap();
        let desc: SortDirection = serde_json::from_str("\"desc\"").unwrap();
        let default: SortDirection = serde_json::from_str("\"default\"").unwrap();

        assert!(matches!(asc, SortDirection::Asc));
        assert!(matches!(desc, SortDirection::Desc));
        assert!(matches!(default, SortDirection::Default));
    }

    #[test]
    fn test_sort_direction_display() {
        assert_eq!(format!("{}", SortDirection::Asc), "asc");
        assert_eq!(format!("{}", SortDirection::Desc), "desc");
        assert_eq!(format!("{}", SortDirection::Default), "default");
    }

    #[test]
    fn test_sort_direction_default() {
        let default_sort = SortDirection::default();
        assert!(matches!(default_sort, SortDirection::Asc));
    }

    #[test]
    fn test_sort_direction_clone() {
        let sort = SortDirection::Desc;
        let cloned = sort.clone();
        assert!(matches!(cloned, SortDirection::Desc));
    }

    #[test]
    fn test_invalid_sort_direction_deserialization() {
        let result: Result<SortDirection, _> = serde_json::from_str("\"invalid\"");
        assert!(result.is_err());
    }
}
