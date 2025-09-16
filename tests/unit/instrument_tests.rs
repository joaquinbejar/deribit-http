use deribit_http::model::instrument::{Instrument, InstrumentKind, InstrumentType, OptionType};
use serde_json;

#[cfg(test)]
mod instrument_kind_tests {
    use super::*;

    #[test]
    fn test_instrument_kind_serialization() {
        let future = InstrumentKind::Future;
        let option = InstrumentKind::Option;
        let spot = InstrumentKind::Spot;
        let future_combo = InstrumentKind::FutureCombo;
        let option_combo = InstrumentKind::OptionCombo;

        assert_eq!(serde_json::to_string(&future).unwrap(), "\"future\"");
        assert_eq!(serde_json::to_string(&option).unwrap(), "\"option\"");
        assert_eq!(serde_json::to_string(&spot).unwrap(), "\"spot\"");
        assert_eq!(
            serde_json::to_string(&future_combo).unwrap(),
            "\"future_combo\""
        );
        assert_eq!(
            serde_json::to_string(&option_combo).unwrap(),
            "\"option_combo\""
        );
    }

    #[test]
    fn test_instrument_kind_deserialization() {
        let future: InstrumentKind = serde_json::from_str("\"future\"").unwrap();
        let option: InstrumentKind = serde_json::from_str("\"option\"").unwrap();
        let spot: InstrumentKind = serde_json::from_str("\"spot\"").unwrap();
        let future_combo: InstrumentKind = serde_json::from_str("\"future_combo\"").unwrap();
        let option_combo: InstrumentKind = serde_json::from_str("\"option_combo\"").unwrap();

        assert_eq!(future, InstrumentKind::Future);
        assert_eq!(option, InstrumentKind::Option);
        assert_eq!(spot, InstrumentKind::Spot);
        assert_eq!(future_combo, InstrumentKind::FutureCombo);
        assert_eq!(option_combo, InstrumentKind::OptionCombo);
    }

    #[test]
    fn test_instrument_kind_display() {
        assert_eq!(format!("{}", InstrumentKind::Future), "future");
        assert_eq!(format!("{}", InstrumentKind::Option), "option");
        assert_eq!(format!("{}", InstrumentKind::Spot), "spot");
        assert_eq!(format!("{}", InstrumentKind::FutureCombo), "future_combo");
        assert_eq!(format!("{}", InstrumentKind::OptionCombo), "option_combo");
    }
}

#[cfg(test)]
mod instrument_type_tests {
    use super::*;

    #[test]
    fn test_instrument_type_serialization() {
        let linear = InstrumentType::Linear;
        let reversed = InstrumentType::Reversed;

        assert_eq!(serde_json::to_string(&linear).unwrap(), "\"linear\"");
        assert_eq!(serde_json::to_string(&reversed).unwrap(), "\"reversed\"");
    }

    #[test]
    fn test_instrument_type_deserialization() {
        let linear: InstrumentType = serde_json::from_str("\"linear\"").unwrap();
        let reversed: InstrumentType = serde_json::from_str("\"reversed\"").unwrap();

        assert_eq!(linear, InstrumentType::Linear);
        assert_eq!(reversed, InstrumentType::Reversed);
    }
}

#[cfg(test)]
mod option_type_tests {
    use super::*;

    #[test]
    fn test_option_type_serialization() {
        let call = OptionType::Call;
        let put = OptionType::Put;

        assert_eq!(serde_json::to_string(&call).unwrap(), "\"call\"");
        assert_eq!(serde_json::to_string(&put).unwrap(), "\"put\"");
    }

    #[test]
    fn test_option_type_deserialization() {
        let call: OptionType = serde_json::from_str("\"call\"").unwrap();
        let put: OptionType = serde_json::from_str("\"put\"").unwrap();

        assert_eq!(call, OptionType::Call);
        assert_eq!(put, OptionType::Put);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_perpetual_instrument() -> Instrument {
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
            min_trade_amount: Some(10.0),
            contract_size: Some(1.0),
            settlement_period: Some("perpetual".to_string()),
            instrument_type: Some(InstrumentType::Linear),
            quote_currency: Some("USD".to_string()),
            settlement_currency: Some("BTC".to_string()),
            creation_timestamp: Some(1609459200000),
            max_leverage: Some(100.0),
            maker_commission: Some(0.0001),
            taker_commission: Some(0.0005),
            instrument_id: Some(12345),
            base_currency: Some("BTC".to_string()),
            counter_currency: Some("USD".to_string()),
        }
    }

