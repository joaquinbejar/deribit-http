/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use crate::model::instrument::Instrument;
use crate::model::ticker::TickerData;
use crate::model::{BasicGreeks, BasicOptionData, OptionType, Spread};
use chrono::{DateTime, TimeZone, Utc};
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

/// Delivery price data
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct DeliveryPriceData {
    /// Date of the delivery price
    pub date: String,
    /// Delivery price value
    pub delivery_price: f64,
}

/// Greeks sub-structure for options
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Greeks {
    /// Delta value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    /// Gamma value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma: Option<f64>,
    /// Vega value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vega: Option<f64>,
    /// Theta value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// Rho value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rho: Option<f64>,
}

/// Combined option instrument data with ticker information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OptionInstrument {
    /// The instrument details
    pub instrument: Instrument,
    /// Real-time ticker data for the option
    pub ticker: TickerData,
}

/// A pair of option instruments representing both call and put options for the same underlying asset
///
/// This structure groups together the call and put options for a specific underlying asset,
/// allowing for easy access to both sides of an option strategy. Both options are optional,
/// meaning you can have just a call, just a put, or both.
///
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OptionInstrumentPair {
    /// Call option instrument data, if available
    pub call: Option<OptionInstrument>,
    /// Put option instrument data, if available  
    pub put: Option<OptionInstrument>,
}

impl OptionInstrumentPair {
    /// Returns the expiration date and time of the option instrument
    ///
    /// # Returns
    ///
    /// * `Some(DateTime<Utc>)` - The expiration timestamp if available
    /// * `None` - If no instrument is available or expiration timestamp is not set
    pub fn expiration(&self) -> Option<DateTime<Utc>> {
        let expiration_timestamp = match self.instrument() {
            Some(i) => i.expiration_timestamp,
            None => return None,
        };

        if let Some(expiration_timestamp) = expiration_timestamp {
            Utc.timestamp_millis_opt(expiration_timestamp).single()
        } else {
            None
        }
    }
    /// Returns the first available instrument from either the call or put option
    ///
    /// # Returns
    ///
    /// * `Some(Instrument)` - The instrument data from call option if available, otherwise from put option
    /// * `None` - If neither call nor put options are available
    pub fn instrument(&self) -> Option<Instrument> {
        self.call
            .as_ref()
            .map(|i| i.instrument.clone())
            .or_else(|| self.put.as_ref().map(|i| i.instrument.clone()))
    }
    /// Returns the first available ticker data from either the call or put option
    ///
    /// # Returns
    ///
    /// * `Some(TickerData)` - The ticker data from call option if available, otherwise from put option
    /// * `None` - If neither call nor put options are available
    pub fn ticker(&self) -> Option<TickerData> {
        self.call
            .as_ref()
            .map(|i| i.ticker.clone())
            .or_else(|| self.put.as_ref().map(|i| i.ticker.clone()))
    }
    /// Calculates the total trading volume across both call and put options
    ///
    /// # Returns
    ///
    /// The sum of volumes from both call and put options. Returns 0.0 if no options are available.
    pub fn volume(&self) -> f64 {
        let mut volume: f64 = 0.0;
        if let Some(call) = &self.call {
            volume += call.ticker.stats.volume
        }
        if let Some(put) = &self.put {
            volume += put.ticker.stats.volume
        }
        volume
    }
    /// Calculates the total open interest across both call and put options
    ///
    /// # Returns
    ///
    /// The sum of open interest from both call and put options. Returns 0.0 if no options are available.
    pub fn open_interest(&self) -> f64 {
        let mut open_interest: f64 = 0.0;
        if let Some(call) = &self.call {
            open_interest += call.ticker.open_interest.unwrap_or(0.0)
        }
        if let Some(put) = &self.put {
            open_interest += put.ticker.open_interest.unwrap_or(0.0)
        }
        open_interest
    }

    /// Calculates the total interest rate across both call and put options
    ///
    /// # Returns
    ///
    /// The sum of interest rates from both call and put options. Returns 0.0 if no options are available.
    pub fn interest_rate(&self) -> f64 {
        let mut interest_rate: f64 = 0.0;
        if let Some(call) = &self.call {
            interest_rate += call.ticker.interest_rate.unwrap_or(0.0)
        }
        if let Some(put) = &self.put {
            interest_rate += put.ticker.interest_rate.unwrap_or(0.0)
        }
        interest_rate
    }

    /// Serializes the option instrument pair to a JSON value
    ///
    /// # Returns
    ///
    /// * `Some(Value)` - The serialized JSON representation of this option pair
    /// * `None` - If serialization fails
    pub fn value(&self) -> Option<Value> {
        serde_json::to_value(self).ok()
    }

    /// Calculates the bid-ask spread for the call option
    ///
    /// # Returns
    ///
    /// A `Spread` struct containing bid, ask, and mid prices for the call option.
    /// Returns empty spread (all None values) if no call option is available.
    pub fn call_spread(&self) -> Spread {
        if let Some(call) = &self.call {
            let bid = call.ticker.best_bid_price;
            let ask = call.ticker.best_ask_price;
            let mid = match (bid, ask) {
                (Some(b), Some(a)) => Some((b + a) / 2.0),
                (Some(b), None) => Some(b),
                (None, Some(a)) => Some(a),
                (None, None) => None,
            };
            Spread { bid, ask, mid }
        } else {
            Spread {
                bid: None,
                ask: None,
                mid: None,
            }
        }
    }

