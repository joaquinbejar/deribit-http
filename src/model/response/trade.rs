/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::model::TradeAllocation;

#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct UserTradeResponse {
    /// Trade amount. For perpetual and inverse futures the amount is in USD units. 
    /// For options and linear futures it is the underlying base currency coin.
    pub amount: f64,

    /// Advanced type of user order: "usd" or "implv" (only for options; omitted if not applicable)
    pub advanced: Option<String>,

    /// true if user order was created with API
    pub api: bool,

    /// ID of the Block RFQ - when trade was part of the Block RFQ
    pub block_rfq_id: Option<u64>,

    /// ID of the Block RFQ quote - when trade was part of the Block RFQ
    pub block_rfq_quote_id: Option<u64>,

    /// Block trade id - when trade was part of a block trade
    pub block_trade_id: Option<String>,

    /// Optional field containing combo instrument name if the trade is a combo trade
    pub combo_id: Option<String>,

    /// Optional field containing combo trade identifier if the trade is a combo trade
    pub combo_trade_id: Option<f64>,

    /// Trade size in contract units (optional, may be absent in historical trades)
    pub contracts: Option<f64>,

    /// Direction: buy, or sell
    pub direction: String,

    /// User's fee in units of the specified fee_currency
    pub fee: f64,

    /// Currency, i.e "BTC", "ETH", "USDC"
    pub fee_currency: String,

    /// Index Price at the moment of trade
    pub index_price: f64,

    /// Unique instrument identifier
    pub instrument_name: String,

    /// Option implied volatility for the price (Option only)
    pub iv: Option<f64>,

    /// User defined label (presented only when previously set for order by user)
    pub label: Option<String>,

    /// Optional field containing leg trades if trade is a combo trade
    pub legs: Option<Vec<serde_json::Value>>,

    /// Optional field (only for trades caused by liquidation): 
    /// "M" when maker side was under liquidation, "T" when taker side was under liquidation, 
    /// "MT" when both sides were under liquidation
    pub liquidation: Option<String>,

    /// Describes what was role of users order: "M" when it was maker order, "T" when it was taker order
    pub liquidity: String,

    /// Mark Price at the moment of trade
    pub mark_price: f64,

    /// Always null according to docs
    pub matching_id: Option<String>,

    /// true if user order is MMP
    pub mmp: bool,

    /// Id of the user order (maker or taker), i.e. subscriber's order id that took part in the trade
    pub order_id: String,

    /// Order type: "limit", "market", or "liquidation"
    pub order_type: String,

    /// true if user order is post-only
    pub post_only: Option<String>,

    /// Price in base currency
    pub price: f64,

    /// Profit and loss in base currency
    pub profit_loss: f64,

    /// QuoteID of the user order (optional, present only for orders placed with private/mass_quote)
    pub quote_id: Option<String>,

    /// QuoteSet of the user order (optional, present only for orders placed with private/mass_quote)
    pub quote_set_id: Option<String>,

    /// true if user order is reduce-only
    pub reduce_only: Option<String>,

    /// true if user order is marked by the platform as a risk reducing order 
    /// (can apply only to orders placed by PM users)
    pub risk_reducing: bool,

    /// Order state: "open", "filled", "rejected", "cancelled", "untriggered" or "archive"
    pub state: String,

    /// Direction of the "tick" (0 = Plus Tick, 1 = Zero-Plus Tick, 2 = Minus Tick, 3 = Zero-Minus Tick)
    pub tick_direction: i32,

    /// The timestamp of the trade (milliseconds since the UNIX epoch)
    pub timestamp: u64,

    /// List of allocations for Block RFQ pre-allocation
    pub trade_allocations: Option<Vec<TradeAllocation>>,

    /// The sequence number of the trade within instrument
    pub trade_seq: u64,

    /// Underlying price for implied volatility calculations (Options only)
    pub underlying_price: Option<f64>,
}