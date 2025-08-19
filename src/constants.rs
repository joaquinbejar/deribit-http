//! HTTP client constants

/// Default timeout for HTTP requests in seconds
pub const DEFAULT_TIMEOUT: u64 = 30;

/// Maximum number of retries for failed requests
pub const MAX_RETRIES: u32 = 3;

/// Production base URL for Deribit API
pub const PRODUCTION_BASE_URL: &str = "https://www.deribit.com/api/v2";

/// Testnet base URL for Deribit API
pub const TESTNET_BASE_URL: &str = "https://test.deribit.com/api/v2";

/// API endpoints
pub mod endpoints {
    /// Public authentication endpoint
    pub const AUTH: &str = "/public/auth";

    // Market data endpoints
    /// Get ticker information for an instrument
    pub const GET_TICKER: &str = "/public/ticker";
    /// Get order book for an instrument
    pub const GET_ORDERBOOK: &str = "/public/get_order_book";
    /// Get recent trades for an instrument
    pub const GET_TRADES: &str = "/public/get_last_trades_by_instrument";
    /// Get available trading instruments
    pub const GET_INSTRUMENTS: &str = "/public/get_instruments";

    // Trading endpoints
    /// Place a buy order
    pub const BUY: &str = "/private/buy";
    /// Place a sell order
    pub const SELL: &str = "/private/sell";
    /// Cancel a specific order
    pub const CANCEL: &str = "/private/cancel";
    /// Cancel all orders
    pub const CANCEL_ALL: &str = "/private/cancel_all";
    /// Get all open orders
    pub const GET_OPEN_ORDERS: &str = "/private/get_open_orders";

    // Account endpoints
    /// Get account summary information
    pub const GET_ACCOUNT_SUMMARY: &str = "/private/get_account_summary";
    /// Get current positions
    pub const GET_POSITIONS: &str = "/private/get_positions";
    /// Get subaccount information
    pub const GET_SUBACCOUNTS: &str = "/private/get_subaccounts";
}

/// HTTP headers
pub mod headers {
    /// Content-Type header name
    pub const CONTENT_TYPE: &str = "Content-Type";
    /// JSON content type value
    pub const APPLICATION_JSON: &str = "application/json";
    /// Authorization header name
    pub const AUTHORIZATION: &str = "Authorization";
    /// User-Agent header name
    pub const USER_AGENT: &str = "User-Agent";
}
