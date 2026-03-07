/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 7/3/26
******************************************************************************/
//! Block RFQ response models for Request for Quote workflow.

use crate::model::types::Direction;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// State of a Block RFQ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BlockRfqState {
    /// Block RFQ is open and accepting quotes
    #[default]
    Open,
    /// Block RFQ has been filled
    Filled,
    /// Block RFQ has been traded
    Traded,
    /// Block RFQ has been cancelled
    Cancelled,
    /// Block RFQ has expired
    Expired,
    /// Block RFQ is closed
    Closed,
    /// Block RFQ was created
    Created,
}

/// Role of user in Block RFQ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BlockRfqRole {
    /// User is the taker (creator) of the RFQ
    #[default]
    Taker,
    /// User is a maker (quoter)
    Maker,
    /// Any role
    Any,
}

/// State of a Block RFQ quote
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum QuoteState {
    /// Quote is open
    #[default]
    Open,
    /// Quote has been filled
    Filled,
    /// Quote has been cancelled
    Cancelled,
}

/// Execution instruction for quotes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionInstruction {
    /// Quote can only be filled entirely or not at all
    AllOrNone,
    /// Quote can be filled partially or fully
    #[default]
    AnyPartOf,
}

/// Time in force for accepting Block RFQ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BlockRfqTimeInForce {
    /// Fill immediately or cancel
    #[default]
    FillOrKill,
    /// Good until cancelled
    GoodTilCancelled,
}

/// Leg of a Block RFQ
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfqLeg {
    /// Instrument name (e.g., "BTC-PERPETUAL")
    pub instrument_name: String,
    /// Direction of the leg
    pub direction: Direction,
    /// Ratio of amount between legs
    #[serde(default)]
    pub ratio: Option<f64>,
    /// Amount for the leg (used in create_block_rfq)
    #[serde(default)]
    pub amount: Option<f64>,
    /// Price for the leg (used in quotes)
    #[serde(default)]
    pub price: Option<f64>,
}

/// Hedge leg of a Block RFQ
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfqHedge {
    /// Instrument name (e.g., "BTC-PERPETUAL")
    pub instrument_name: String,
    /// Direction of the hedge
    pub direction: Direction,
    /// Price for the hedge
    pub price: f64,
    /// Amount for the hedge
    pub amount: f64,
}

/// Bid/Ask quote in Block RFQ
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockRfqBidAsk {
    /// Maker of the quote
    #[serde(default)]
    pub maker: Option<String>,
    /// Price of the quote
    pub price: f64,
    /// Timestamp of last update (milliseconds since Unix epoch)
    #[serde(default)]
    pub last_update_timestamp: Option<i64>,
    /// Execution instruction
    #[serde(default)]
    pub execution_instruction: Option<ExecutionInstruction>,
    /// Amount of the quote
    #[serde(default)]
    pub amount: Option<f64>,
}

/// Trade allocation for Block RFQ pre-allocation
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockRfqTradeAllocation {
    /// User ID (subaccount or main account)
    #[serde(default)]
    pub user_id: Option<i64>,
    /// Client info for broker allocation
    #[serde(default)]
    pub client_info: Option<BlockRfqClientInfo>,
    /// Amount allocated
    pub amount: f64,
}

/// Client info for broker allocation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockRfqClientInfo {
    /// Client ID
    pub client_id: String,
    /// User ID within client
    #[serde(default)]
    pub user_id: Option<String>,
}

/// Index prices in Block RFQ trade
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexPrices {
    /// BTC/USD index price
    #[serde(default)]
    pub btc_usd: Option<f64>,
    /// BTC/USDC index price
    #[serde(default)]
    pub btc_usdc: Option<f64>,
    /// ETH/USD index price
    #[serde(default)]
    pub eth_usd: Option<f64>,
    /// ETH/USDC index price
    #[serde(default)]
    pub eth_usdc: Option<f64>,
}

