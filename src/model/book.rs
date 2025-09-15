/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Book summary information for an instrument
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct BookSummary {
    /// Instrument name
    pub instrument_name: String,
    /// Base currency
    pub base_currency: String,
    /// Quote currency (usually USD)
    pub quote_currency: String,
    /// 24h trading volume
    pub volume: f64,
    /// 24h trading volume in USD
    pub volume_usd: f64,
    /// Open interest
    pub open_interest: f64,
    /// 24h price change percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_change: Option<f64>,
    /// Current mark price
    pub mark_price: f64,
    /// Mark implied volatility (options only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_iv: Option<f64>,
    /// Best bid price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    /// Best ask price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    /// Mid price (bid + ask) / 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid_price: Option<f64>,
    /// Last trade price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<f64>,
    /// 24h high price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<f64>,
    /// 24h low price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<f64>,
    /// Estimated delivery price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_price: Option<f64>,
    /// Current funding rate (perpetuals only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_funding: Option<f64>,
    /// 8h funding rate (perpetuals only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_8h: Option<f64>,
    /// Creation timestamp (milliseconds since Unix epoch)
    pub creation_timestamp: i64,
    // Additional optional fields merged from deribit-http types.rs
    /// Underlying index name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_index: Option<String>,
    /// Underlying price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_price: Option<f64>,
    /// Interest rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_rate: Option<f64>,
}

impl BookSummary {
    /// Create a new book summary
    pub fn new(
        instrument_name: String,
        base_currency: String,
        quote_currency: String,
        mark_price: f64,
        creation_timestamp: i64,
    ) -> Self {
        Self {
            instrument_name,
            base_currency,
            quote_currency,
            volume: 0.0,
            volume_usd: 0.0,
            open_interest: 0.0,
            price_change: None,
            mark_price,
            mark_iv: None,
            bid_price: None,
            ask_price: None,
            mid_price: None,
            last: None,
            high: None,
            low: None,
            estimated_delivery_price: None,
            current_funding: None,
            funding_8h: None,
            creation_timestamp,
            // initialize merged optional fields
            underlying_index: None,
            underlying_price: None,
            interest_rate: None,
        }
    }

    /// Set volume information
    pub fn with_volume(mut self, volume: f64, volume_usd: f64) -> Self {
        self.volume = volume;
        self.volume_usd = volume_usd;
        self
    }

    /// Set price information
    pub fn with_prices(
        mut self,
        bid: Option<f64>,
        ask: Option<f64>,
        last: Option<f64>,
        high: Option<f64>,
        low: Option<f64>,
    ) -> Self {
        self.bid_price = bid;
        self.ask_price = ask;
        self.last = last;
        self.high = high;
        self.low = low;

        // Calculate mid price if both bid and ask are available
        if let (Some(bid), Some(ask)) = (bid, ask) {
            self.mid_price = Some((bid + ask) / 2.0);
        }

        self
    }

    /// Set open interest
    pub fn with_open_interest(mut self, open_interest: f64) -> Self {
        self.open_interest = open_interest;
        self
    }

    /// Set price change percentage
    pub fn with_price_change(mut self, price_change: f64) -> Self {
        self.price_change = Some(price_change);
        self
    }

    /// Set implied volatility (for options)
    pub fn with_iv(mut self, mark_iv: f64) -> Self {
        self.mark_iv = Some(mark_iv);
        self
    }

    /// Set funding rates (for perpetuals)
    pub fn with_funding(mut self, current: f64, funding_8h: f64) -> Self {
        self.current_funding = Some(current);
        self.funding_8h = Some(funding_8h);
        self
    }

    /// Set estimated delivery price
    pub fn with_delivery_price(mut self, price: f64) -> Self {
        self.estimated_delivery_price = Some(price);
        self
    }

