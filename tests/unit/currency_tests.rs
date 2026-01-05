//! Unit tests for currency model

use deribit_http::model::currency::{
    Currency, CurrencyExpirations, CurrencyInfo, CurrencyInfoCollection, CurrencyStruct,
};
use deribit_http::model::withdrawal::WithdrawalPriority;
use serde_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_variants() {
        let btc = Currency::Btc;
        let eth = Currency::Eth;
        let usdc = Currency::Usdc;
        let usdt = Currency::Usdt;
        let eurr = Currency::Eurr;

        // Test that variants exist and can be created
        assert_eq!(format!("{}", btc), "BTC");
        assert_eq!(format!("{}", eth), "ETH");
        assert_eq!(format!("{}", usdc), "USDC");
        assert_eq!(format!("{}", usdt), "USDT");
        assert_eq!(format!("{}", eurr), "EURR");
    }

    #[test]
    fn test_currency_display() {
        assert_eq!(Currency::Btc.to_string(), "BTC");
        assert_eq!(Currency::Eth.to_string(), "ETH");
        assert_eq!(Currency::Usdc.to_string(), "USDC");
        assert_eq!(Currency::Usdt.to_string(), "USDT");
        assert_eq!(Currency::Eurr.to_string(), "EURR");
    }

    #[test]
    fn test_currency_serialization() {
        let btc = Currency::Btc;
        let json = serde_json::to_string(&btc).unwrap();
        let deserialized: Currency = serde_json::from_str(&json).unwrap();

        // Test serialization format
        assert_eq!(json, "\"BTC\"");

        // Test that deserialization works by checking display format
        assert_eq!(format!("{}", deserialized), "BTC");
    }

    #[test]
    fn test_currency_deserialization() {
        let json = "\"USDC\"";
        let usdc: Currency = serde_json::from_str(json).unwrap();

        assert_eq!(format!("{}", usdc), "USDC");
    }

    #[test]
    fn test_currency_clone() {
        let btc = Currency::Btc;
        let btc_clone = btc.clone();

        // Test that clone works by comparing display format
        assert_eq!(format!("{}", btc), format!("{}", btc_clone));
    }
}

#[cfg(test)]
mod currency_info_tests {
    use super::*;

    fn create_mock_currency_info() -> CurrencyInfo {
        CurrencyInfo::new(
            "BITCOIN".to_string(),
            "BTC".to_string(),
            "Bitcoin".to_string(),
            8,
            1,
            0.0005,
            0.001,
        )
    }

    fn create_withdrawal_priority() -> WithdrawalPriority {
        WithdrawalPriority {
            name: "high".to_string(),
            value: 1.0,
        }
    }

    #[test]
    fn test_currency_info_new() {
        let info = create_mock_currency_info();

        assert_eq!(info.coin_type, "BITCOIN");
        assert_eq!(info.currency, "BTC");
        assert_eq!(info.currency_long, "Bitcoin");
        assert_eq!(info.fee_precision, 8);
        assert_eq!(info.min_confirmations, 1);
        assert_eq!(info.min_withdrawal_fee, 0.0005);
        assert_eq!(info.withdrawal_fee, 0.001);
        assert!(info.withdrawal_priorities.is_empty());
        assert_eq!(info.disabled, None);
        assert_eq!(info.min_deposit_amount, None);
        assert_eq!(info.max_withdrawal_amount, None);
    }

    #[test]
    fn test_currency_info_add_priority() {
        let mut info = create_mock_currency_info();
        let priority = create_withdrawal_priority();

        info.add_priority(priority.clone());

        assert_eq!(info.withdrawal_priorities.len(), 1);
        assert_eq!(info.withdrawal_priorities[0].name, "high");
    }

    #[test]
    fn test_currency_info_with_disabled() {
        let info = create_mock_currency_info().with_disabled(true);

        assert_eq!(info.disabled, Some(true));
        assert!(!info.is_enabled());
    }

    #[test]
    fn test_currency_info_with_deposit_limit() {
        let info = create_mock_currency_info().with_deposit_limit(0.01);

        assert_eq!(info.min_deposit_amount, Some(0.01));
    }

    #[test]
    fn test_currency_info_with_withdrawal_limit() {
        let info = create_mock_currency_info().with_withdrawal_limit(100.0);

        assert_eq!(info.max_withdrawal_amount, Some(100.0));
    }

    #[test]
    fn test_currency_info_is_enabled() {
        let enabled_info = create_mock_currency_info();
        assert!(enabled_info.is_enabled()); // Default is enabled

        let disabled_info = create_mock_currency_info().with_disabled(true);
        assert!(!disabled_info.is_enabled());

        let explicitly_enabled = create_mock_currency_info().with_disabled(false);
        assert!(explicitly_enabled.is_enabled());
    }

