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
    // Authentication endpoints
    /// Public authentication endpoint
    pub const AUTH: &str = "/public/auth";

    // Public market data endpoints
    /// Get ticker information for an instrument
    pub const GET_TICKER: &str = "/public/ticker";
    /// Get instrument information by name
    pub const GET_INSTRUMENT: &str = "/public/get_instrument";
    /// Get list of available instruments
    pub const GET_INSTRUMENTS: &str = "/public/get_instruments";
    /// Get order book for an instrument
    pub const GET_ORDER_BOOK: &str = "/public/get_order_book";
    /// Get book summary by currency
    pub const GET_BOOK_SUMMARY_BY_CURRENCY: &str = "/public/get_book_summary_by_currency";
    /// Get book summary by instrument
    pub const GET_BOOK_SUMMARY_BY_INSTRUMENT: &str = "/public/get_book_summary_by_instrument";
    /// Get contract size for an instrument
    pub const GET_CONTRACT_SIZE: &str = "/public/get_contract_size";
    /// Get list of available currencies
    pub const GET_CURRENCIES: &str = "/public/get_currencies";
    /// Get index information
    pub const GET_INDEX: &str = "/public/get_index";
    /// Get index price
    pub const GET_INDEX_PRICE: &str = "/public/get_index_price";
    /// Get index price names
    pub const GET_INDEX_PRICE_NAMES: &str = "/public/get_index_price_names";
    /// Get server time
    pub const GET_SERVER_TIME: &str = "/public/get_time";
    /// Test connection to the API
    pub const TEST_CONNECTION: &str = "/public/test";
    /// Get API status
    pub const GET_STATUS: &str = "/public/status";
    /// Get APR history
    pub const GET_APR_HISTORY: &str = "/public/get_apr_history";
    /// Get options information
    pub const GET_OPTIONS: &str = "/public/get_options";
    /// Get options pair information
    pub const GET_OPTIONS_PAIR: &str = "/public/get_options_pair";
    /// Get last trades by instrument
    pub const GET_LAST_TRADES_BY_INSTRUMENT: &str = "/public/get_last_trades_by_instrument";
    /// Get historical volatility data
    pub const GET_HISTORICAL_VOLATILITY: &str = "/public/get_historical_volatility";
    /// Get funding chart data
    pub const GET_FUNDING_CHART_DATA: &str = "/public/get_funding_chart_data";
    /// Get TradingView chart data
    pub const GET_TRADINGVIEW_CHART_DATA: &str = "/public/get_tradingview_chart_data";
    /// Get delivery prices
    pub const GET_DELIVERY_PRICES: &str = "/public/get_delivery_prices";
    /// Get expiration dates
    pub const GET_EXPIRATIONS: &str = "/public/get_expirations";
    /// Get funding rate history
    pub const GET_FUNDING_RATE_HISTORY: &str = "/public/get_funding_rate_history";
    /// Get current funding rate value
    pub const GET_FUNDING_RATE_VALUE: &str = "/public/get_funding_rate_value";
    /// Get last settlements by currency
    pub const GET_LAST_SETTLEMENTS_BY_CURRENCY: &str = "/public/get_last_settlements_by_currency";
    /// Get last settlements by instrument
    pub const GET_LAST_SETTLEMENTS_BY_INSTRUMENT: &str =
        "/public/get_last_settlements_by_instrument";
    /// Get last trades by currency
    pub const GET_LAST_TRADES_BY_CURRENCY: &str = "/public/get_last_trades_by_currency";
    /// Get last trades by currency and time
    pub const GET_LAST_TRADES_BY_CURRENCY_AND_TIME: &str =
        "/public/get_last_trades_by_currency_and_time";
    /// Get last trades by instrument and time
    pub const GET_LAST_TRADES_BY_INSTRUMENT_AND_TIME: &str =
        "/public/get_last_trades_by_instrument_and_time";
    /// Get order book by instrument ID
    pub const GET_ORDER_BOOK_BY_INSTRUMENT_ID: &str = "/public/get_order_book_by_instrument_id";

    // Private trading endpoints
    /// Place a buy order
    pub const BUY: &str = "/private/buy";
    /// Place a sell order
    pub const SELL: &str = "/private/sell";
    /// Cancel a specific order
    pub const CANCEL: &str = "/private/cancel";
    /// Cancel all orders
    pub const CANCEL_ALL: &str = "/private/cancel_all";
    /// Cancel all orders by currency
    pub const CANCEL_ALL_BY_CURRENCY: &str = "/private/cancel_all_by_currency";
    /// Cancel all orders by currency pair
    pub const CANCEL_ALL_BY_CURRENCY_PAIR: &str = "/private/cancel_all_by_currency_pair";
    /// Cancel all orders by instrument
    pub const CANCEL_ALL_BY_INSTRUMENT: &str = "/private/cancel_all_by_instrument";
    /// Cancel all orders by kind or type
    pub const CANCEL_ALL_BY_KIND_OR_TYPE: &str = "/private/cancel_all_by_kind_or_type";
    /// Cancel orders by label
    pub const CANCEL_BY_LABEL: &str = "/private/cancel_by_label";
    /// Edit order
    pub const EDIT: &str = "/private/edit";
    /// Cancel quotes
    pub const CANCEL_QUOTES: &str = "/private/cancel_quotes";

    // Private account endpoints
    /// Get account summary information
    pub const GET_ACCOUNT_SUMMARY: &str = "/private/get_account_summary";
    /// Get position
    pub const GET_POSITION: &str = "/private/get_position";
    /// Get current positions
    pub const GET_POSITIONS: &str = "/private/get_positions";
    /// Get subaccount information
    pub const GET_SUBACCOUNTS: &str = "/private/get_subaccounts";
    /// Get transaction log
    pub const GET_TRANSACTION_LOG: &str = "/private/get_transaction_log";
    /// Get deposits
    pub const GET_DEPOSITS: &str = "/private/get_deposits";
    /// Get withdrawals
    pub const GET_WITHDRAWALS: &str = "/private/get_withdrawals";
    /// Submit transfer to subaccount
    pub const SUBMIT_TRANSFER_TO_SUBACCOUNT: &str = "/private/submit_transfer_to_subaccount";
    /// Submit transfer to user
    pub const SUBMIT_TRANSFER_TO_USER: &str = "/private/submit_transfer_to_user";

    // Private order endpoints
    /// Get all open orders
    pub const GET_OPEN_ORDERS: &str = "/private/get_open_orders";
    /// Get open orders by label
    pub const GET_OPEN_ORDERS_BY_LABEL: &str = "/private/get_open_orders_by_label";
    /// Get order state
    pub const GET_ORDER_STATE: &str = "/private/get_order_state";
    /// Get open orders by currency
    pub const GET_OPEN_ORDERS_BY_CURRENCY: &str = "/private/get_open_orders_by_currency";
    /// Get open orders by instrument
    pub const GET_OPEN_ORDERS_BY_INSTRUMENT: &str = "/private/get_open_orders_by_instrument";
    /// Get order history by currency
    pub const GET_ORDER_HISTORY_BY_CURRENCY: &str = "/private/get_order_history_by_currency";
    /// Get order history by instrument
    pub const GET_ORDER_HISTORY_BY_INSTRUMENT: &str = "/private/get_order_history_by_instrument";

    // Private trade endpoints
    /// Get user trades by instrument
    pub const GET_USER_TRADES_BY_INSTRUMENT: &str = "/private/get_user_trades_by_instrument";
    /// Get user trades by currency
    pub const GET_USER_TRADES_BY_CURRENCY: &str = "/private/get_user_trades_by_currency";
    /// Get user trades by currency and time
    pub const GET_USER_TRADES_BY_CURRENCY_AND_TIME: &str =
        "/private/get_user_trades_by_currency_and_time";
    /// Get user trades by instrument and time
    pub const GET_USER_TRADES_BY_INSTRUMENT_AND_TIME: &str =
        "/private/get_user_trades_by_instrument_and_time";
    /// Get user trades by order
    pub const GET_USER_TRADES_BY_ORDER: &str = "/private/get_user_trades_by_order";
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
