//! REST API endpoints implementation
//!
//! This module implements all Deribit REST API endpoints including
//! market data, trading, account management, and system endpoints.

use crate::DeribitHttpClient;
use crate::error::HttpError;
use crate::model::http_types::ApiResponse;
use serde::{Deserialize, Serialize};

/// Market data endpoints
impl DeribitHttpClient {
    /// Get server time
    ///
    /// Returns the current server time in milliseconds since Unix epoch.
    /// This is a public endpoint that doesn't require authentication.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true); // testnet
    /// let server_time = client.get_server_time().await?;
    /// println!("Server time: {}", server_time);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_server_time(&self) -> Result<u64, HttpError> {
        let url = format!("{}/public/get_time", self.base_url());

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get server time failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<u64> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No server time in response".to_string()))
    }

    /// Test connectivity to the API
    ///
    /// Returns a simple "ok" response to test API connectivity.
    /// This is a public endpoint that doesn't require authentication.
    pub async fn test_connection(&self) -> Result<String, HttpError> {
        let url = format!("{}/public/test", self.base_url());

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Test connection failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<String> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No test result in response".to_string()))
    }

    /// Get ticker information for an instrument
    ///
    /// Returns ticker data including last price, bid/ask, volume, etc.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier (e.g., "BTC-PERPETUAL")
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use deribit_http::DeribitHttpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DeribitHttpClient::new(true);
    /// let ticker = client.get_ticker("BTC-PERPETUAL").await?;
    /// println!("Last price: {}", ticker.last_price);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ticker(&self, instrument_name: &str) -> Result<TickerData, HttpError> {
        let url = format!(
            "{}/public/ticker?instrument_name={}",
            self.base_url(),
            instrument_name
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get ticker failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<TickerData> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No ticker data in response".to_string()))
    }

    /// Get order book for an instrument
    ///
    /// Returns the current order book with bids and asks.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier
    /// * `depth` - Optional depth of the order book (default: 5)
    pub async fn get_order_book(
        &self,
        instrument_name: &str,
        depth: Option<u32>,
    ) -> Result<OrderBook, HttpError> {
        let mut url = format!(
            "{}/public/get_order_book?instrument_name={}",
            self.base_url(),
            instrument_name
        );

        if let Some(d) = depth {
            url.push_str(&format!("&depth={}", d));
        }

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get order book failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<OrderBook> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No order book data in response".to_string()))
    }

    /// Get available instruments
    ///
    /// Returns a list of available trading instruments.
    ///
    /// # Arguments
    ///
    /// * `currency` - The currency (e.g., "BTC", "ETH")
    /// * `kind` - Optional instrument kind ("future", "option", "spot")
    /// * `expired` - Whether to include expired instruments
    pub async fn get_instruments(
        &self,
        currency: &str,
        kind: Option<&str>,
        expired: Option<bool>,
    ) -> Result<Vec<Instrument>, HttpError> {
        let mut url = format!(
            "{}/public/get_instruments?currency={}",
            self.base_url(),
            currency
        );

        if let Some(k) = kind {
            url.push_str(&format!("&kind={}", k));
        }

        if let Some(exp) = expired {
            url.push_str(&format!("&expired={}", exp));
        }

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get instruments failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<Instrument>> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response.result.ok_or_else(|| {
            HttpError::InvalidResponse("No instruments data in response".to_string())
        })
    }

    /// Get recent trades for an instrument
    ///
    /// Returns recent trade history for the specified instrument.
    ///
    /// # Arguments
    ///
    /// * `instrument_name` - The instrument identifier
    /// * `count` - Optional number of trades to return (default: 10, max: 1000)
    /// * `include_old` - Whether to include old trades
    pub async fn get_last_trades(
        &self,
        instrument_name: &str,
        count: Option<u32>,
        include_old: Option<bool>,
    ) -> Result<Vec<Trade>, HttpError> {
        let mut url = format!(
            "{}/public/get_last_trades_by_instrument?instrument_name={}",
            self.base_url(),
            instrument_name
        );

        if let Some(c) = count {
            url.push_str(&format!("&count={}", c));
        }

        if let Some(old) = include_old {
            url.push_str(&format!("&include_old={}", old));
        }

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Get last trades failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<Vec<Trade>> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No trades data in response".to_string()))
    }
}