/// Block RFQ representation
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfq {
    /// Block RFQ ID
    pub block_rfq_id: i64,
    /// State of the Block RFQ
    pub state: BlockRfqState,
    /// Role of the user
    pub role: BlockRfqRole,
    /// Total amount
    pub amount: f64,
    /// Minimum amount for trading
    #[serde(default)]
    pub min_trade_amount: Option<f64>,
    /// Combo ID
    #[serde(default)]
    pub combo_id: Option<String>,
    /// List of legs
    pub legs: Vec<BlockRfqLeg>,
    /// Hedge leg (optional)
    #[serde(default)]
    pub hedge: Option<BlockRfqHedge>,
    /// Creation timestamp (milliseconds since Unix epoch)
    pub creation_timestamp: i64,
    /// Expiration timestamp (milliseconds since Unix epoch)
    pub expiration_timestamp: i64,
    /// User-defined label
    #[serde(default)]
    pub label: Option<String>,
    /// List of targeted makers
    #[serde(default)]
    pub makers: Option<Vec<String>>,
    /// Taker rating
    #[serde(default)]
    pub taker_rating: Option<String>,
    /// Bid quotes
    #[serde(default)]
    pub bids: Option<Vec<BlockRfqBidAsk>>,
    /// Ask quotes
    #[serde(default)]
    pub asks: Option<Vec<BlockRfqBidAsk>>,
    /// Mark price (for filled RFQs)
    #[serde(default)]
    pub mark_price: Option<f64>,
    /// Trades (for filled RFQs)
    #[serde(default)]
    pub trades: Option<Vec<BlockRfqTradeInfo>>,
}

impl BlockRfq {
    /// Returns `true` if the Block RFQ is open
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.state == BlockRfqState::Open
    }

    /// Returns `true` if the Block RFQ is filled
    #[must_use]
    pub fn is_filled(&self) -> bool {
        self.state == BlockRfqState::Filled
    }

    /// Returns `true` if the Block RFQ is cancelled
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.state == BlockRfqState::Cancelled
    }

    /// Returns `true` if the user is the taker
    #[must_use]
    pub fn is_taker(&self) -> bool {
        self.role == BlockRfqRole::Taker
    }

    /// Returns `true` if the user is a maker
    #[must_use]
    pub fn is_maker(&self) -> bool {
        self.role == BlockRfqRole::Maker
    }
}

/// Trade info within a Block RFQ
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfqTradeInfo {
    /// Trade price
    pub price: f64,
    /// Trade amount
    pub amount: f64,
    /// Trade direction
    pub direction: Direction,
    /// Hedge amount (optional)
    #[serde(default)]
    pub hedge_amount: Option<f64>,
}

/// Block RFQ quote response
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfqQuote {
    /// Quote ID
    pub block_rfq_quote_id: i64,
    /// Block RFQ ID
    pub block_rfq_id: i64,
    /// Quote state
    pub quote_state: QuoteState,
    /// Price
    pub price: f64,
    /// Amount
    pub amount: f64,
    /// Direction
    pub direction: Direction,
    /// Filled amount
    #[serde(default)]
    pub filled_amount: Option<f64>,
    /// Legs
    pub legs: Vec<BlockRfqLeg>,
    /// Hedge (optional)
    #[serde(default)]
    pub hedge: Option<BlockRfqHedge>,
    /// Execution instruction
    #[serde(default)]
    pub execution_instruction: Option<ExecutionInstruction>,
    /// Creation timestamp (milliseconds since Unix epoch)
    pub creation_timestamp: i64,
    /// Last update timestamp (milliseconds since Unix epoch)
    pub last_update_timestamp: i64,
    /// Whether the quote was replaced/edited
    #[serde(default)]
    pub replaced: Option<bool>,
    /// User-defined label
    #[serde(default)]
    pub label: Option<String>,
    /// Application name
    #[serde(default)]
    pub app_name: Option<String>,
    /// Cancellation reason
    #[serde(default)]
    pub cancel_reason: Option<String>,
}

impl BlockRfqQuote {
    /// Returns `true` if the quote is open
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.quote_state == QuoteState::Open
    }

    /// Returns `true` if the quote is filled
    #[must_use]
    pub fn is_filled(&self) -> bool {
        self.quote_state == QuoteState::Filled
    }

    /// Returns `true` if the quote is cancelled
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.quote_state == QuoteState::Cancelled
    }
}

/// Public Block RFQ trade (from get_block_rfq_trades)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfqPublicTrade {
    /// Block RFQ ID
    pub id: i64,
    /// Timestamp (milliseconds since Unix epoch)
    pub timestamp: i64,
    /// Combo ID
    #[serde(default)]
    pub combo_id: Option<String>,
    /// Legs
    pub legs: Vec<BlockRfqLeg>,
    /// Amount
    pub amount: f64,
    /// Direction
    pub direction: Direction,
    /// Mark price
    #[serde(default)]
    pub mark_price: Option<f64>,
    /// Trades
    #[serde(default)]
    pub trades: Option<Vec<BlockRfqTradeInfo>>,
    /// Hedge (optional)
    #[serde(default)]
    pub hedge: Option<BlockRfqHedge>,
    /// Index prices
    #[serde(default)]
    pub index_prices: Option<IndexPrices>,
}

