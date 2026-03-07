/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 7/3/26
******************************************************************************/
//! Combo books models for Deribit API
//!
//! This module provides types for combo instrument operations including
//! creating combos and calculating leg prices.

use serde::{Deserialize, Serialize};

/// Combo state enumeration
///
/// Represents the current state of a combo instrument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ComboState {
    /// Request for quote state
    #[default]
    Rfq,
    /// Active combo available for trading
    Active,
    /// Inactive combo not available for trading
    Inactive,
}

impl std::fmt::Display for ComboState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComboState::Rfq => write!(f, "rfq"),
            ComboState::Active => write!(f, "active"),
            ComboState::Inactive => write!(f, "inactive"),
        }
    }
}

/// A leg within a combo instrument
///
/// Represents a single leg of a combo, consisting of an instrument
/// and an amount multiplier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComboLeg {
    /// Unique instrument identifier for this leg
    pub instrument_name: String,
    /// Size multiplier of the leg. A negative value indicates that trades
    /// on this leg are in the opposite direction to the combo trades.
    pub amount: i64,
}

impl ComboLeg {
    /// Creates a new combo leg
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier
    /// * `amount` - The size multiplier (can be negative for opposite direction)
    #[must_use]
    pub fn new(instrument_name: impl Into<String>, amount: i64) -> Self {
        Self {
            instrument_name: instrument_name.into(),
            amount,
        }
    }

    /// Returns true if this leg represents the opposite direction
    #[must_use]
    pub fn is_opposite_direction(&self) -> bool {
        self.amount < 0
    }
}

/// Combo instrument information
///
/// Contains full details about a combo instrument including its legs,
/// state, and timestamps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Combo {
    /// Unique combo identifier (e.g., "BTC-FS-29APR22_PERP")
    pub id: String,
    /// Numeric instrument ID
    pub instrument_id: u64,
    /// Current state of the combo
    pub state: ComboState,
    /// Timestamp of the last state change in milliseconds since Unix epoch
    pub state_timestamp: u64,
    /// Timestamp when the combo was created in milliseconds since Unix epoch
    pub creation_timestamp: u64,
    /// List of legs that make up this combo
    pub legs: Vec<ComboLeg>,
}

impl Combo {
    /// Returns true if the combo is currently active for trading
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.state == ComboState::Active
    }

    /// Returns true if the combo is in RFQ state
    #[must_use]
    pub fn is_rfq(&self) -> bool {
        self.state == ComboState::Rfq
    }

    /// Returns the number of legs in this combo
    #[must_use]
    pub fn leg_count(&self) -> usize {
        self.legs.len()
    }
}

/// Trade input for creating a combo
///
/// Used as input to the `create_combo` endpoint to specify
/// the instruments and directions for each leg.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComboTrade {
    /// Instrument name for this trade
    pub instrument_name: String,
    /// Trade amount (optional). For perpetual and inverse futures the amount
    /// is in USD units. For options and linear futures it is the underlying
    /// base currency coin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Direction of trade from the maker perspective: "buy" or "sell"
    pub direction: String,
}

impl ComboTrade {
    /// Creates a new combo trade
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier
    /// * `direction` - Trade direction ("buy" or "sell")
    /// * `amount` - Optional trade amount
    #[must_use]
    pub fn new(
        instrument_name: impl Into<String>,
        direction: impl Into<String>,
        amount: Option<f64>,
    ) -> Self {
        Self {
            instrument_name: instrument_name.into(),
            direction: direction.into(),
            amount,
        }
    }

    /// Creates a buy trade
    #[must_use]
    pub fn buy(instrument_name: impl Into<String>, amount: Option<f64>) -> Self {
        Self::new(instrument_name, "buy", amount)
    }

    /// Creates a sell trade
    #[must_use]
    pub fn sell(instrument_name: impl Into<String>, amount: Option<f64>) -> Self {
        Self::new(instrument_name, "sell", amount)
    }
}

/// Leg input for `get_leg_prices` endpoint
///
/// Specifies the parameters for calculating individual leg prices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegInput {
    /// Instrument name for this leg
    pub instrument_name: String,
    /// Trade amount. For perpetual and inverse futures the amount is in USD
    /// units. For options and linear futures it is the underlying base
    /// currency coin.
    pub amount: f64,
    /// Direction of the leg: "buy" or "sell"
    pub direction: String,
}

impl LegInput {
    /// Creates a new leg input
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier
    /// * `amount` - Trade amount
    /// * `direction` - Trade direction ("buy" or "sell")
    #[must_use]
    pub fn new(
        instrument_name: impl Into<String>,
        amount: f64,
        direction: impl Into<String>,
    ) -> Self {
        Self {
            instrument_name: instrument_name.into(),
            amount,
            direction: direction.into(),
        }
    }

    /// Creates a buy leg input
    #[must_use]
    pub fn buy(instrument_name: impl Into<String>, amount: f64) -> Self {
        Self::new(instrument_name, amount, "buy")
    }

    /// Creates a sell leg input
    #[must_use]
    pub fn sell(instrument_name: impl Into<String>, amount: f64) -> Self {
        Self::new(instrument_name, amount, "sell")
    }
}