    #[test]
    fn test_currency_info_get_priority() {
        let mut info = create_mock_currency_info();
        let priority1 = WithdrawalPriority {
            name: "low".to_string(),
            value: 0.5,
        };
        let priority2 = WithdrawalPriority {
            name: "high".to_string(),
            value: 1.5,
        };

        info.add_priority(priority1);
        info.add_priority(priority2);

        let found = info.get_priority("high");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "high");
        assert_eq!(found.unwrap().value, 1.5);

        let not_found = info.get_priority("medium");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_currency_info_highest_priority() {
        let mut info = create_mock_currency_info();
        let priority1 = WithdrawalPriority {
            name: "low".to_string(),
            value: 0.5,
        };
        let priority2 = WithdrawalPriority {
            name: "high".to_string(),
            value: 1.5,
        };
        let priority3 = WithdrawalPriority {
            name: "medium".to_string(),
            value: 1.0,
        };

        info.add_priority(priority1);
        info.add_priority(priority2);
        info.add_priority(priority3);

        let highest = info.highest_priority();
        assert!(highest.is_some());
        assert_eq!(highest.unwrap().name, "high");
        assert_eq!(highest.unwrap().value, 1.5);
    }

    #[test]
    fn test_currency_info_lowest_priority() {
        let mut info = create_mock_currency_info();
        let priority1 = WithdrawalPriority {
            name: "low".to_string(),
            value: 0.5,
        };
        let priority2 = WithdrawalPriority {
            name: "high".to_string(),
            value: 1.5,
        };
        let priority3 = WithdrawalPriority {
            name: "medium".to_string(),
            value: 1.0,
        };

        info.add_priority(priority1);
        info.add_priority(priority2);
        info.add_priority(priority3);

        let lowest = info.lowest_priority();
        assert!(lowest.is_some());
        assert_eq!(lowest.unwrap().name, "low");
        assert_eq!(lowest.unwrap().value, 0.5);
    }

    #[test]
    fn test_currency_info_no_priorities() {
        let info = create_mock_currency_info();

        assert!(info.highest_priority().is_none());
        assert!(info.lowest_priority().is_none());
        assert!(info.get_priority("any").is_none());
    }

    #[test]
    fn test_currency_info_serialization() {
        let info = create_mock_currency_info()
            .with_disabled(false)
            .with_deposit_limit(0.01)
            .with_withdrawal_limit(100.0);

        let json = serde_json::to_string(&info).unwrap();
        let deserialized: CurrencyInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(info, deserialized);
    }
}

#[cfg(test)]
mod currency_info_collection_tests {
    use super::*;

    fn create_mock_collection() -> CurrencyInfoCollection {
        let mut collection = CurrencyInfoCollection::new();

        let btc_info = CurrencyInfo::new(
            "BITCOIN".to_string(),
            "BTC".to_string(),
            "Bitcoin".to_string(),
            8,
            1,
            0.0005,
            0.001,
        );

        let eth_info = CurrencyInfo::new(
            "ETHEREUM".to_string(),
            "ETH".to_string(),
            "Ethereum".to_string(),
            18,
            12,
            0.005,
            0.01,
        )
        .with_disabled(true);

        let mut usdc_info = CurrencyInfo::new(
            "USD_COIN".to_string(),
            "USDC".to_string(),
            "USD Coin".to_string(),
            6,
            6,
            1.0,
            2.0,
        );
        usdc_info.add_priority(WithdrawalPriority {
            name: "standard".to_string(),
            value: 1.0,
        });

        collection.add(btc_info);
        collection.add(eth_info);
        collection.add(usdc_info);

        collection
    }

    #[test]
    fn test_currency_info_collection_new() {
        let collection = CurrencyInfoCollection::new();
        assert!(collection.currencies.is_empty());
    }

    #[test]
    fn test_currency_info_collection_default() {
        let collection = CurrencyInfoCollection::default();
        assert!(collection.currencies.is_empty());
    }

    #[test]
    fn test_currency_info_collection_add() {
        let mut collection = CurrencyInfoCollection::new();
        let info = CurrencyInfo::new(
            "BITCOIN".to_string(),
            "BTC".to_string(),
            "Bitcoin".to_string(),
            8,
            1,
            0.0005,
            0.001,
        );

        collection.add(info);
        assert_eq!(collection.currencies.len(), 1);
    }

    #[test]
    fn test_currency_info_collection_get() {
        let collection = create_mock_collection();

        let btc = collection.get("BTC".to_string());
        assert!(btc.is_some());
        assert_eq!(btc.unwrap().currency, "BTC");

        let not_found = collection.get("XRP".to_string());
        assert!(not_found.is_none());
    }

    #[test]
    fn test_currency_info_collection_enabled() {
        let collection = create_mock_collection();
        let enabled = collection.enabled();

        assert_eq!(enabled.len(), 2); // BTC and USDC are enabled, ETH is disabled
        assert!(enabled.iter().all(|c| c.is_enabled()));
    }