/// Ticker data structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerData {
    /// Instrument name
    pub instrument_name: String,
    /// Last trade price
    pub last_price: f64,
    /// Best bid price
    pub best_bid_price: Option<f64>,
    /// Best ask price
    pub best_ask_price: Option<f64>,
    /// Best bid amount
    pub best_bid_amount: Option<f64>,
    /// Best ask amount
    pub best_ask_amount: Option<f64>,
    /// Current funding rate
    pub current_funding: Option<f64>,
    /// Estimated delivery price
    pub estimated_delivery_price: Option<f64>,
    /// 8h funding rate
    pub funding_8h: Option<f64>,
    /// Index price
    pub index_price: Option<f64>,
    /// Interest value
    pub interest_value: Option<f64>,
    /// Mark price
    pub mark_price: Option<f64>,
    /// Maximum price
    pub max_price: Option<f64>,
    /// Minimum price
    pub min_price: Option<f64>,
    /// Open interest
    pub open_interest: Option<f64>,
    /// Settlement price
    pub settlement_price: Option<f64>,
    /// Instrument state
    pub state: Option<String>,
    /// Statistics
    pub stats: Option<TickerStats>,
    /// Timestamp
    pub timestamp: u64,
}

/// Ticker statistics structure
#[derive(Clone, Serialize, Deserialize)]
pub struct TickerStats {
    /// 24h high price
    pub high: Option<f64>,
    /// 24h low price
    pub low: Option<f64>,
    /// Price change percentage
    pub price_change: Option<f64>,
    /// 24h volume
    pub volume: Option<f64>,
    /// 24h volume in USD
    pub volume_usd: Option<f64>,
}

/// Order book structure
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// Instrument name
    pub instrument_name: String,
    /// Bids (price, amount)
    pub bids: Vec<[f64; 2]>,
    /// Asks (price, amount)
    pub asks: Vec<[f64; 2]>,
    /// Timestamp
    pub timestamp: u64,
    /// Change ID
    pub change_id: Option<u64>,
}

/// Instrument information
#[derive(Clone, Serialize, Deserialize)]
pub struct Instrument {
    /// Instrument name
    pub instrument_name: String,
    /// Instrument kind
    pub kind: String,
    /// Base currency
    pub base_currency: String,
    /// Quote currency
    pub quote_currency: String,
    /// Settlement currency
    pub settlement_currency: String,
    /// Contract size
    pub contract_size: f64,
    /// Minimum trade amount
    pub min_trade_amount: f64,
    /// Tick size
    pub tick_size: f64,
    /// Is active
    pub is_active: bool,
    /// Expiration timestamp
    pub expiration_timestamp: Option<u64>,
    /// Creation timestamp
    pub creation_timestamp: u64,
}

/// Trade information
#[derive(Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Trade ID
    pub trade_id: String,
    /// Instrument name
    pub instrument_name: String,
    /// Trade price
    pub price: f64,
    /// Trade amount
    pub amount: f64,
    /// Trade direction
    pub direction: String,
    /// Timestamp
    pub timestamp: u64,
    /// Index price
    pub index_price: Option<f64>,
}