/// Response for get_block_rfq_trades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfqTradesResponse {
    /// Continuation token for pagination
    #[serde(default)]
    pub continuation: Option<String>,
    /// List of Block RFQ trades
    pub block_rfqs: Vec<BlockRfqPublicTrade>,
}

impl BlockRfqTradesResponse {
    /// Returns `true` if there are no trades
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.block_rfqs.is_empty()
    }

    /// Returns the number of trades
    #[must_use]
    pub fn len(&self) -> usize {
        self.block_rfqs.len()
    }

    /// Returns `true` if there are more results
    #[must_use]
    pub fn has_more(&self) -> bool {
        self.continuation.is_some()
    }
}

/// Response for get_block_rfqs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfqsResponse {
    /// Continuation token for pagination
    #[serde(default)]
    pub continuation: Option<String>,
    /// List of Block RFQs
    pub block_rfqs: Vec<BlockRfq>,
}

impl BlockRfqsResponse {
    /// Returns `true` if there are no RFQs
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.block_rfqs.is_empty()
    }

    /// Returns the number of RFQs
    #[must_use]
    pub fn len(&self) -> usize {
        self.block_rfqs.len()
    }

    /// Returns `true` if there are more results
    #[must_use]
    pub fn has_more(&self) -> bool {
        self.continuation.is_some()
    }
}

/// Individual trade in accept_block_rfq response
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfqAcceptTrade {
    /// Trade ID
    pub trade_id: String,
    /// Trade sequence number
    #[serde(default)]
    pub trade_seq: Option<i64>,
    /// Instrument name
    pub instrument_name: String,
    /// Timestamp (milliseconds since Unix epoch)
    pub timestamp: i64,
    /// Trade state
    pub state: String,
    /// Fee
    #[serde(default)]
    pub fee: Option<f64>,
    /// Fee currency
    #[serde(default)]
    pub fee_currency: Option<String>,
    /// Amount
    pub amount: f64,
    /// Direction
    pub direction: Direction,
    /// Price
    pub price: f64,
    /// Index price
    #[serde(default)]
    pub index_price: Option<f64>,
    /// Mark price
    #[serde(default)]
    pub mark_price: Option<f64>,
    /// Profit/loss
    #[serde(default)]
    pub profit_loss: Option<f64>,
    /// Order ID
    #[serde(default)]
    pub order_id: Option<String>,
    /// Order type
    #[serde(default)]
    pub order_type: Option<String>,
    /// Tick direction
    #[serde(default)]
    pub tick_direction: Option<i32>,
    /// Combo ID
    #[serde(default)]
    pub combo_id: Option<String>,
    /// Block RFQ ID
    #[serde(default)]
    pub block_rfq_id: Option<i64>,
    /// Block trade ID
    #[serde(default)]
    pub block_trade_id: Option<String>,
    /// Block trade leg count
    #[serde(default)]
    pub block_trade_leg_count: Option<i32>,
    /// Whether API was used
    #[serde(default)]
    pub api: Option<bool>,
    /// Number of contracts
    #[serde(default)]
    pub contracts: Option<f64>,
    /// Post only flag
    #[serde(default)]
    pub post_only: Option<bool>,
    /// MMP flag
    #[serde(default)]
    pub mmp: Option<bool>,
    /// Risk reducing flag
    #[serde(default)]
    pub risk_reducing: Option<bool>,
    /// Reduce only flag
    #[serde(default)]
    pub reduce_only: Option<bool>,
    /// Self trade flag
    #[serde(default)]
    pub self_trade: Option<bool>,
    /// Liquidity indicator (T=taker, M=maker)
    #[serde(default)]
    pub liquidity: Option<String>,
    /// Matching ID
    #[serde(default)]
    pub matching_id: Option<String>,
}

/// Block trade in accept_block_rfq response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRfqAcceptBlockTrade {
    /// Block trade ID
    pub id: String,
    /// Timestamp (milliseconds since Unix epoch)
    pub timestamp: i64,
    /// Individual trades
    pub trades: Vec<BlockRfqAcceptTrade>,
}