/// Individual leg price in response
///
/// Contains the calculated price and ratio for a single leg.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegPrice {
    /// Instrument name for this leg
    pub instrument_name: String,
    /// Direction: "buy" or "sell"
    pub direction: String,
    /// Calculated price for this leg
    pub price: f64,
    /// Ratio of amount between legs
    pub ratio: i64,
}

/// Response from `get_leg_prices` endpoint
///
/// Contains the calculated leg prices for a combo structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegPricesResponse {
    /// This value multiplied by the ratio of a leg gives trade size on that leg
    pub amount: f64,
    /// List of leg prices
    pub legs: Vec<LegPrice>,
}

impl LegPricesResponse {
    /// Returns the number of legs in the response
    #[must_use]
    pub fn leg_count(&self) -> usize {
        self.legs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combo_state_serialization() {
        assert_eq!(
            serde_json::to_string(&ComboState::Active).unwrap(),
            "\"active\""
        );
        assert_eq!(serde_json::to_string(&ComboState::Rfq).unwrap(), "\"rfq\"");
        assert_eq!(
            serde_json::to_string(&ComboState::Inactive).unwrap(),
            "\"inactive\""
        );
    }

    #[test]
    fn test_combo_state_deserialization() {
        assert_eq!(
            serde_json::from_str::<ComboState>("\"active\"").unwrap(),
            ComboState::Active
        );
        assert_eq!(
            serde_json::from_str::<ComboState>("\"rfq\"").unwrap(),
            ComboState::Rfq
        );
        assert_eq!(
            serde_json::from_str::<ComboState>("\"inactive\"").unwrap(),
            ComboState::Inactive
        );
    }

    #[test]
    fn test_combo_leg_new() {
        let leg = ComboLeg::new("BTC-PERPETUAL", -1);
        assert_eq!(leg.instrument_name, "BTC-PERPETUAL");
        assert_eq!(leg.amount, -1);
        assert!(leg.is_opposite_direction());
    }

    #[test]
    fn test_combo_leg_positive_amount() {
        let leg = ComboLeg::new("BTC-29APR22", 1);
        assert!(!leg.is_opposite_direction());
    }

    #[test]
    fn test_combo_trade_buy() {
        let trade = ComboTrade::buy("BTC-29APR22-37500-C", Some(1.0));
        assert_eq!(trade.instrument_name, "BTC-29APR22-37500-C");
        assert_eq!(trade.direction, "buy");
        assert_eq!(trade.amount, Some(1.0));
    }

    #[test]
    fn test_combo_trade_sell() {
        let trade = ComboTrade::sell("BTC-29APR22-37500-P", None);
        assert_eq!(trade.direction, "sell");
        assert!(trade.amount.is_none());
    }

    #[test]
    fn test_leg_input_new() {
        let leg = LegInput::new("BTC-1NOV24-67000-C", 2.0, "buy");
        assert_eq!(leg.instrument_name, "BTC-1NOV24-67000-C");
        assert_eq!(leg.amount, 2.0);
        assert_eq!(leg.direction, "buy");
    }

    #[test]
    fn test_combo_deserialization() {
        let json = r#"{
            "state_timestamp": 1650960943922,
            "state": "rfq",
            "legs": [
                {"instrument_name": "BTC-29APR22-37500-C", "amount": 1},
                {"instrument_name": "BTC-29APR22-37500-P", "amount": -1}
            ],
            "id": "BTC-REV-29APR22-37500",
            "instrument_id": 52,
            "creation_timestamp": 1650960943000
        }"#;

        let combo: Combo = serde_json::from_str(json).unwrap();
        assert_eq!(combo.id, "BTC-REV-29APR22-37500");
        assert_eq!(combo.instrument_id, 52);
        assert_eq!(combo.state, ComboState::Rfq);
        assert!(combo.is_rfq());
        assert!(!combo.is_active());
        assert_eq!(combo.leg_count(), 2);
        assert_eq!(combo.legs[0].instrument_name, "BTC-29APR22-37500-C");
        assert_eq!(combo.legs[0].amount, 1);
        assert_eq!(combo.legs[1].amount, -1);
    }

    #[test]
    fn test_leg_prices_response_deserialization() {
        let json = r#"{
            "legs": [
                {"ratio": 1, "instrument_name": "BTC-1NOV24-67000-C", "price": 0.6001, "direction": "buy"},
                {"ratio": 1, "instrument_name": "BTC-1NOV24-66000-C", "price": 0.0001, "direction": "sell"}
            ],
            "amount": 2
        }"#;

        let response: LegPricesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.amount, 2.0);
        assert_eq!(response.leg_count(), 2);
        assert_eq!(response.legs[0].price, 0.6001);
        assert_eq!(response.legs[1].direction, "sell");
    }

    #[test]
    fn test_combo_trade_serialization() {
        let trade = ComboTrade::new("BTC-29APR22-37500-C", "buy", Some(1.0));
        let json = serde_json::to_string(&trade).unwrap();
        assert!(json.contains("\"instrument_name\":\"BTC-29APR22-37500-C\""));
        assert!(json.contains("\"direction\":\"buy\""));
        assert!(json.contains("\"amount\":1.0"));
    }

    #[test]
    fn test_combo_trade_without_amount() {
        let trade = ComboTrade::new("BTC-29APR22-37500-C", "buy", None);
        let json = serde_json::to_string(&trade).unwrap();
        assert!(!json.contains("amount"));
    }
}