    /// Get spread (ask - bid)
    pub fn spread(&self) -> Option<f64> {
        match (self.bid_price, self.ask_price) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Get spread percentage
    pub fn spread_percentage(&self) -> Option<f64> {
        match (self.spread(), self.mid_price) {
            (Some(spread), Some(mid)) if mid > 0.0 => Some((spread / mid) * 100.0),
            _ => None,
        }
    }

    /// Check if this is a perpetual contract
    pub fn is_perpetual(&self) -> bool {
        self.instrument_name.contains("PERPETUAL")
    }

    /// Check if this is an option
    pub fn is_option(&self) -> bool {
        // Options end with -C or -P (call/put) but not PERPETUAL
        !self.is_perpetual()
            && (self.instrument_name.ends_with("-C") || self.instrument_name.ends_with("-P"))
    }

    /// Check if this is a future
    pub fn is_future(&self) -> bool {
        !self.is_perpetual() && !self.is_option()
    }

    /// Get 24h price change in absolute terms
    pub fn price_change_absolute(&self) -> Option<f64> {
        self.price_change.map(|change| {
            if let Some(last) = self.last {
                last * (change / 100.0)
            } else {
                self.mark_price * (change / 100.0)
            }
        })
    }
}

/// Collection of book summaries
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct BookSummaries {
    /// List of book summaries
    pub summaries: Vec<BookSummary>,
}

impl BookSummaries {
    /// Create a new collection
    pub fn new() -> Self {
        Self {
            summaries: Vec::new(),
        }
    }

    /// Add a book summary
    pub fn add(&mut self, summary: BookSummary) {
        self.summaries.push(summary);
    }

    /// Get summaries by currency
    pub fn by_currency(&self, currency: String) -> Vec<&BookSummary> {
        self.summaries
            .iter()
            .filter(|s| s.base_currency == currency)
            .collect()
    }

    /// Get summaries by instrument type
    pub fn perpetuals(&self) -> Vec<&BookSummary> {
        self.summaries.iter().filter(|s| s.is_perpetual()).collect()
    }

    /// Get option summaries
    pub fn options(&self) -> Vec<&BookSummary> {
        self.summaries.iter().filter(|s| s.is_option()).collect()
    }

    /// Get future summaries
    pub fn futures(&self) -> Vec<&BookSummary> {
        self.summaries.iter().filter(|s| s.is_future()).collect()
    }

    /// Sort by volume (descending)
    pub fn sort_by_volume(&mut self) {
        self.summaries
            .sort_by(|a, b| b.volume_usd.partial_cmp(&a.volume_usd).unwrap());
    }

    /// Sort by open interest (descending)
    pub fn sort_by_open_interest(&mut self) {
        self.summaries
            .sort_by(|a, b| b.open_interest.partial_cmp(&a.open_interest).unwrap());
    }
}

impl Default for BookSummaries {
    fn default() -> Self {
        Self::new()
    }
}

/// Order book entry
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OrderBookEntry {
    /// Price level
    pub price: f64,
    /// Amount at this price level
    pub amount: f64,
}

impl OrderBookEntry {
    /// Create a new order book entry
    pub fn new(price: f64, amount: f64) -> Self {
        Self { price, amount }
    }

    /// Calculate notional value
    pub fn notional(&self) -> f64 {
        self.price * self.amount
    }
}

/// Order book data
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Instrument name
    pub instrument_name: String,
    /// Timestamp of the order book
    pub timestamp: i64,
    /// Bid levels (sorted by price descending)
    pub bids: Vec<OrderBookEntry>,
    /// Ask levels (sorted by price ascending)
    pub asks: Vec<OrderBookEntry>,
    /// Change ID for incremental updates
    pub change_id: u64,
    /// Previous change ID
    pub prev_change_id: Option<u64>,
}

impl OrderBook {
    /// Create a new empty order book
    pub fn new(instrument_name: String, timestamp: i64, change_id: u64) -> Self {
        Self {
            instrument_name,
            timestamp,
            bids: Vec::new(),
            asks: Vec::new(),
            change_id,
            prev_change_id: None,
        }
    }

    /// Get best bid price
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.first().map(|entry| entry.price)
    }

    /// Get best ask price
    pub fn best_ask(&self) -> Option<f64> {
        self.asks.first().map(|entry| entry.price)
    }

    /// Get bid-ask spread
    pub fn spread(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Get mid price
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some((ask + bid) / 2.0),
            _ => None,
        }
    }

    /// Calculate total bid volume
    pub fn total_bid_volume(&self) -> f64 {
        self.bids.iter().map(|entry| entry.amount).sum()
    }

    /// Calculate total ask volume
    pub fn total_ask_volume(&self) -> f64 {
        self.asks.iter().map(|entry| entry.amount).sum()
    }

    /// Get volume at specific price level
    pub fn volume_at_price(&self, price: f64, is_bid: bool) -> f64 {
        let levels = if is_bid { &self.bids } else { &self.asks };
        levels
            .iter()
            .find(|entry| (entry.price - price).abs() < f64::EPSILON)
            .map(|entry| entry.amount)
            .unwrap_or(0.0)
    }
}