/// Trading endpoints
impl DeribitHttpClient {
    /// Place a buy order
    ///
    /// Places a buy order for the specified instrument.
    ///
    /// # Arguments
    ///
    /// * `request` - The buy order request parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::{DeribitHttpClient, BuyOrderRequest, OrderType};
    /// 
    /// let client = DeribitHttpClient::new(true);
    /// let request = BuyOrderRequest {
    ///     instrument_name: "BTC-PERPETUAL".to_string(),
    ///     amount: Some(100.0),
    ///     contracts: None,
    ///     order_type: OrderType::Market,
    ///     price: None,
    ///     label: Some("test_order".to_string()),
    ///     ..Default::default()
    /// };
    /// // let result = client.buy_order(request).await?;
    /// // println!("Order placed: {}", result.order.order_id);
    /// ```
    pub async fn buy_order(&self, request: BuyOrderRequest) -> Result<OrderResponse, HttpError> {
        let mut query_params = vec![
            ("instrument_name".to_string(), request.instrument_name),
        ];

        // Add amount or contracts (one is required)
        if let Some(amount) = request.amount {
            query_params.push(("amount".to_string(), amount.to_string()));
        } else if let Some(contracts) = request.contracts {
            query_params.push(("contracts".to_string(), contracts.to_string()));
        } else {
            return Err(HttpError::InvalidResponse(
                "Either amount or contracts must be specified".to_string(),
            ));
        }

        query_params.push(("type".to_string(), request.order_type.as_str().to_string()));

        if let Some(price) = request.price {
            query_params.push(("price".to_string(), price.to_string()));
        }

        if let Some(label) = request.label {
            query_params.push(("label".to_string(), label));
        }

        query_params.push(("time_in_force".to_string(), request.time_in_force.as_str().to_string()));

        if request.post_only {
            query_params.push(("post_only".to_string(), "true".to_string()));
        }

        if request.reduce_only {
            query_params.push(("reduce_only".to_string(), "true".to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}/private/buy?{}", self.base_url(), query_string);

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Buy order failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<OrderResponse> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No order data in response".to_string()))
    }

    /// Place a sell order
    ///
    /// Places a sell order for the specified instrument.
    ///
    /// # Arguments
    ///
    /// * `request` - The sell order request parameters
    pub async fn sell_order(&self, request: SellOrderRequest) -> Result<OrderResponse, HttpError> {
        let mut query_params = vec![
            ("instrument_name".to_string(), request.instrument_name),
        ];

        // Add amount or contracts (one is required)
        if let Some(amount) = request.amount {
            query_params.push(("amount".to_string(), amount.to_string()));
        } else if let Some(contracts) = request.contracts {
            query_params.push(("contracts".to_string(), contracts.to_string()));
        } else {
            return Err(HttpError::InvalidResponse(
                "Either amount or contracts must be specified".to_string(),
            ));
        }

        query_params.push(("type".to_string(), request.order_type.as_str().to_string()));

        if let Some(price) = request.price {
            query_params.push(("price".to_string(), price.to_string()));
        }

        if let Some(label) = request.label {
            query_params.push(("label".to_string(), label));
        }

        query_params.push(("time_in_force".to_string(), request.time_in_force.as_str().to_string()));

        if request.post_only {
            query_params.push(("post_only".to_string(), "true".to_string()));
        }

        if request.reduce_only {
            query_params.push(("reduce_only".to_string(), "true".to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("{}/private/sell?{}", self.base_url(), query_string);

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Sell order failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<OrderResponse> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No order data in response".to_string()))
    }

    /// Cancel an order
    ///
    /// Cancels an existing order by order ID.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The ID of the order to cancel
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deribit_http::DeribitHttpClient;
    /// 
    /// let client = DeribitHttpClient::new(true);
    /// // let result = client.cancel_order("BTC-12345").await?;
    /// // println!("Order cancelled: {}", result.order_id);
    /// ```
    pub async fn cancel_order(&self, order_id: &str) -> Result<OrderInfo, HttpError> {
        let url = format!(
            "{}/private/cancel?order_id={}",
            self.base_url(),
            urlencoding::encode(order_id)
        );

        let response = self
            .http_client()
            .get(&url)
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(HttpError::RequestFailed(format!(
                "Cancel order failed: {}",
                error_text
            )));
        }

        let api_response: ApiResponse<OrderInfo> = response
            .json()
            .await
            .map_err(|e| HttpError::InvalidResponse(e.to_string()))?;

        if let Some(error) = api_response.error {
            return Err(HttpError::RequestFailed(format!(
                "API error: {} - {}",
                error.code, error.message
            )));
        }

        api_response
            .result
            .ok_or_else(|| HttpError::InvalidResponse("No order data in response".to_string()))
    }
}

/// Order type enumeration
#[derive(Clone, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "stop_limit")]
    StopLimit,
    #[serde(rename = "stop_market")]
    StopMarket,
    #[serde(rename = "take_limit")]
    TakeLimit,
    #[serde(rename = "take_market")]
    TakeMarket,
    #[serde(rename = "market_limit")]
    MarketLimit,
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

impl OrderType {
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

/// Time in force enumeration
#[derive(Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "good_til_cancelled")]
    GoodTilCancelled,
    #[serde(rename = "good_til_day")]
    GoodTilDay,
    #[serde(rename = "fill_or_kill")]
    FillOrKill,
    #[serde(rename = "immediate_or_cancel")]
    ImmediateOrCancel,
}

impl TimeInForce {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeInForce::GoodTilCancelled => "good_til_cancelled",
            TimeInForce::GoodTilDay => "good_til_day",
            TimeInForce::FillOrKill => "fill_or_kill",
            TimeInForce::ImmediateOrCancel => "immediate_or_cancel",
        }
    }
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::GoodTilCancelled
    }
}

/// Buy order request structure
#[derive(Clone, Serialize, Deserialize)]
pub struct BuyOrderRequest {
    /// Instrument name
    pub instrument_name: String,
    /// Order amount (alternative to contracts)
    pub amount: Option<f64>,
    /// Order size in contracts (alternative to amount)
    pub contracts: Option<f64>,
    /// Order type
    pub order_type: OrderType,
    /// Order price (required for limit orders)
    pub price: Option<f64>,
    /// User-defined label
    pub label: Option<String>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Post-only flag
    pub post_only: bool,
    /// Reduce-only flag
    pub reduce_only: bool,
}

impl Default for BuyOrderRequest {
    fn default() -> Self {
        Self {
            instrument_name: String::new(),
            amount: None,
            contracts: None,
            order_type: OrderType::Limit,
            price: None,
            label: None,
            time_in_force: TimeInForce::default(),
            post_only: false,
            reduce_only: false,
        }
    }
}

/// Sell order request structure
#[derive(Clone, Serialize, Deserialize)]
pub struct SellOrderRequest {
    /// Instrument name
    pub instrument_name: String,
    /// Order amount (alternative to contracts)
    pub amount: Option<f64>,
    /// Order size in contracts (alternative to amount)
    pub contracts: Option<f64>,
    /// Order type
    pub order_type: OrderType,
    /// Order price (required for limit orders)
    pub price: Option<f64>,
    /// User-defined label
    pub label: Option<String>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Post-only flag
    pub post_only: bool,
    /// Reduce-only flag
    pub reduce_only: bool,
}

impl Default for SellOrderRequest {
    fn default() -> Self {
        Self {
            instrument_name: String::new(),
            amount: None,
            contracts: None,
            order_type: OrderType::Limit,
            price: None,
            label: None,
            time_in_force: TimeInForce::default(),
            post_only: false,
            reduce_only: false,
        }
    }
}

/// Order response structure
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    /// List of trades executed
    pub trades: Vec<TradeExecution>,
    /// Order information
    pub order: OrderInfo,
}

