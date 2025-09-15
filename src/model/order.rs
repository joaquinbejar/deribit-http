/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Order status enumeration
#[derive(DebugPretty, DisplaySimple, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// Order has been accepted by the system
    New,
    /// Order has been partially filled
    PartiallyFilled,
    /// Order has been completely filled
    Filled,
    /// Order is done for the day
    DoneForDay,
    /// Order has been cancelled
    Canceled,
    /// Order has been replaced
    Replaced,
    /// Order cancellation is pending
    PendingCancel,
    /// Order has been stopped
    Stopped,
    /// Order has been rejected
    Rejected,
    /// Order has been suspended
    Suspended,
    /// Order is pending acceptance
    PendingNew,
    /// Order has been calculated
    Calculated,
    /// Order has expired
    Expired,
    /// Order has been accepted for bidding
    AcceptedForBidding,
    /// Order replacement is pending
    PendingReplace,
}


/// Order side enumeration
#[derive(DebugPretty, DisplaySimple, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

/// Order type enum
#[derive(DebugPretty, DisplaySimple, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// Limit order - executes at specified price or better
    #[serde(rename = "limit")]
    Limit,
    /// Market order - executes immediately at best available price
    #[serde(rename = "market")]
    Market,
    /// Stop limit order - becomes limit order when stop price is reached
    #[serde(rename = "stop_limit")]
    StopLimit,
    /// Stop market order - becomes market order when stop price is reached
    #[serde(rename = "stop_market")]
    StopMarket,
    /// Take limit order - limit order to take profit
    #[serde(rename = "take_limit")]
    TakeLimit,
    /// Take market order - market order to take profit
    #[serde(rename = "take_market")]
    TakeMarket,
    /// Market limit order - market order with limit price protection
    #[serde(rename = "market_limit")]
    MarketLimit,
    /// Trailing stop order - stop order that trails the market price
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

impl OrderType {
    /// Returns the string representation of the order type
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
            OrderType::StopLimit => "stop_limit",
            OrderType::StopMarket => "stop_market",
            OrderType::TakeLimit => "take_limit",
            OrderType::TakeMarket => "take_market",
            OrderType::MarketLimit => "market_limit",
            OrderType::TrailingStop => "trailing_stop",
        }
    }
}