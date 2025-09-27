/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 20/9/25
******************************************************************************/
use crate::HttpError;
use crate::prelude::OptionType;
use chrono::{DateTime, Utc};
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Option contract information structure
///
/// Contains all the essential information about an option contract including
/// the underlying symbol, option type, strike price, and expiration date.
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionInfo {
    /// The underlying asset symbol (e.g., "BTC", "ETH")
    pub symbol: String,
    /// The type of option (Call or Put)
    pub option_type: OptionType,
    /// The strike price of the option contract
    pub strike_price: f64,
    /// The expiration date in DDMMMYY format (e.g., "28NOV25")
    pub expiration_date: String,
}

impl OptionInfo {
    /// Parses an option string in the format: "SYMBOL-DDMMMYY-STRIKE-TYPE"
    /// Example: "BTC-28NOV25-108000-P"
    pub fn parse_from_string(option_string: &str) -> Result<Self, HttpError> {
        let parts: Vec<&str> = option_string.split('-').collect();

        // Validate we have exactly 4 parts
        if parts.len() != 4 {
            return Err(HttpError::ParseError("InvalidFormat".to_string()));
        }

        let symbol = parts[0].to_string();
        let expiration_date = parts[1].to_string();

        // Parse strike price
        let strike_price = parts[2]
            .parse::<f64>()
            .map_err(|_| HttpError::ParseError("InvalidStrikePrice".to_string()))?;

        // Parse option type
        let option_type = match parts[3].to_uppercase().as_str() {
            "C" => OptionType::Call,
            "P" => OptionType::Put,
            _ => return Err(HttpError::ParseError("InvalidOptionType".to_string())),
        };

        // Basic validation for expiration date format (DDMMMYY)
        if expiration_date.len() != 7 {
            return Err(HttpError::ParseError("InvalidExpirationDate".to_string()));
        };

        Ok(OptionInfo {
            symbol,
            option_type,
            strike_price,
            expiration_date,
        })
    }
}

/// Spread information for bid/ask prices
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Spread {
    /// Best bid price
    pub bid: Option<f64>,
    /// Best ask price
    pub ask: Option<f64>,
    /// Mid price (average of bid and ask)
    pub mid: Option<f64>,
}

/// Basic Greeks values for option pricing
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct BasicGreeks {
    /// Delta value for call option
    pub delta_call: Option<f64>,
    /// Delta value for put option
    pub delta_put: Option<f64>,
    /// Gamma value (rate of change of delta)
    pub gamma: Option<f64>,
}

/// Comprehensive option data structure containing all relevant pricing and risk information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct BasicOptionData {
    /// Strike price of the option
    pub strike_price: f64,
    /// Best bid price for call option
    pub call_bid: Option<f64>,
    /// Best ask price for call option
    pub call_ask: Option<f64>,
    /// Best bid price for put option
    pub put_bid: Option<f64>,
    /// Best ask price for put option
    pub put_ask: Option<f64>,
    /// Implied volatility for call and put options (call_iv, put_iv)
    pub implied_volatility: (Option<f64>, Option<f64>),
    /// Delta value for call option
    pub delta_call: Option<f64>,
    /// Delta value for put option
    pub delta_put: Option<f64>,
    /// Gamma value (rate of change of delta)
    pub gamma: Option<f64>,
    /// Total trading volume
    pub volume: f64,
    /// Total open interest
    pub open_interest: f64,
    /// Option expiration date
    pub expiration_date: Option<DateTime<Utc>>,
    /// Current price of the underlying asset
    pub underlying_price: Option<f64>,
    /// Risk-free interest rate
    pub risk_free_rate: f64,
    /// Additional fields as JSON value
    pub extra_fields: Option<Value>,
}