/// Trade execution information
#[derive(Clone, Serialize, Deserialize)]
pub struct TradeExecution {
    /// Trade sequence number
    pub trade_seq: u64,
    /// Trade ID
    pub trade_id: String,
    /// Timestamp
    pub timestamp: u64,
    /// Tick direction
    pub tick_direction: i32,
    /// Trade state
    pub state: String,
    /// Reduce only flag
    pub reduce_only: bool,
    /// Trade price
    pub price: f64,
    /// Post only flag
    pub post_only: bool,
    /// Order type
    pub order_type: String,
    /// Order ID
    pub order_id: String,
    /// Matching ID
    pub matching_id: Option<String>,
    /// Mark price
    pub mark_price: f64,
    /// Liquidity (T=taker, M=maker)
    pub liquidity: String,
    /// Order label
    pub label: String,
    /// Instrument name
    pub instrument_name: String,
    /// Index price
    pub index_price: f64,
    /// Fee currency
    pub fee_currency: String,
    /// Fee amount
    pub fee: f64,
    /// Trade direction
    pub direction: String,
    /// Trade amount
    pub amount: f64,
}

/// Order information structure
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderInfo {
    /// Web order flag
    pub web: bool,
    /// Time in force
    pub time_in_force: String,
    /// Replaced flag
    pub replaced: bool,
    /// Reduce only flag
    pub reduce_only: bool,
    /// Order price
    pub price: f64,
    /// Post only flag
    pub post_only: bool,
    /// Order type
    pub order_type: String,
    /// Order state
    pub order_state: String,
    /// Order ID
    pub order_id: String,
    /// Maximum show amount
    pub max_show: f64,
    /// Last update timestamp
    pub last_update_timestamp: u64,
    /// Order label
    pub label: String,
    /// Rebalance flag
    pub is_rebalance: bool,
    /// Liquidation flag
    pub is_liquidation: bool,
    /// Instrument name
    pub instrument_name: String,
    /// Filled amount
    pub filled_amount: f64,
    /// Order direction
    pub direction: String,
    /// Creation timestamp
    pub creation_timestamp: u64,
    /// Average price
    pub average_price: f64,
    /// API flag
    pub api: bool,
    /// Order amount
    pub amount: f64,
}

// Implement Display and Debug traits using macros from deribit-base
deribit_base::impl_json_display!(TickerData);
deribit_base::impl_json_debug_pretty!(TickerData);

deribit_base::impl_json_display!(TickerStats);
deribit_base::impl_json_debug_pretty!(TickerStats);

deribit_base::impl_json_display!(OrderBook);
deribit_base::impl_json_debug_pretty!(OrderBook);

deribit_base::impl_json_display!(Instrument);
deribit_base::impl_json_debug_pretty!(Instrument);

deribit_base::impl_json_display!(Trade);
deribit_base::impl_json_debug_pretty!(Trade);

deribit_base::impl_json_display!(OrderType);
deribit_base::impl_json_debug_pretty!(OrderType);

deribit_base::impl_json_display!(TimeInForce);
deribit_base::impl_json_debug_pretty!(TimeInForce);

deribit_base::impl_json_display!(BuyOrderRequest);
deribit_base::impl_json_debug_pretty!(BuyOrderRequest);

deribit_base::impl_json_display!(SellOrderRequest);
deribit_base::impl_json_debug_pretty!(SellOrderRequest);

deribit_base::impl_json_display!(OrderResponse);
deribit_base::impl_json_debug_pretty!(OrderResponse);

deribit_base::impl_json_display!(TradeExecution);
deribit_base::impl_json_debug_pretty!(TradeExecution);

deribit_base::impl_json_display!(OrderInfo);
deribit_base::impl_json_debug_pretty!(OrderInfo);
