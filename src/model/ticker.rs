/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use crate::model::instrument::InstrumentKind;
use crate::model::other::Greeks;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Ticker stats sub-structure
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TickerStats {
    /// Trading volume
    pub volume: f64,
    /// Trading volume in USD
    pub volume_usd: Option<f64>,
    /// Price change from previous period
    pub price_change: Option<f64>,
    /// Highest price in the period
    pub high: Option<f64>,
    /// Lowest price in the period
    pub low: Option<f64>,
}

/// Ticker data structure with corrected field types
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TickerData {
    /// Name of the instrument
    pub instrument_name: String,
    /// Last traded price
    pub last_price: Option<f64>,
    /// Current mark price
    pub mark_price: f64,
    /// Best bid price available
    pub best_bid_price: Option<f64>,
    /// Best ask price available
    pub best_ask_price: Option<f64>,
    /// Amount available at best bid price
    pub best_bid_amount: f64,
    /// Amount available at best ask price
    pub best_ask_amount: f64,
    /// Trading volume in base currency
    pub volume: Option<f64>,
    /// Trading volume in USD
    pub volume_usd: Option<f64>,
    /// Open interest for the instrument
    pub open_interest: Option<f64>,
    /// Highest price in 24h period
    pub high: Option<f64>,
    /// Lowest price in 24h period
    pub low: Option<f64>,
    /// Absolute price change in 24h
    pub price_change: Option<f64>,
    /// Percentage price change in 24h
    pub price_change_percentage: Option<f64>,
    /// Implied volatility at best bid
    pub bid_iv: Option<f64>,
    /// Implied volatility at best ask
    pub ask_iv: Option<f64>,
    /// Mark implied volatility
    pub mark_iv: Option<f64>,
    /// Timestamp of the ticker data
    pub timestamp: u64,
    /// Current state of the instrument
    pub state: String,
    /// Settlement price (for expired instruments)
    pub settlement_price: Option<f64>,
    /// Additional ticker statistics
    pub stats: TickerStats,
    /// Greeks for options (delta, gamma, vega, theta, rho)
    pub greeks: Option<Greeks>,
    /// Index price
    pub index_price: Option<f64>,
    /// Minimum price
    pub min_price: Option<f64>,
    /// Maximum price
    pub max_price: Option<f64>,
    /// Interest rate
    pub interest_rate: Option<f64>,
    /// Underlying price
    pub underlying_price: Option<f64>,
    /// Underlying index
    pub underlying_index: Option<String>,
    /// Estimated delivery price
    pub estimated_delivery_price: Option<f64>,
}

/// Ticker information
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Ticker {
    /// Instrument name
    pub instrument_name: String,
    /// Timestamp of the ticker data
    pub timestamp: i64,
    /// Best bid price
    pub best_bid_price: Option<f64>,
    /// Best bid amount
    pub best_bid_amount: Option<f64>,
    /// Best ask price
    pub best_ask_price: Option<f64>,
    /// Best ask amount
    pub best_ask_amount: Option<f64>,
    /// Last trade price
    pub last_price: Option<f64>,
    /// Mark price
    pub mark_price: Option<f64>,
    /// Index price
    pub index_price: Option<f64>,
    /// Open interest
    pub open_interest: f64,
    /// 24h volume
    pub volume_24h: f64,
    /// 24h volume in USD
    pub volume_usd_24h: f64,
    /// 24h price change
    pub price_change_24h: f64,
    /// High price in 24h
    pub high_24h: Option<f64>,
    /// Low price in 24h
    pub low_24h: Option<f64>,
    /// Underlying price (for derivatives)
    pub underlying_price: Option<f64>,
    /// Underlying index
    pub underlying_index: Option<String>,
    /// Instrument kind
    pub instrument_kind: Option<InstrumentKind>,
    /// Current funding rate (for perpetuals)
    pub current_funding: Option<f64>,
    /// Funding 8h rate
    pub funding_8h: Option<f64>,
    /// Implied volatility (for options)
    pub iv: Option<f64>,
    /// Greeks (for options)
    pub greeks: Option<Greeks>,
    /// Interest rate
    pub interest_rate: Option<f64>,
}

impl Ticker {
    /// Calculate bid-ask spread
    pub fn spread(&self) -> Option<f64> {
        match (self.best_ask_price, self.best_bid_price) {
            (Some(ask), Some(bid)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Calculate mid price
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_ask_price, self.best_bid_price) {
            (Some(ask), Some(bid)) => Some((ask + bid) / 2.0),
            _ => None,
        }
    }

    /// Calculate spread percentage
    pub fn spread_percentage(&self) -> Option<f64> {
        match (self.spread(), self.mid_price()) {
            (Some(spread), Some(mid)) if mid != 0.0 => Some((spread / mid) * 100.0),
            _ => None,
        }
    }

    /// Check if there's a valid bid-ask spread
    pub fn has_valid_spread(&self) -> bool {
        self.best_bid_price.is_some() && self.best_ask_price.is_some()
    }
}
