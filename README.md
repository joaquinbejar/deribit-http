<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/joaquinbejar/deribit-http/refs/heads/main/doc/images/logo.png" alt="deribit-http" style="width: 80%; height: 80%;">
</div>

[![Dual License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/deribit-http.svg)](https://crates.io/crates/deribit-http)
[![Downloads](https://img.shields.io/crates/d/deribit-http.svg)](https://crates.io/crates/deribit-http)
[![Stars](https://img.shields.io/github/stars/joaquinbejar/deribit-http.svg)](https://github.com/joaquinbejar/deribit-http/stargazers)
[![Issues](https://img.shields.io/github/issues/joaquinbejar/deribit-http.svg)](https://github.com/joaquinbejar/deribit-http/issues)
[![PRs](https://img.shields.io/github/issues-pr/joaquinbejar/deribit-http.svg)](https://github.com/joaquinbejar/deribit-http/pulls)
[![Build Status](https://img.shields.io/github/workflow/status/joaquinbejar/deribit-http/CI)](https://github.com/joaquinbejar/deribit-http/actions)
[![Coverage](https://img.shields.io/codecov/c/github/joaquinbejar/deribit-http)](https://codecov.io/gh/joaquinbejar/deribit-http)
[![Dependencies](https://img.shields.io/librariesio/github/joaquinbejar/deribit-http)](https://libraries.io/github/joaquinbejar/deribit-http)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/deribit-http)
[![Wiki](https://img.shields.io/badge/wiki-latest-blue.svg)](https://deepwiki.com/joaquinbejar/deribit-http)

## Deribit HTTP Client (deribit_http)

Asynchronous HTTP client for the Deribit API, designed for server integrations,
batch jobs and tooling that prefer REST/HTTP over WebSocket. Built on top of
`reqwest` and `tokio`, it provides a typed set of methods for public and
private endpoints, OAuth2 authentication, category-based rate limiting and
Serde-powered data models.

This crate-level documentation is intended to be used by `cargo readme` to generate the README.

### Key features
- Pure async HTTP (reqwest + tokio).
- Simple Testnet/Mainnet setup: `DeribitHttpClient::new()` or `default`.
- Built-in OAuth2 (Client Credentials); utilities for `exchange_token` and `fork_token`.
- Category-based rate limiting (trading, market, account, auth, general) with a token-bucket approach.
- Strongly-typed data models (Serde) and JSON-RPC responses mapped to `ApiResponse`/`ApiError`.
- Re-exported types and structures for ergonomic usage.
- Explicit focus on Deribit REST public/private endpoints (no WebSocket/streaming in this crate).

### Quick start
```rust
use deribit_http::DeribitHttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // true = testnet, false = mainnet
    let client = DeribitHttpClient::new();

    // Public calls (no authentication required)
    let currencies = client.get_currencies().await?;
    println!("Supports {} currencies", currencies.len());

    // Example: ticker
    let ticker = client.get_ticker("BTC-PERPETUAL").await?;
    println!("Mark price: {}", ticker.mark_price);

    Ok(())
}
```

### Authentication and private endpoints
- OAuth2 (Client Credentials): `DeribitHttpClient::authenticate_oauth2(client_id, client_secret)` returns an `AuthToken` and keeps it in the `AuthManager`.
- Helpers: `is_authenticated()`, `get_auth_token()`.
- Session management: `exchange_token(refresh_token, subject_id, scope)` and `fork_token(refresh_token, session_name, scope)`.
- API Key: the `authenticate_api_key` method exists but is currently not implemented and will return an error.

### Configuration
- Environment shortcut: `DeribitHttpClient::new()` for Testnet and `new(false)` for Production.
- Custom configuration: `DeribitHttpClient::with_config(HttpConfig)` lets you set `base_url`, `timeout`, `user_agent`, `testnet`, and optional credentials.
- Validation: configuration is validated on client creation.

### Project structure (modules)
- `auth`: `AuthManager` (OAuth2, token management) and related types (e.g. `AuthRequest`).
- `client`: `DeribitHttpClient`, public/private methods, auth helpers, `exchange_token` and `fork_token`.
- `config`: `HttpConfig` and environment helpers (testnet/production) and headers/base_url.
- `connection` and `session`: infrastructure support types (shared across the ecosystem).
- `endpoints`: HTTP implementation of public and private methods (see coverage below).
- `error`: `HttpError` variants such as `NetworkError`, `RequestFailed`, `InvalidResponse`, `AuthenticationFailed`, `ConfigError`.
- `message` and `model`: HTTP types (`ApiResponse`, `ApiError`, `AuthToken`, etc.).
- `rate_limit`: `RateLimiter` and `categorize_endpoint` with per-category limits.
- `constants`: base URLs (production/testnet), endpoint routes, and common headers.

### Implemented public endpoints (selection)
The following methods exist on `DeribitHttpClient` within `endpoints::public`:
- Currencies and markets: `get_currencies()`.
- Indices and prices: `get_index(currency)`, `get_index_price(index_name)`, `get_index_price_names()`.
- Book summary: `get_book_summary_by_currency(currency, kind)`, `get_book_summary_by_instrument(instrument_name)`.
- Instruments: `get_instrument(instrument_name)`, `get_instruments(currency, kind, expired)`, `get_contract_size(instrument_name)`.
- System and status: `get_server_time()`, `test_connection()`, `get_status()`.
- Market data: `get_ticker(instrument_name)`, `get_order_book(instrument_name, depth)`, `get_order_book_by_instrument_id(instrument_id, depth)`.
- Trades: `get_last_trades(instrument_name, ...)`, `get_last_trades_by_currency(...)`,
  `get_last_trades_by_currency_and_time(...)`, `get_last_trades_by_instrument_and_time(...)`.
- Volatility and interest: `get_historical_volatility(currency)`, `get_apr_history(currency, ...)`.
- Funding: `get_funding_chart_data(instrument_name, length)`, `get_funding_rate_history(instrument_name, start, end)`, `get_funding_rate_value(instrument_name, start, end)`.
- TradingView: `get_tradingview_chart_data(instrument_name, start, end, resolution)`.
- Other: `get_delivery_prices(index_name, ...)`, `get_expirations(currency, kind, currency_pair)`.
- Note: `hello(client_name, client_version)` is documented but WebSocket-only; in HTTP it will return a configuration error.

Included models (re-exported): `Currency`, `IndexData`, `BookSummary`, `Instrument`, `Trade`,
`TickerData`, `OrderBook`, `FundingChartData`, `TradingViewChartData`, `DeliveryPricesResponse`,
`ExpirationsResponse`, `FundingRateData`, `SettlementsResponse`, `LastTradesResponse`, etc.

### Implemented private endpoints (selection)
Require valid authentication (OAuth2). Examples include:
- Accounts and movements: `get_subaccounts()`, `get_transaction_log(...)`, `get_deposits(...)`, `get_withdrawals(...)`.
- Basic trading: `buy_order(...)`, `sell_order(...)`, `cancel_order(order_id)`.
- Account: `get_account_summary(currency, ...)`.

Useful re-exported models: `Subaccount`, `TransactionLog`, `DepositsResponse`, `WithdrawalsResponse`,
`OrderResponse`, `OrderInfo`, `AccountSummary`, `Position`, `PortfolioInfo`, etc.

### Limitations and important notes
- This crate does not implement WebSocket or streaming. Some Deribit endpoints exist only over WS
  (for example, `/public/hello` and `/private/logout`) and are not available in this HTTP client.
- API Key authentication: the `authenticate_api_key` stub exists but is not yet implemented in the HTTP client.
- Deribit uses JSON-RPC over HTTP; this client exposes ergonomic methods that build URLs with query params
  and parse `ApiResponse<T>` in a strongly-typed manner.

### Error handling
The `HttpError` type centralizes common failures: network issues (`NetworkError`),
non-success HTTP responses (`RequestFailed`), parsing/structure errors (`InvalidResponse`),
authentication failures (`AuthenticationFailed`), and configuration conditions (`ConfigError`).

### Rate limiting
The `RateLimiter` categorizes each URL and applies a token-bucket scheme per category
(Trading, MarketData, Account, Auth, General). You can inspect it via `rate_limiter()`.

### README generation
This crate-level header is consumed by `cargo readme`. To generate the README:
```bash
cargo readme -o README.md
```
Ensure you have `cargo-readme` installed (`cargo install cargo-readme`).

## Contribution and Contact

We welcome contributions to this project! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the project still builds and all tests pass.
4. Commit your changes and push your branch to your forked repository.
5. Submit a pull request to the main repository.

If you have any questions, issues, or would like to provide feedback, please feel free to contact the project maintainer:

### **Contact Information**
- **Author**: Joaquín Béjar García
- **Email**: jb@taunais.com
- **Telegram**: [@joaquin_bejar](https://t.me/joaquin_bejar)
- **Repository**: <https://github.com/joaquinbejar/deribit-http>
- **Documentation**: <https://docs.rs/deribit-http>

We appreciate your interest and look forward to your contributions!

## ✍️ License

Licensed under MIT license

## Disclaimer

This software is not officially associated with Deribit. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a demo account before using in a live trading environment.
