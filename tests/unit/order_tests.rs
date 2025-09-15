use deribit_http::model::order::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[cfg(test)]
mod order_status_tests {
    use super::*;

    #[test]
    fn test_order_status_serialization() {
        let status = OrderStatus::New;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"New\"");
    }

    #[test]
    fn test_order_status_deserialization() {
        let json = "\"Filled\"";
        let status: OrderStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, OrderStatus::Filled);
    }

    #[test]
    fn test_all_order_statuses_serialization() {
        let statuses = vec![
            OrderStatus::New,
            OrderStatus::PartiallyFilled,
            OrderStatus::Filled,
            OrderStatus::DoneForDay,
            OrderStatus::Canceled,
            OrderStatus::Replaced,
            OrderStatus::PendingCancel,
            OrderStatus::Stopped,
            OrderStatus::Rejected,
            OrderStatus::Suspended,
            OrderStatus::PendingNew,
            OrderStatus::Calculated,
            OrderStatus::Expired,
            OrderStatus::AcceptedForBidding,
            OrderStatus::PendingReplace,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: OrderStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_order_status_clone() {
        let status = OrderStatus::PartiallyFilled;
        let cloned = status;
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_order_status_copy() {
        let status = OrderStatus::Canceled;
        let copied = status;
        assert_eq!(status, copied);
    }

    #[test]
    fn test_order_status_partial_eq() {
        assert_eq!(OrderStatus::New, OrderStatus::New);
        assert_ne!(OrderStatus::New, OrderStatus::Filled);
    }

    #[test]
    fn test_order_status_eq() {
        let status1 = OrderStatus::Rejected;
        let status2 = OrderStatus::Rejected;
        let status3 = OrderStatus::Suspended;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_order_status_debug() {
        let status = OrderStatus::PendingNew;
        let debug_str = format!("{:?}", status);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_order_status_display() {
        let status = OrderStatus::AcceptedForBidding;
        let display_str = format!("{}", status);
        assert!(!display_str.is_empty());
    }
}

#[cfg(test)]
mod order_side_tests {
    use super::*;

    #[test]
    fn test_order_side_serialization() {
        let buy_side = OrderSide::Buy;
        let sell_side = OrderSide::Sell;

        let buy_serialized = serde_json::to_string(&buy_side).unwrap();
        let sell_serialized = serde_json::to_string(&sell_side).unwrap();

        assert_eq!(buy_serialized, "\"Buy\"");
        assert_eq!(sell_serialized, "\"Sell\"");
    }

    #[test]
    fn test_order_side_deserialization() {
        let buy_json = "\"Buy\"";
        let sell_json = "\"Sell\"";

        let buy_side: OrderSide = serde_json::from_str(buy_json).unwrap();
        let sell_side: OrderSide = serde_json::from_str(sell_json).unwrap();

        assert_eq!(buy_side, OrderSide::Buy);
        assert_eq!(sell_side, OrderSide::Sell);
    }

    #[test]
    fn test_order_side_round_trip_serialization() {
        let sides = vec![OrderSide::Buy, OrderSide::Sell];

        for side in sides {
            let serialized = serde_json::to_string(&side).unwrap();
            let deserialized: OrderSide = serde_json::from_str(&serialized).unwrap();
            assert_eq!(side, deserialized);
        }
    }

    #[test]
    fn test_order_side_clone() {
        let side = OrderSide::Buy;
        let cloned = side;
        assert_eq!(side, cloned);
    }

    #[test]
    fn test_order_side_copy() {
        let side = OrderSide::Sell;
        let copied = side;
        assert_eq!(side, copied);
    }

    #[test]
    fn test_order_side_partial_eq() {
        assert_eq!(OrderSide::Buy, OrderSide::Buy);
        assert_eq!(OrderSide::Sell, OrderSide::Sell);
        assert_ne!(OrderSide::Buy, OrderSide::Sell);
    }

    #[test]
    fn test_order_side_debug() {
        let buy_side = OrderSide::Buy;
        let sell_side = OrderSide::Sell;

        let buy_debug = format!("{:?}", buy_side);
        let sell_debug = format!("{:?}", sell_side);

        assert!(!buy_debug.is_empty());
        assert!(!sell_debug.is_empty());
    }

    #[test]
    fn test_order_side_display() {
        let buy_side = OrderSide::Buy;
        let sell_side = OrderSide::Sell;

        let buy_display = format!("{}", buy_side);
        let sell_display = format!("{}", sell_side);

        assert!(!buy_display.is_empty());
        assert!(!sell_display.is_empty());
    }
}

#[cfg(test)]
mod order_type_tests {
    use super::*;

    #[test]
    fn test_order_type_serialization() {
        let limit_order = OrderType::Limit;
        let market_order = OrderType::Market;

        let limit_serialized = serde_json::to_string(&limit_order).unwrap();
        let market_serialized = serde_json::to_string(&market_order).unwrap();

        assert_eq!(limit_serialized, "\"limit\"");
        assert_eq!(market_serialized, "\"market\"");
    }

    #[test]
    fn test_order_type_deserialization() {
        let limit_json = "\"limit\"";
        let market_json = "\"market\"";

        let limit_type: OrderType = serde_json::from_str(limit_json).unwrap();
        let market_type: OrderType = serde_json::from_str(market_json).unwrap();

        assert_eq!(limit_type, OrderType::Limit);
        assert_eq!(market_type, OrderType::Market);
    }

    #[test]
    fn test_all_order_types_serialization() {
        let order_types = vec![
            OrderType::Limit,
            OrderType::Market,
            OrderType::StopLimit,
            OrderType::StopMarket,
            OrderType::TakeLimit,
            OrderType::TakeMarket,
            OrderType::MarketLimit,
            OrderType::TrailingStop,
        ];

        for order_type in order_types {
            let serialized = serde_json::to_string(&order_type).unwrap();
            let deserialized: OrderType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(order_type, deserialized);
        }
    }

    #[test]
    fn test_order_type_as_str() {
        assert_eq!(OrderType::Limit.as_str(), "limit");
        assert_eq!(OrderType::Market.as_str(), "market");
        assert_eq!(OrderType::StopLimit.as_str(), "stop_limit");
        assert_eq!(OrderType::StopMarket.as_str(), "stop_market");
        assert_eq!(OrderType::TakeLimit.as_str(), "take_limit");
        assert_eq!(OrderType::TakeMarket.as_str(), "take_market");
        assert_eq!(OrderType::MarketLimit.as_str(), "market_limit");
        assert_eq!(OrderType::TrailingStop.as_str(), "trailing_stop");
    }

    #[test]
    fn test_order_type_as_str_matches_serialization() {
        let order_types = vec![
            OrderType::Limit,
            OrderType::Market,
            OrderType::StopLimit,
            OrderType::StopMarket,
            OrderType::TakeLimit,
            OrderType::TakeMarket,
            OrderType::MarketLimit,
            OrderType::TrailingStop,
        ];

        for order_type in order_types {
            let serialized = serde_json::to_string(&order_type).unwrap();
            let expected = format!("\"{}\"", order_type.as_str());
            assert_eq!(serialized, expected);
        }
    }

    #[test]
    fn test_order_type_clone() {
        let order_type = OrderType::StopLimit;
        let cloned = order_type;
        assert_eq!(order_type, cloned);
    }

    #[test]
    fn test_order_type_copy() {
        let order_type = OrderType::TakeMarket;
        let copied = order_type;
        assert_eq!(order_type, copied);
    }

    #[test]
    fn test_order_type_partial_eq() {
        assert_eq!(OrderType::Limit, OrderType::Limit);
        assert_eq!(OrderType::Market, OrderType::Market);
        assert_ne!(OrderType::Limit, OrderType::Market);
        assert_ne!(OrderType::StopLimit, OrderType::StopMarket);
    }

    #[test]
    fn test_order_type_debug() {
        let order_type = OrderType::TrailingStop;
        let debug_str = format!("{:?}", order_type);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_order_type_display() {
        let order_type = OrderType::MarketLimit;
        let display_str = format!("{}", order_type);
        assert!(!display_str.is_empty());
    }

    #[test]
    fn test_order_type_serde_rename() {
        // Test that serde rename attributes work correctly
        let stop_limit = OrderType::StopLimit;
        let serialized = serde_json::to_string(&stop_limit).unwrap();
        assert_eq!(serialized, "\"stop_limit\"");

        let deserialized: OrderType = serde_json::from_str("\"stop_limit\"").unwrap();
        assert_eq!(deserialized, OrderType::StopLimit);
    }

    #[test]
    fn test_order_type_deserialization_error() {
        let invalid_json = "\"invalid_order_type\"";
        let result: Result<OrderType, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_order_enums_in_struct() {
        // Test that all enums work together in a hypothetical order struct
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct MockOrder {
            status: OrderStatus,
            side: OrderSide,
            order_type: OrderType,
        }

        let order = MockOrder {
            status: OrderStatus::PartiallyFilled,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
        };

        let serialized = serde_json::to_string(&order).unwrap();
        let deserialized: MockOrder = serde_json::from_str(&serialized).unwrap();

        assert_eq!(order, deserialized);
        assert!(serialized.contains("\"status\":\"PartiallyFilled\""));
        assert!(serialized.contains("\"side\":\"Buy\""));
        assert!(serialized.contains("\"order_type\":\"limit\""));
    }

    #[test]
    fn test_order_enums_collections() {
        let statuses = vec![OrderStatus::New, OrderStatus::Filled, OrderStatus::Canceled];
        let sides = vec![OrderSide::Buy, OrderSide::Sell];
        let types = vec![OrderType::Limit, OrderType::Market, OrderType::StopLimit];

        let statuses_serialized = serde_json::to_string(&statuses).unwrap();
        let sides_serialized = serde_json::to_string(&sides).unwrap();
        let types_serialized = serde_json::to_string(&types).unwrap();

        let statuses_deserialized: Vec<OrderStatus> =
            serde_json::from_str(&statuses_serialized).unwrap();
        let sides_deserialized: Vec<OrderSide> = serde_json::from_str(&sides_serialized).unwrap();
        let types_deserialized: Vec<OrderType> = serde_json::from_str(&types_serialized).unwrap();

        assert_eq!(statuses, statuses_deserialized);
        assert_eq!(sides, sides_deserialized);
        assert_eq!(types, types_deserialized);
    }

    #[test]
    fn test_order_enums_match_patterns() {
        let status = OrderStatus::New;
        let side = OrderSide::Buy;
        let order_type = OrderType::Limit;

        let status_str = match status {
            OrderStatus::New => "new",
            OrderStatus::Filled => "filled",
            _ => "other",
        };

        let side_str = match side {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        };

        let type_str = match order_type {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
            _ => "other",
        };

        assert_eq!(status_str, "new");
        assert_eq!(side_str, "buy");
        assert_eq!(type_str, "limit");
    }
}