/// Response for accept_block_rfq
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptBlockRfqResponse {
    /// Block trades
    pub block_trades: Vec<BlockRfqAcceptBlockTrade>,
}

impl AcceptBlockRfqResponse {
    /// Returns the total number of trades across all block trades
    #[must_use]
    pub fn total_trades(&self) -> usize {
        self.block_trades.iter().map(|bt| bt.trades.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_rfq_state_deserialization() {
        let json = r#""open""#;
        let state: BlockRfqState = serde_json::from_str(json).unwrap();
        assert_eq!(state, BlockRfqState::Open);

        let json = r#""cancelled""#;
        let state: BlockRfqState = serde_json::from_str(json).unwrap();
        assert_eq!(state, BlockRfqState::Cancelled);
    }

    #[test]
    fn test_block_rfq_role_deserialization() {
        let json = r#""taker""#;
        let role: BlockRfqRole = serde_json::from_str(json).unwrap();
        assert_eq!(role, BlockRfqRole::Taker);

        let json = r#""maker""#;
        let role: BlockRfqRole = serde_json::from_str(json).unwrap();
        assert_eq!(role, BlockRfqRole::Maker);
    }

    #[test]
    fn test_block_rfq_leg_deserialization() {
        let json = r#"{
            "instrument_name": "BTC-PERPETUAL",
            "direction": "buy",
            "ratio": 1,
            "price": 70000
        }"#;

        let leg: BlockRfqLeg = serde_json::from_str(json).unwrap();
        assert_eq!(leg.instrument_name, "BTC-PERPETUAL");
        assert!(matches!(leg.direction, Direction::Buy));
        assert_eq!(leg.ratio, Some(1.0));
        assert_eq!(leg.price, Some(70000.0));
    }

    #[test]
    fn test_block_rfq_deserialization() {
        let json = r#"{
            "block_rfq_id": 507,
            "state": "created",
            "role": "taker",
            "amount": 20000,
            "combo_id": "BTC-15NOV24",
            "legs": [
                {
                    "direction": "sell",
                    "instrument_name": "BTC-15NOV24",
                    "ratio": 1
                }
            ],
            "creation_timestamp": 1731062187555,
            "expiration_timestamp": 1731062487555,
            "bids": [],
            "asks": [],
            "makers": ["MAKER1"]
        }"#;

        let rfq: BlockRfq = serde_json::from_str(json).unwrap();
        assert_eq!(rfq.block_rfq_id, 507);
        assert_eq!(rfq.state, BlockRfqState::Created);
        assert!(rfq.is_taker());
        assert_eq!(rfq.amount, 20000.0);
        assert_eq!(rfq.legs.len(), 1);
    }

    #[test]
    fn test_block_rfq_quote_deserialization() {
        let json = r#"{
            "block_rfq_quote_id": 8,
            "block_rfq_id": 3,
            "quote_state": "open",
            "price": 69600,
            "amount": 10000,
            "direction": "buy",
            "legs": [
                {
                    "direction": "buy",
                    "price": 69600,
                    "instrument_name": "BTC-15NOV24",
                    "ratio": 1
                }
            ],
            "creation_timestamp": 1731076586371,
            "last_update_timestamp": 1731076586371,
            "replaced": false,
            "filled_amount": 0,
            "execution_instruction": "all_or_none"
        }"#;

        let quote: BlockRfqQuote = serde_json::from_str(json).unwrap();
        assert_eq!(quote.block_rfq_quote_id, 8);
        assert!(quote.is_open());
        assert_eq!(
            quote.execution_instruction,
            Some(ExecutionInstruction::AllOrNone)
        );
    }

    #[test]
    fn test_block_rfq_trades_response_deserialization() {
        let json = r#"{
            "continuation": "1739739009234:6570",
            "block_rfqs": [
                {
                    "id": 6611,
                    "timestamp": 1739803305362,
                    "combo_id": "BTC-CS-28FEB25-100000_106000",
                    "legs": [
                        {
                            "price": 0.1,
                            "direction": "buy",
                            "instrument_name": "BTC-28FEB25-100000-C",
                            "ratio": 1
                        }
                    ],
                    "amount": 12.5,
                    "direction": "sell",
                    "mark_price": 0.010356754
                }
            ]
        }"#;

        let response: BlockRfqTradesResponse = serde_json::from_str(json).unwrap();
        assert!(response.has_more());
        assert_eq!(response.len(), 1);
        assert_eq!(response.block_rfqs[0].id, 6611);
    }
}
