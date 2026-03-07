//! # Deribit HTTP Client (deribit_http)
//!
//! **Production-ready** asynchronous HTTP client for the Deribit API v2.
//! Version **0.6.0** provides **~95% coverage** of all HTTP-compatible Deribit endpoints.
//!
//! Designed for server integrations, batch jobs, and tooling that prefer REST/HTTP over WebSocket.
//! Built on `reqwest` and `tokio`, with full WASM/Cloudflare Workers support.
//!
//! ## Key features
//! - **100+ endpoints** implemented across public and private APIs
//! - **Pure async HTTP** with reqwest + tokio (native) or fetch (WASM)
//! - **Cross-platform**: Native, WASM browsers, and Cloudflare Workers
//! - **OAuth2 authentication** with automatic token renewal
//! - **Token-bucket rate limiting** per endpoint category
//! - **40+ strongly-typed models** with Serde serialization
//! - **126 unit tests** ensuring reliability
//!
//! ## Installation
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! deribit-http = "0.6"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ## Quick start
//! ```rust
//! use deribit_http::DeribitHttpClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // true = testnet, false = mainnet
//!     let client = DeribitHttpClient::new();
//!
//!     // Public calls (no authentication required)
//!     let currencies = client.get_currencies().await?;
//!     println!("Supports {} currencies", currencies.len());
//!
//!     // Example: ticker
//!     let ticker = client.get_ticker("BTC-PERPETUAL").await?;
//!     println!("Mark price: {}", ticker.mark_price);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication and private endpoints
//! - OAuth2 (Client Credentials): `DeribitHttpClient::authenticate_oauth2(client_id, client_secret)` returns an `AuthToken` and keeps it in the `AuthManager`.
//! - Helpers: `is_authenticated()`, `get_auth_token()`.
//! - Session management: `exchange_token(refresh_token, subject_id, scope)` and `fork_token(refresh_token, session_name, scope)`.
//! - API Key: the `authenticate_api_key` method exists but is currently not implemented and will return an error.
//!
//! ## Configuration
//! - Environment shortcut: `DeribitHttpClient::new()` for Testnet and `new(false)` for Production.
//! - Custom configuration: `DeribitHttpClient::with_config(HttpConfig)` lets you set `base_url`, `timeout`, `user_agent`, `testnet`, and optional credentials.
//! - Validation: configuration is validated on client creation.
//!
//! ## Project structure (modules)
//! - `auth`: `AuthManager` (OAuth2, token management) and related types (e.g. `AuthRequest`).
//! - `client`: `DeribitHttpClient`, public/private methods, auth helpers, `exchange_token` and `fork_token`.
//! - `config`: `HttpConfig` and environment helpers (testnet/production) and headers/base_url.
//! - `connection` and `session`: infrastructure support types (shared across the ecosystem).
//! - `endpoints`: HTTP implementation of public and private methods (see coverage below).
//! - `error`: `HttpError` variants such as `NetworkError`, `RequestFailed`, `InvalidResponse`, `AuthenticationFailed`, `ConfigError`.
//! - `message` and `model`: HTTP types (`ApiResponse`, `ApiError`, `AuthToken`, etc.).
//! - `rate_limit`: `RateLimiter` and `categorize_endpoint` with per-category limits.
//! - `constants`: base URLs (production/testnet), endpoint routes, and common headers.
//!
//! ## Public endpoints (30+)
//!
//! | Category | Endpoints |
//! |----------|-----------|
//! | **System** | `get_server_time()`, `test_connection()`, `get_status()` |
//! | **Currencies** | `get_currencies()`, `get_apr_history()` |
//! | **Indices** | `get_index()`, `get_index_price()`, `get_index_price_names()`, `get_index_chart_data()` |
//! | **Instruments** | `get_instrument()`, `get_instruments()`, `get_contract_size()` |
//! | **Book Summary** | `get_book_summary_by_currency()`, `get_book_summary_by_instrument()` |
//! | **Market Data** | `get_ticker()`, `get_order_book()`, `get_order_book_by_instrument_id()` |
//! | **Trades** | `get_last_trades()`, `get_last_trades_by_currency()`, `get_last_trades_by_*_and_time()` |
//! | **Funding** | `get_funding_chart_data()`, `get_funding_rate_history()`, `get_funding_rate_value()` |
//! | **Volatility** | `get_historical_volatility()`, `get_volatility_index_data()` |
//! | **Settlements** | `get_last_settlements_by_currency()`, `get_last_settlements_by_instrument()` |
//! | **TradingView** | `get_tradingview_chart_data()` |
//! | **Combo Books** | `get_combo_details()`, `get_combo_ids()`, `get_combos()` |
//! | **Block RFQ** | `get_block_rfq_trades()` |
//!
//! ## Private endpoints (70+)
//!
//! Require valid authentication (OAuth2):
//!
//! | Category | Endpoints |
//! |----------|-----------|
//! | **Trading** | `buy_order()`, `sell_order()`, `edit_order()`, `cancel_order()`, `cancel_all()`, `cancel_all_by_*()` |
//! | **Orders** | `get_open_orders()`, `get_order_state()`, `get_order_history_by_currency()`, `get_order_history_by_instrument()` |
//! | **Positions** | `get_position()`, `get_positions()`, `close_position()`, `move_positions()` |
//! | **User Trades** | `get_user_trades_by_instrument()`, `get_user_trades_by_currency()`, `get_user_trades_by_order()` |
//! | **Account** | `get_account_summary()`, `get_account_summaries()`, `get_subaccounts()`, `get_subaccounts_details()` |
//! | **Subaccounts** | `create_subaccount()`, `change_subaccount_name()`, `toggle_subaccount_login()`, `remove_subaccount()` |
//! | **API Keys** | `create_api_key()`, `edit_api_key()`, `remove_api_key()`, `list_api_keys()`, `enable_api_key()`, `disable_api_key()` |
//! | **Wallet** | `get_deposits()`, `get_withdrawals()`, `withdraw()`, `cancel_withdrawal()`, `create_deposit_address()` |
//! | **Transfers** | `get_transfers()`, `submit_transfer_to_subaccount()`, `submit_transfer_between_subaccounts()`, `cancel_transfer_by_id()` |
//! | **Block Trade** | `execute_block_trade()`, `verify_block_trade()`, `get_block_trade()`, `get_block_trades()`, `simulate_block_trade()` |
//! | **Block RFQ** | `create_block_rfq()`, `accept_block_rfq()`, `add_block_rfq_quote()`, `cancel_block_rfq()`, `get_block_rfqs()` |
//! | **Combo Books** | `create_combo()`, `get_leg_prices()` |
//! | **MMP** | `get_mmp_config()`, `set_mmp_config()`, `reset_mmp()`, `get_mmp_status()` |
//! | **Mass Quote** | `mass_quote()`, `cancel_quotes()` |
//! | **Margins** | `get_margins()`, `get_order_margin_by_ids()` |
//! | **Settlement** | `get_settlement_history_by_currency()`, `get_settlement_history_by_instrument()` |
//!
//! ## Limitations and important notes
//! - This crate does not implement WebSocket or streaming. Some Deribit endpoints exist only over WS
//!   (for example, `/public/hello` and `/private/logout`) and are not available in this HTTP client.
//! - API Key authentication: the `authenticate_api_key` stub exists but is not yet implemented in the HTTP client.
//! - Deribit uses JSON-RPC over HTTP; this client exposes ergonomic methods that build URLs with query params
//!   and parse `ApiResponse<T>` in a strongly-typed manner.
//!
//! ## Error handling
//! The `HttpError` type centralizes common failures: network issues (`NetworkError`),
//! non-success HTTP responses (`RequestFailed`), parsing/structure errors (`InvalidResponse`),
//! authentication failures (`AuthenticationFailed`), and configuration conditions (`ConfigError`).
//!
//! ## Rate limiting
//! The `RateLimiter` categorizes each URL and applies a token-bucket scheme per category
//! (Trading, MarketData, Account, Auth, General). You can inspect it via `rate_limiter()`.
//!
//! ## Examples
//!
//! See the `examples/` directory for comprehensive examples:
//! - **Public examples** (16 binaries): Market data, instruments, trades, funding, settlements
//! - **Private examples** (7 binaries): Trading, orders, positions, account management, mass quotes
//!
//! ```bash
//! # Run a public example
//! cargo run --bin check_currencies
//!
//! # Run a private example (requires .env with credentials)
//! cargo run --bin trading_endpoints
//! ```
//!
//! ## Platform support
//!
//! | Target | Status |
//! |--------|--------|
//! | Native (tokio) | ✅ Full support |
//! | WASM (browser) | ✅ Full support |
//! | Cloudflare Workers | ✅ Full support |

pub mod auth;
pub mod client;
pub mod config;
pub mod connection;
/// HTTP API endpoints implementation for public and private Deribit API methods
pub mod endpoints;
pub mod error;
pub mod message;
pub mod model;
pub mod prelude;
pub mod rate_limit;
pub mod session;
/// Cross-platform async sleep for native and WASM targets
pub mod sleep_compat;
/// Cross-platform Mutex re-export for native and WASM targets
pub mod sync_compat;
/// Cross-platform time utilities for native and WASM targets
pub mod time_compat;

// Constants
/// Application constants and configuration
pub mod constants;
/// Logging utilities and configuration
pub mod logger;
/// Utility functions and helpers
pub mod utils;

// Re-export main client and error types
pub use client::*;
pub use error::*;

// Re-export specific types to avoid conflicts
pub use auth::AuthRequest;
pub use auth::{ApiKeyAuth, AuthManager};
pub use config::ApiCredentials;
pub use config::HttpConfig;
pub use connection::*;
pub use message::{HttpMessageBuilder, HttpRequestBuilder, HttpResponseHandler};
pub use session::*;
