/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 20/9/25
******************************************************************************/
use crate::HttpError;
use crate::prelude::OptionType;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

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
