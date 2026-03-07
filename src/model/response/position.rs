/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 7/3/26
******************************************************************************/
//! Position response models

use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Result of a single position move trade
///
/// Represents the outcome of moving a position between subaccounts.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovePositionResult {
    /// Instrument name (e.g., "BTC-PERPETUAL")
    pub instrument_name: String,
    /// Trade direction from source perspective ("buy" or "sell")
    pub direction: String,
    /// Price of the trade
    pub price: f64,
    /// Trade amount (USD for perpetual/inverse, base currency for options/linear)
    pub amount: f64,
    /// Source subaccount ID
    pub source_uid: i64,
    /// Target subaccount ID
    pub target_uid: i64,
}

impl MovePositionResult {
    /// Create a new move position result
    pub fn new(
        instrument_name: impl Into<String>,
        direction: impl Into<String>,
        price: f64,
        amount: f64,
        source_uid: i64,
        target_uid: i64,
    ) -> Self {
        Self {
            instrument_name: instrument_name.into(),
            direction: direction.into(),
            price,
            amount,
            source_uid,
            target_uid,
        }
    }

    /// Check if this is a buy trade
    pub fn is_buy(&self) -> bool {
        self.direction == "buy"
    }

    /// Check if this is a sell trade
    pub fn is_sell(&self) -> bool {
        self.direction == "sell"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_position_result_new() {
        let result = MovePositionResult::new("BTC-PERPETUAL", "buy", 35800.0, 110.0, 3, 23);
        assert_eq!(result.instrument_name, "BTC-PERPETUAL");
        assert_eq!(result.direction, "buy");
        assert_eq!(result.price, 35800.0);
        assert_eq!(result.amount, 110.0);
        assert_eq!(result.source_uid, 3);
        assert_eq!(result.target_uid, 23);
    }

    #[test]
    fn test_move_position_result_is_buy() {
        let result = MovePositionResult::new("BTC-PERPETUAL", "buy", 35800.0, 110.0, 3, 23);
        assert!(result.is_buy());
        assert!(!result.is_sell());
    }

    #[test]
    fn test_move_position_result_is_sell() {
        let result = MovePositionResult::new("BTC-PERPETUAL", "sell", 35800.0, 110.0, 3, 23);
        assert!(result.is_sell());
        assert!(!result.is_buy());
    }

    #[test]
    fn test_move_position_result_deserialization() {
        let json = r#"{
            "instrument_name": "BTC-PERPETUAL",
            "direction": "buy",
            "price": 35800.0,
            "amount": 110.0,
            "source_uid": 3,
            "target_uid": 23
        }"#;

        let result: MovePositionResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.instrument_name, "BTC-PERPETUAL");
        assert_eq!(result.direction, "buy");
        assert_eq!(result.price, 35800.0);
        assert_eq!(result.amount, 110.0);
        assert_eq!(result.source_uid, 3);
        assert_eq!(result.target_uid, 23);
    }
}
