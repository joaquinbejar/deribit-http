/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::Display;

/// Instrument kind enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstrumentKind {
    /// Future contract
    Future,
    /// Option contract
    Option,
    /// Spot trading
    Spot,
    /// Future combo
    #[serde(rename = "future_combo")]
    FutureCombo,
    /// Option combo
    #[serde(rename = "option_combo")]
    OptionCombo,
}

impl Display for InstrumentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstrumentKind::Future => write!(f, "future"),
            InstrumentKind::Option => write!(f, "option"),
            InstrumentKind::Spot => write!(f, "spot"),
            InstrumentKind::FutureCombo => write!(f, "future_combo"),
            InstrumentKind::OptionCombo => write!(f, "option_combo"),
        }
    }
}

/// Instrument type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstrumentType {
    /// Linear instrument
    Linear,
    /// Reversed instrument
    Reversed,
}

/// Instrument information
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Instrument {
    /// Instrument name (e.g., "BTC-PERPETUAL", "ETH-25JUL25-3000-C")
    pub instrument_name: String,
    /// Price index used for mark price calculation
    pub price_index: Option<String>,
    /// Instrument kind
    pub kind: Option<InstrumentKind>,
    /// Base currency
    pub currency: Option<String>,
    /// Whether the instrument is active for trading
    pub is_active: Option<bool>,
    /// Expiration timestamp (None for perpetuals)
    pub expiration_timestamp: Option<i64>,
    /// Strike price (for options)
    pub strike: Option<f64>,
    /// Option type (call/put, for options only)
    pub option_type: Option<OptionType>,
    /// Minimum price movement
    pub tick_size: Option<f64>,
    /// Minimum trade amount
    pub min_trade_amount: Option<f64>,
    /// Contract size
    pub contract_size: Option<f64>,
    /// Settlement period
    pub settlement_period: Option<String>,
    /// Instrument type (linear/reversed)
    pub instrument_type: Option<InstrumentType>,
    /// Quote currency
    pub quote_currency: Option<String>,
    /// Settlement currency
    pub settlement_currency: Option<String>,
    /// Creation timestamp
    pub creation_timestamp: Option<i64>,
    /// Maximum leverage
    pub max_leverage: Option<f64>,
    /// Maker commission rate
    pub maker_commission: Option<f64>,
    /// Taker commission rate
    pub taker_commission: Option<f64>,
    /// Unique instrument identifier
    pub instrument_id: Option<u32>,
    /// Base currency for the instrument
    pub base_currency: Option<String>,
    /// Counter currency for the instrument
    pub counter_currency: Option<String>,
}

impl Instrument {
    /// Check if the instrument is a perpetual contract
    pub fn is_perpetual(&self) -> bool {
        self.expiration_timestamp.is_none()
            && self
                .kind
                .as_ref()
                .is_some_and(|k| matches!(k, InstrumentKind::Future))
    }

    /// Check if the instrument is an option
    pub fn is_option(&self) -> bool {
        self.kind
            .as_ref()
            .is_some_and(|k| matches!(k, InstrumentKind::Option | InstrumentKind::OptionCombo))
    }

    /// Check if the instrument is a future
    pub fn is_future(&self) -> bool {
        self.kind
            .as_ref()
            .is_some_and(|k| matches!(k, InstrumentKind::Future | InstrumentKind::FutureCombo))
    }

    /// Check if the instrument is a spot
    pub fn is_spot(&self) -> bool {
        self.kind
            .as_ref()
            .is_some_and(|k| matches!(k, InstrumentKind::Spot))
    }
}

/// Option type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionType {
    /// Call option
    Call,
    /// Put option
    Put,
}
