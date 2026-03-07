/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 7/3/26
******************************************************************************/
//! Subaccount response models

use crate::model::position::Position;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Subaccount details with positions
///
/// Contains position details for a specific subaccount, including
/// all open positions and optionally open orders.
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct SubaccountDetails {
    /// Subaccount ID
    pub uid: i64,
    /// List of positions for this subaccount
    pub positions: Vec<Position>,
    /// Open orders (optional, when with_open_orders=true)
    pub open_orders: Option<Vec<serde_json::Value>>,
}

impl SubaccountDetails {
    /// Create a new subaccount details instance
    pub fn new(uid: i64, positions: Vec<Position>) -> Self {
        Self {
            uid,
            positions,
            open_orders: None,
        }
    }

    /// Create a new subaccount details instance with open orders
    pub fn with_open_orders(
        uid: i64,
        positions: Vec<Position>,
        open_orders: Vec<serde_json::Value>,
    ) -> Self {
        Self {
            uid,
            positions,
            open_orders: Some(open_orders),
        }
    }

    /// Check if this subaccount has any positions
    pub fn has_positions(&self) -> bool {
        !self.positions.is_empty()
    }

    /// Get the number of positions
    pub fn position_count(&self) -> usize {
        self.positions.len()
    }

    /// Check if this subaccount has open orders
    pub fn has_open_orders(&self) -> bool {
        self.open_orders
            .as_ref()
            .map(|orders: &Vec<serde_json::Value>| !orders.is_empty())
            .unwrap_or(false)
    }

    /// Get the number of open orders
    pub fn open_orders_count(&self) -> usize {
        self.open_orders
            .as_ref()
            .map(|orders: &Vec<serde_json::Value>| orders.len())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::types::Direction;

    fn create_test_position() -> Position {
        Position {
            average_price: 49571.3,
            average_price_usd: None,
            delta: Some(0.004152776),
            direction: Direction::Buy,
            estimated_liquidation_price: Some(2.33),
            floating_profit_loss: Some(-0.00003451),
            floating_profit_loss_usd: None,
            gamma: None,
            index_price: Some(47897.12),
            initial_margin: Some(0.000122508),
            instrument_name: "BTC-PERPETUAL".to_string(),
            interest_value: None,
            kind: Some("future".to_string()),
            leverage: Some(34),
            maintenance_margin: Some(0.000089286),
            mark_price: Some(48160.55),
            open_orders_margin: Some(0.0),
            realized_funding: Some(-8.8e-7),
            realized_profit_loss: Some(-8.79e-7),
            settlement_price: Some(48150.36),
            size: 200.0,
            size_currency: Some(0.004152776),
            theta: None,
            total_profit_loss: Some(-0.000118183),
            vega: None,
            unrealized_profit_loss: None,
        }
    }

    #[test]
    fn test_subaccount_details_new() {
        let positions = vec![create_test_position()];
        let details = SubaccountDetails::new(3, positions);

        assert_eq!(details.uid, 3);
        assert_eq!(details.position_count(), 1);
        assert!(details.has_positions());
        assert!(!details.has_open_orders());
        assert_eq!(details.open_orders_count(), 0);
    }

    #[test]
    fn test_subaccount_details_empty() {
        let details = SubaccountDetails::new(10, vec![]);

        assert_eq!(details.uid, 10);
        assert_eq!(details.position_count(), 0);
        assert!(!details.has_positions());
    }

    #[test]
    fn test_subaccount_details_deserialization() {
        let json = r#"{
            "uid": 3,
            "positions": [
                {
                    "total_profit_loss": -0.000118183,
                    "size_currency": 0.004152776,
                    "size": 200,
                    "settlement_price": 48150.36,
                    "realized_profit_loss": -8.79e-7,
                    "realized_funding": -8.8e-7,
                    "open_orders_margin": 0,
                    "mark_price": 48160.55,
                    "maintenance_margin": 0.000089286,
                    "leverage": 34,
                    "kind": "future",
                    "instrument_name": "BTC-PERPETUAL",
                    "initial_margin": 0.000122508,
                    "index_price": 47897.12,
                    "floating_profit_loss": -0.00003451,
                    "estimated_liquidation_price": 2.33,
                    "direction": "buy",
                    "delta": 0.004152776,
                    "average_price": 49571.3
                }
            ]
        }"#;

        let details: SubaccountDetails = serde_json::from_str(json).unwrap();
        assert_eq!(details.uid, 3);
        assert_eq!(details.position_count(), 1);
        assert_eq!(details.positions[0].instrument_name, "BTC-PERPETUAL");
    }
}