    /// Calculates the bid-ask spread for the put option
    ///
    /// # Returns
    ///
    /// A `Spread` struct containing bid, ask, and mid prices for the put option.
    /// Returns empty spread (all None values) if no put option is available.
    pub fn put_spread(&self) -> Spread {
        if let Some(put) = &self.put {
            let bid = put.ticker.best_bid_price;
            let ask = put.ticker.best_ask_price;
            let mid = match (bid, ask) {
                (Some(b), Some(a)) => Some((b + a) / 2.0),
                (Some(b), None) => Some(b),
                (None, Some(a)) => Some(a),
                (None, None) => None,
            };
            Spread { bid, ask, mid }
        } else {
            Spread {
                bid: None,
                ask: None,
                mid: None,
            }
        }
    }

    /// Returns the implied volatility for both call and put options
    ///
    /// # Returns
    ///
    /// A tuple containing `(call_iv, put_iv)` where each element is the implied volatility
    /// for the respective option, or `None` if not available.
    pub fn iv(&self) -> (Option<f64>, Option<f64>) {
        let call_iv = self.call.as_ref().and_then(|c| c.ticker.mark_iv);
        let put_iv = self.put.as_ref().and_then(|p| p.ticker.mark_iv);
        (call_iv, put_iv)
    }

    /// Calculates and returns the basic Greeks for both call and put options
    ///
    /// # Returns
    ///
    /// A `BasicGreeks` struct containing delta values for call and put options,
    /// and gamma value (taken from either option if available).
    pub fn greeks(&self) -> BasicGreeks {
        let delta_call = self
            .call
            .as_ref()
            .and_then(|c| c.ticker.greeks.as_ref().and_then(|g| g.delta));
        let delta_put = self
            .put
            .as_ref()
            .and_then(|p| p.ticker.greeks.as_ref().and_then(|g| g.delta));
        let gamma = self
            .call
            .as_ref()
            .and_then(|c| c.ticker.greeks.as_ref().and_then(|g| g.gamma))
            .or_else(|| {
                self.put
                    .as_ref()
                    .and_then(|p| p.ticker.greeks.as_ref().and_then(|g| g.gamma))
            });
        BasicGreeks {
            delta_call,
            delta_put,
            gamma,
        }
    }

    /// Extracts and consolidates all relevant option data into a structured format
    ///
    /// # Returns
    ///
    /// A `BasicOptionData` struct containing comprehensive option information including
    /// strike price, bid/ask prices, implied volatility, Greeks, volume, open interest,
    /// expiration date, underlying price, risk-free rate, and additional fields.
    pub fn data(&self) -> BasicOptionData {
        let strike_price: f64 = match self.instrument() {
            Some(i) => i.strike.unwrap_or(0.0),
            None => 0.0,
        };
        let call_spread = self.call_spread();
        let call_bid: Option<f64> = call_spread.bid;
        let call_ask: Option<f64> = call_spread.ask;
        let put_spread = self.put_spread();
        let put_bid: Option<f64> = put_spread.bid;
        let put_ask: Option<f64> = put_spread.ask;
        let implied_volatility = self.iv();
        let greeks = self.greeks();
        let delta_call: Option<f64> = greeks.delta_call;
        let delta_put: Option<f64> = greeks.delta_put;
        let gamma: Option<f64> = greeks.gamma;
        let volume = self.volume();
        let open_interest: f64 = self.open_interest();
        let expiration_date: Option<DateTime<Utc>> = self.expiration();
        let underlying_price: Option<f64> = self.ticker().and_then(|t| t.underlying_price);
        let risk_free_rate: f64 = self.interest_rate();
        let extra_fields: Option<Value> = self.value();
        BasicOptionData {
            strike_price,
            call_bid,
            call_ask,
            put_bid,
            put_ask,
            implied_volatility,
            delta_call,
            delta_put,
            gamma,
            volume,
            open_interest,
            expiration_date,
            underlying_price,
            risk_free_rate,
            extra_fields,
        }
    }
}

/// Parsed option instrument with ticker data
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize)]
pub struct ParsedOptionWithTicker {
    /// The instrument name (e.g., "BTC-25DEC21-50000-C")
    pub instrument_name: String,
    /// Strike price of the option
    pub strike: f64,
    /// Type of option (Call or Put)
    pub option_type: OptionType,
    /// Expiry date string
    pub expiry: String,
    /// Associated ticker data
    pub ticker: TickerData,
}

/// Sort direction options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    /// Ascending sort order
    #[default]
    Asc,
    /// Descending sort order
    Desc,
    /// Default sort order (platform-specific)
    Default,
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortDirection::Asc => write!(f, "asc"),
            SortDirection::Desc => write!(f, "desc"),
            SortDirection::Default => write!(f, "default"),
        }
    }
}
