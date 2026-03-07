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
    /// Get mark price history
    pub const GET_MARK_PRICE_HISTORY: &str = "/public/get_mark_price_history";
    /// Get supported index names
    pub const GET_SUPPORTED_INDEX_NAMES: &str = "/public/get_supported_index_names";
    /// Get trade volumes
    pub const GET_TRADE_VOLUMES: &str = "/public/get_trade_volumes";
    /// Get volatility index data
    pub const GET_VOLATILITY_INDEX_DATA: &str = "/public/get_volatility_index_data";
    /// Get index chart data
    pub const GET_INDEX_CHART_DATA: &str = "/public/get_index_chart_data";

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
    /// Edit order by label
    pub const EDIT_BY_LABEL: &str = "/private/edit_by_label";
    /// Cancel quotes
    pub const CANCEL_QUOTES: &str = "/private/cancel_quotes";
    /// Close an existing position
    pub const CLOSE_POSITION: &str = "/private/close_position";
    /// Get margin requirements
    pub const GET_MARGINS: &str = "/private/get_margins";
    /// Get MMP configuration
    pub const GET_MMP_CONFIG: &str = "/private/get_mmp_config";
    /// Get MMP status
    pub const GET_MMP_STATUS: &str = "/private/get_mmp_status";
    /// Set MMP configuration
    pub const SET_MMP_CONFIG: &str = "/private/set_mmp_config";
    /// Reset MMP limits
    pub const RESET_MMP: &str = "/private/reset_mmp";
    /// Get order margin by IDs
    pub const GET_ORDER_MARGIN_BY_IDS: &str = "/private/get_order_margin_by_ids";
    /// Get order state by label
    pub const GET_ORDER_STATE_BY_LABEL: &str = "/private/get_order_state_by_label";
    /// Get settlement history by currency
    pub const GET_SETTLEMENT_HISTORY_BY_CURRENCY: &str =
        "/private/get_settlement_history_by_currency";
    /// Get settlement history by instrument
    pub const GET_SETTLEMENT_HISTORY_BY_INSTRUMENT: &str =
        "/private/get_settlement_history_by_instrument";
    /// Get trigger order history
    pub const GET_TRIGGER_ORDER_HISTORY: &str = "/private/get_trigger_order_history";

    // Private account endpoints
    /// Get account summary information
    pub const GET_ACCOUNT_SUMMARY: &str = "/private/get_account_summary";
    /// Get account summaries for all currencies
    pub const GET_ACCOUNT_SUMMARIES: &str = "/private/get_account_summaries";
    /// Get position
    pub const GET_POSITION: &str = "/private/get_position";
    /// Get current positions
    pub const GET_POSITIONS: &str = "/private/get_positions";
    /// Get subaccount information
    pub const GET_SUBACCOUNTS: &str = "/private/get_subaccounts";
    /// Get subaccounts details with positions
    pub const GET_SUBACCOUNTS_DETAILS: &str = "/private/get_subaccounts_details";
    /// Create a new subaccount
    pub const CREATE_SUBACCOUNT: &str = "/private/create_subaccount";
    /// Remove an empty subaccount
    pub const REMOVE_SUBACCOUNT: &str = "/private/remove_subaccount";
    /// Change the name of a subaccount
    pub const CHANGE_SUBACCOUNT_NAME: &str = "/private/change_subaccount_name";
    /// Enable or disable login for a subaccount
    pub const TOGGLE_SUBACCOUNT_LOGIN: &str = "/private/toggle_subaccount_login";
    /// Set email address for a subaccount
    pub const SET_EMAIL_FOR_SUBACCOUNT: &str = "/private/set_email_for_subaccount";
    /// Enable or disable notifications for a subaccount
    pub const TOGGLE_NOTIFICATIONS_FROM_SUBACCOUNT: &str =
        "/private/toggle_notifications_from_subaccount";
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
    /// Get transfers list
    pub const GET_TRANSFERS: &str = "/private/get_transfers";
    /// Cancel a transfer by ID
    pub const CANCEL_TRANSFER_BY_ID: &str = "/private/cancel_transfer_by_id";
    /// Submit transfer between subaccounts
    pub const SUBMIT_TRANSFER_BETWEEN_SUBACCOUNTS: &str =
        "/private/submit_transfer_between_subaccounts";
    /// Move positions between subaccounts
    pub const MOVE_POSITIONS: &str = "/private/move_positions";

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

    // API Key Management endpoints
    /// Create a new API key
    pub const CREATE_API_KEY: &str = "/private/create_api_key";
    /// Edit an existing API key
    pub const EDIT_API_KEY: &str = "/private/edit_api_key";
    /// Disable an API key
    pub const DISABLE_API_KEY: &str = "/private/disable_api_key";
    /// Enable an API key
    pub const ENABLE_API_KEY: &str = "/private/enable_api_key";
    /// List all API keys
    pub const LIST_API_KEYS: &str = "/private/list_api_keys";
    /// Remove an API key
    pub const REMOVE_API_KEY: &str = "/private/remove_api_key";
    /// Reset an API key secret
    pub const RESET_API_KEY: &str = "/private/reset_api_key";
    /// Change API key name
    pub const CHANGE_API_KEY_NAME: &str = "/private/change_api_key_name";
    /// Change API key scope
    pub const CHANGE_SCOPE_IN_API_KEY: &str = "/private/change_scope_in_api_key";

    // Address Beneficiary endpoints
    /// Save address beneficiary information
    pub const SAVE_ADDRESS_BENEFICIARY: &str = "/private/save_address_beneficiary";
    /// Delete address beneficiary information
    pub const DELETE_ADDRESS_BENEFICIARY: &str = "/private/delete_address_beneficiary";
    /// Get address beneficiary information
    pub const GET_ADDRESS_BENEFICIARY: &str = "/private/get_address_beneficiary";
    /// List address beneficiaries with pagination
    pub const LIST_ADDRESS_BENEFICIARIES: &str = "/private/list_address_beneficiaries";
    /// Set clearance originator for a deposit
    pub const SET_CLEARANCE_ORIGINATOR: &str = "/private/set_clearance_originator";

    // Wallet endpoints
    /// Create a new withdrawal request
    pub const WITHDRAW: &str = "/private/withdraw";
    /// Cancel a pending withdrawal
    pub const CANCEL_WITHDRAWAL: &str = "/private/cancel_withdrawal";
    /// Create a new deposit address
    pub const CREATE_DEPOSIT_ADDRESS: &str = "/private/create_deposit_address";
    /// Get the current deposit address
    pub const GET_CURRENT_DEPOSIT_ADDRESS: &str = "/private/get_current_deposit_address";
    /// Add an address to the address book
    pub const ADD_TO_ADDRESS_BOOK: &str = "/private/add_to_address_book";
    /// Remove an address from the address book
    pub const REMOVE_FROM_ADDRESS_BOOK: &str = "/private/remove_from_address_book";
    /// Update an address in the address book
    pub const UPDATE_IN_ADDRESS_BOOK: &str = "/private/update_in_address_book";
    /// Get addresses from the address book
    pub const GET_ADDRESS_BOOK: &str = "/private/get_address_book";

    // Remaining account endpoints
    /// Get account access log
    pub const GET_ACCESS_LOG: &str = "/private/get_access_log";
    /// Get user account locks
    pub const GET_USER_LOCKS: &str = "/private/get_user_locks";
    /// List custody accounts
    pub const LIST_CUSTODY_ACCOUNTS: &str = "/private/list_custody_accounts";
    /// Simulate portfolio margin
    pub const SIMULATE_PORTFOLIO: &str = "/private/simulate_portfolio";
    /// PME margin simulation
    pub const PME_SIMULATE: &str = "/private/pme/simulate";
    /// Change margin model
    pub const CHANGE_MARGIN_MODEL: &str = "/private/change_margin_model";
    /// Set self-trading configuration
    pub const SET_SELF_TRADING_CONFIG: &str = "/private/set_self_trading_config";
    /// Set disabled trading products
    pub const SET_DISABLED_TRADING_PRODUCTS: &str = "/private/set_disabled_trading_products";
    /// Get public announcements
    pub const GET_ANNOUNCEMENTS: &str = "/public/get_announcements";
    /// Get new (unread) announcements
    pub const GET_NEW_ANNOUNCEMENTS: &str = "/private/get_new_announcements";
    /// Mark announcement as read
    pub const SET_ANNOUNCEMENT_AS_READ: &str = "/private/set_announcement_as_read";
    /// Enable affiliate program
    pub const ENABLE_AFFILIATE_PROGRAM: &str = "/private/enable_affiliate_program";
    /// Get affiliate program information
    pub const GET_AFFILIATE_PROGRAM_INFO: &str = "/private/get_affiliate_program_info";
    /// Set email language preference
    pub const SET_EMAIL_LANGUAGE: &str = "/private/set_email_language";
    /// Get email language preference
    pub const GET_EMAIL_LANGUAGE: &str = "/private/get_email_language";

    // Block Trade endpoints
    /// Approve a pending block trade
    pub const APPROVE_BLOCK_TRADE: &str = "/private/approve_block_trade";
    /// Execute a block trade with counterparty signature
    pub const EXECUTE_BLOCK_TRADE: &str = "/private/execute_block_trade";
    /// Get a specific block trade by ID
    pub const GET_BLOCK_TRADE: &str = "/private/get_block_trade";
    /// Get pending block trade requests
    pub const GET_BLOCK_TRADE_REQUESTS: &str = "/private/get_block_trade_requests";
    /// List block trades with optional filters
    pub const GET_BLOCK_TRADES: &str = "/private/get_block_trades";
    /// Get broker trade requests
    pub const GET_BROKER_TRADE_REQUESTS: &str = "/private/get_broker_trade_requests";
    /// List broker trades
    pub const GET_BROKER_TRADES: &str = "/private/get_broker_trades";
    /// Invalidate a block trade signature
    pub const INVALIDATE_BLOCK_TRADE_SIGNATURE: &str = "/private/invalidate_block_trade_signature";
    /// Reject a pending block trade
    pub const REJECT_BLOCK_TRADE: &str = "/private/reject_block_trade";
    /// Simulate if a block trade can be executed
    pub const SIMULATE_BLOCK_TRADE: &str = "/private/simulate_block_trade";
    /// Verify and create a block trade signature
    pub const VERIFY_BLOCK_TRADE: &str = "/private/verify_block_trade";
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