    fn create_mock_option_instrument() -> Instrument {
        Instrument {
            instrument_name: "BTC-25JUL25-50000-C".to_string(),
            price_index: Some("btc_usd".to_string()),
            kind: Some(InstrumentKind::Option),
            currency: Some("BTC".to_string()),
            is_active: Some(true),
            expiration_timestamp: Some(1640419200000),
            strike: Some(50000.0),
            option_type: Some(OptionType::Call),
            tick_size: Some(0.0005),
            min_trade_amount: Some(0.1),
            contract_size: Some(1.0),
            settlement_period: Some("month".to_string()),
            instrument_type: Some(InstrumentType::Reversed),
            quote_currency: Some("USD".to_string()),
            settlement_currency: Some("BTC".to_string()),
            creation_timestamp: Some(1609459200000),
            max_leverage: Some(1.0),
            maker_commission: Some(0.0003),
            taker_commission: Some(0.0003),
            instrument_id: Some(67890),
            base_currency: Some("BTC".to_string()),
            counter_currency: Some("USD".to_string()),
        }
    }

    #[test]
    fn test_instrument_default() {
        let instrument = Instrument::default();
        assert_eq!(instrument.instrument_name, "");
        assert_eq!(instrument.price_index, None);
        assert_eq!(instrument.kind, None);
        assert_eq!(instrument.currency, None);
        assert_eq!(instrument.is_active, None);
    }

    #[test]
    fn test_instrument_is_perpetual() {
        let perpetual = create_mock_perpetual_instrument();
        let option = create_mock_option_instrument();

        assert!(perpetual.is_perpetual());
        assert!(!option.is_perpetual());

        // Test with no kind
        let mut no_kind = create_mock_perpetual_instrument();
        no_kind.kind = None;
        assert!(!no_kind.is_perpetual());

        // Test with expiration timestamp
        let mut with_expiration = create_mock_perpetual_instrument();
        with_expiration.expiration_timestamp = Some(1640419200000);
        assert!(!with_expiration.is_perpetual());
    }

    #[test]
    fn test_instrument_is_option() {
        let perpetual = create_mock_perpetual_instrument();
        let option = create_mock_option_instrument();

        assert!(!perpetual.is_option());
        assert!(option.is_option());

        // Test with option combo
        let mut option_combo = create_mock_option_instrument();
        option_combo.kind = Some(InstrumentKind::OptionCombo);
        assert!(option_combo.is_option());
    }

    #[test]
    fn test_instrument_is_future() {
        let perpetual = create_mock_perpetual_instrument();
        let option = create_mock_option_instrument();

        assert!(perpetual.is_future());
        assert!(!option.is_future());

        // Test with future combo
        let mut future_combo = create_mock_perpetual_instrument();
        future_combo.kind = Some(InstrumentKind::FutureCombo);
        assert!(future_combo.is_future());
    }

    #[test]
    fn test_instrument_is_spot() {
        let perpetual = create_mock_perpetual_instrument();
        let option = create_mock_option_instrument();

        assert!(!perpetual.is_spot());
        assert!(!option.is_spot());

        // Test with spot
        let mut spot = create_mock_perpetual_instrument();
        spot.kind = Some(InstrumentKind::Spot);
        assert!(spot.is_spot());
    }

    #[test]
    fn test_instrument_serialization() {
        let instrument = create_mock_perpetual_instrument();

        let json = serde_json::to_string(&instrument).unwrap();
        let deserialized: Instrument = serde_json::from_str(&json).unwrap();

        assert_eq!(instrument.instrument_name, deserialized.instrument_name);
        assert_eq!(instrument.kind, deserialized.kind);
        assert_eq!(instrument.currency, deserialized.currency);
        assert_eq!(instrument.is_active, deserialized.is_active);
        assert_eq!(
            instrument.expiration_timestamp,
            deserialized.expiration_timestamp
        );
        assert_eq!(instrument.strike, deserialized.strike);
        assert_eq!(instrument.option_type, deserialized.option_type);
    }

    #[test]
    fn test_instrument_option_serialization() {
        let option = create_mock_option_instrument();

        let json = serde_json::to_string(&option).unwrap();
        let deserialized: Instrument = serde_json::from_str(&json).unwrap();

        assert_eq!(option.option_type, deserialized.option_type);
        assert_eq!(option.strike, deserialized.strike);
        assert_eq!(
            option.expiration_timestamp,
            deserialized.expiration_timestamp
        );
    }

    #[test]
    fn test_instrument_edge_cases() {
        // Test instrument with minimal fields
        let minimal_instrument = Instrument {
            instrument_name: "".to_string(),
            ..Default::default()
        };

        assert!(!minimal_instrument.is_perpetual());
        assert!(!minimal_instrument.is_option());
        assert!(!minimal_instrument.is_future());
        assert!(!minimal_instrument.is_spot());
    }

    #[test]
    fn test_instrument_clone_debug() {
        let instrument = create_mock_perpetual_instrument();
        let cloned = instrument.clone();

        assert_eq!(instrument.instrument_name, cloned.instrument_name);
        assert_eq!(instrument.kind, cloned.kind);

        // Test Debug trait (uses DebugPretty)
        let debug_str = format!("{:?}", instrument);
        assert!(debug_str.contains("BTC-PERPETUAL"));
        assert!(debug_str.contains("perpetual"));
    }
}
