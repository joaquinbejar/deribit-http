//! Trading products models for Deribit API
//!
//! This module contains types for trading product configuration.

use serde::{Deserialize, Serialize};

/// Trading product types that can be enabled/disabled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradingProduct {
    /// Futures contracts
    Futures,
    /// Options contracts
    Options,
    /// Spot trading
    Spots,
    /// Future combo instruments
    FutureCombos,
    /// Option combo instruments
    OptionCombos,
}

impl TradingProduct {
    /// Returns the product as a string for API requests
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Futures => "futures",
            Self::Options => "options",
            Self::Spots => "spots",
            Self::FutureCombos => "future_combos",
            Self::OptionCombos => "option_combos",
        }
    }
}

impl std::fmt::Display for TradingProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trading_product_serialization() {
        let product = TradingProduct::FutureCombos;
        let json = serde_json::to_string(&product).expect("Failed to serialize");
        assert_eq!(json, "\"future_combos\"");
    }

    #[test]
    fn test_trading_product_deserialization() {
        let json = "\"option_combos\"";
        let product: TradingProduct = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(product, TradingProduct::OptionCombos);
    }

    #[test]
    fn test_trading_product_as_str() {
        assert_eq!(TradingProduct::Spots.as_str(), "spots");
        assert_eq!(TradingProduct::Options.as_str(), "options");
    }
}
