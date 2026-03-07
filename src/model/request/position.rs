/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 7/3/26
******************************************************************************/
//! Position request models

use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A single trade specification for moving positions
///
/// Represents a position trade to be moved between subaccounts.
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovePositionTrade {
    /// Instrument name (e.g., "BTC-PERPETUAL")
    pub instrument_name: String,
    /// Trade amount (USD for perpetual/inverse, base currency for options/linear)
    pub amount: f64,
    /// Price for trade (optional, defaults to average position price)
    pub price: Option<f64>,
}

impl MovePositionTrade {
    /// Create a new move position trade
    pub fn new(instrument_name: impl Into<String>, amount: f64) -> Self {
        Self {
            instrument_name: instrument_name.into(),
            amount,
            price: None,
        }
    }

    /// Create a new move position trade with a specific price
    pub fn with_price(instrument_name: impl Into<String>, amount: f64, price: f64) -> Self {
        Self {
            instrument_name: instrument_name.into(),
            amount,
            price: Some(price),
        }
    }

    /// Set the price for this trade
    #[must_use]
    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }
}

/// Request to move positions between subaccounts
///
/// Contains all parameters needed for the move_positions API call.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovePositionsRequest {
    /// Currency symbol (e.g., "BTC", "ETH", "USDC")
    pub currency: String,
    /// Source subaccount ID
    pub source_uid: i64,
    /// Target subaccount ID
    pub target_uid: i64,
    /// List of trades for position move
    pub trades: Vec<MovePositionTrade>,
}

impl MovePositionsRequest {
    /// Create a new move positions request
    pub fn new(
        currency: impl Into<String>,
        source_uid: i64,
        target_uid: i64,
        trades: Vec<MovePositionTrade>,
    ) -> Self {
        Self {
            currency: currency.into(),
            source_uid,
            target_uid,
            trades,
        }
    }

    /// Add a trade to the request
    pub fn add_trade(&mut self, trade: MovePositionTrade) {
        self.trades.push(trade);
    }

    /// Get the number of trades
    pub fn trade_count(&self) -> usize {
        self.trades.len()
    }

    /// Check if the request has any trades
    pub fn has_trades(&self) -> bool {
        !self.trades.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_position_trade_new() {
        let trade = MovePositionTrade::new("BTC-PERPETUAL", 100.0);
        assert_eq!(trade.instrument_name, "BTC-PERPETUAL");
        assert_eq!(trade.amount, 100.0);
        assert!(trade.price.is_none());
    }

    #[test]
    fn test_move_position_trade_with_price() {
        let trade = MovePositionTrade::with_price("BTC-PERPETUAL", 100.0, 35800.0);
        assert_eq!(trade.instrument_name, "BTC-PERPETUAL");
        assert_eq!(trade.amount, 100.0);
        assert_eq!(trade.price, Some(35800.0));
    }

    #[test]
    fn test_move_positions_request_new() {
        let trades = vec![MovePositionTrade::new("BTC-PERPETUAL", 100.0)];
        let request = MovePositionsRequest::new("BTC", 3, 23, trades);
        assert_eq!(request.currency, "BTC");
        assert_eq!(request.source_uid, 3);
        assert_eq!(request.target_uid, 23);
        assert_eq!(request.trade_count(), 1);
    }

    #[test]
    fn test_move_positions_request_serialization() {
        let trades = vec![
            MovePositionTrade::with_price("BTC-PERPETUAL", 110.0, 35800.0),
            MovePositionTrade::new("BTC-28JAN22-32500-C", 0.1),
        ];
        let request = MovePositionsRequest::new("BTC", 3, 23, trades);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("BTC-PERPETUAL"));
        assert!(json.contains("source_uid"));
        assert!(json.contains("target_uid"));
    }
}