    #[test]
    fn test_currency_info_collection_with_withdrawal() {
        let collection = create_mock_collection();
        let with_withdrawal = collection.with_withdrawal();

        assert_eq!(with_withdrawal.len(), 1); // Only USDC has withdrawal priorities
        assert!(
            with_withdrawal
                .iter()
                .all(|c| !c.withdrawal_priorities.is_empty())
        );
    }

    #[test]
    fn test_currency_info_collection_serialization() {
        let collection = create_mock_collection();

        let json = serde_json::to_string(&collection).unwrap();
        let deserialized: CurrencyInfoCollection = serde_json::from_str(&json).unwrap();

        assert_eq!(collection, deserialized);
    }
}

#[cfg(test)]
mod currency_struct_tests {
    use super::*;

    #[test]
    fn test_currency_struct_serialization() {
        let currency_struct = CurrencyStruct {
            currency: "BTC".to_string(),
            currency_long: "Bitcoin".to_string(),
            decimals: Some(8),
            fee_precision: Some(8),
            min_confirmations: 1,
            min_withdrawal_fee: 0.0005,
            withdrawal_fee: 0.001,
            withdrawal_priorities: vec![WithdrawalPriority {
                name: "standard".to_string(),
                value: 1.0,
            }],
            apr: Some(0.05),
            coin_type: Some("BTC".to_string()),
            network_fee: Some(0.000003),
            network_currency: Some("BTC".to_string()),
            in_cross_collateral_pool: Some(true),
        };

        let json = serde_json::to_string(&currency_struct).unwrap();
        let deserialized: CurrencyStruct = serde_json::from_str(&json).unwrap();

        assert_eq!(currency_struct.currency, deserialized.currency);
        assert_eq!(currency_struct.currency_long, deserialized.currency_long);
        assert_eq!(currency_struct.fee_precision, deserialized.fee_precision);
        assert_eq!(
            currency_struct.min_confirmations,
            deserialized.min_confirmations
        );
        assert_eq!(
            currency_struct.min_withdrawal_fee,
            deserialized.min_withdrawal_fee
        );
        assert_eq!(currency_struct.withdrawal_fee, deserialized.withdrawal_fee);
        assert_eq!(
            currency_struct.withdrawal_priorities.len(),
            deserialized.withdrawal_priorities.len()
        );
        assert_eq!(currency_struct.apr, deserialized.apr);
    }

    #[test]
    fn test_currency_struct_optional_fields() {
        let currency_struct = CurrencyStruct {
            currency: "ETH".to_string(),
            currency_long: "Ethereum".to_string(),
            decimals: None,
            fee_precision: None,
            min_confirmations: 12,
            min_withdrawal_fee: 0.005,
            withdrawal_fee: 0.01,
            withdrawal_priorities: vec![],
            apr: None, // Test None case
            coin_type: None,
            network_fee: None,
            network_currency: None,
            in_cross_collateral_pool: None,
        };

        let json = serde_json::to_string(&currency_struct).unwrap();
        let deserialized: CurrencyStruct = serde_json::from_str(&json).unwrap();

        assert_eq!(currency_struct.apr, None);
        assert_eq!(deserialized.apr, None);
    }
}

#[cfg(test)]
mod currency_expirations_tests {
    use super::*;

    #[test]
    fn test_currency_expirations_serialization() {
        let expirations = CurrencyExpirations {
            future: Some(vec!["25DEC21".to_string(), "31DEC21".to_string()]),
            option: Some(vec![
                "25DEC21".to_string(),
                "31DEC21".to_string(),
                "07JAN22".to_string(),
            ]),
        };

        let json = serde_json::to_string(&expirations).unwrap();
        let deserialized: CurrencyExpirations = serde_json::from_str(&json).unwrap();

        assert_eq!(expirations.future, deserialized.future);
        assert_eq!(expirations.option, deserialized.option);
    }

    #[test]
    fn test_currency_expirations_none_values() {
        let expirations = CurrencyExpirations {
            future: None,
            option: None,
        };

        let json = serde_json::to_string(&expirations).unwrap();
        let deserialized: CurrencyExpirations = serde_json::from_str(&json).unwrap();

        assert_eq!(expirations.future, None);
        assert_eq!(expirations.option, None);
        assert_eq!(deserialized.future, None);
        assert_eq!(deserialized.option, None);
    }

    #[test]
    fn test_currency_expirations_mixed_values() {
        let expirations = CurrencyExpirations {
            future: Some(vec!["25DEC21".to_string()]),
            option: None,
        };

        let json = serde_json::to_string(&expirations).unwrap();
        let deserialized: CurrencyExpirations = serde_json::from_str(&json).unwrap();

        assert!(expirations.future.is_some());
        assert_eq!(expirations.future.as_ref().unwrap().len(), 1);
        assert!(expirations.option.is_none());
        assert_eq!(deserialized.future, expirations.future);
        assert_eq!(deserialized.option, expirations.option);
    }
}
