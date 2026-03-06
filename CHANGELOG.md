# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1] - 2026-01-10

### Changed
- Enable `serde` feature for `url` crate

## [0.5.0] - 2026-01-05

### Added
- WebAssembly (wasm32) target support
- WASM-compatible async primitives

### Changed
- Update edition to 2024
- Fix `get_user_trades_by_currency` to pass all TradesRequest parameters
- Update `CurrencyStruct` with additional fields (`fee_precision` optional, `decimals` added)
- Refactor enums to use `#[derive(Default)]` instead of manual impl

### Fixed
- Pass all TradesRequest parameters in `get_user_trades_by_currency`

## [0.4.1] - 2025-09-27

### Added
- Comprehensive option data structures, methods, and utilities

## [0.4.0] - 2025-09-22

### Added
- `option` module with `OptionInfo` struct and parsing logic
- `from_deribit_format_date` utility function for date parsing
- `get_position` method for retrieving positions by instrument
- `GET_POSITION` constant

### Changed
- Update `Subaccount` and related structs with new fields
- Implement `Portfolio` methods for initialization

## [0.3.2] - 2025-09-16

### Changed
- Version bump with minor fixes

## [0.3.1] - 2025-09-16

### Changed
- Update edition to 2024
- Refactor test modules to unify naming convention

## [0.3.0] - 2025-09-15

### Added
- Comprehensive unit tests for models (`Trade`, `Ticker`, `book`, `instrument`, `funding`, `currency`)
- Unit tests for private and public endpoints
- Custom configuration option for HTTP client
- User trades request/response models with pagination
- Stop-limit order support with `trigger` and `trigger_price` parameters
- `FeeStructure` model
- `UserTradeResponse` and `TradeAllocation` structs

### Changed
- Replace environment-based tests with mock server integration
- Transition integration tests to use real Deribit testnet
- Centralize URL constants
- Make `average_price` and `filled_amount` optional in `OrderResponse`
- Extensive refactoring for code quality and documentation

### Fixed
- `get_token` logic to correctly return token when not expired

## [0.2.1] - 2025-09-10

### Changed
- Update `deribit-base` dependency to 0.2.6

## [0.2.0] - 2025-09-10

### Changed
- Refactor configuration and authentication logic
- Replace `utils` module with enhanced `credentials` module
- Improve token management with validity checks

## [0.1.3] - 2025-09-09

### Changed
- Update `deribit-base` dependency to 0.2.4

## [0.1.2] - 2025-09-09

### Added
- Contact section in README (Telegram, repository link, documentation)

### Changed
- Update `deribit-base` dependency to 0.2.1

## [0.1.1] - 2025-08-19

### Added
- Initial HTTP REST API client for Deribit
- Public endpoints (currencies, index, instruments, order book, trades, ticker)
- Private endpoints (account, orders, positions, trades)
- OAuth2 authentication flow
- Rate limiting support
- Comprehensive error handling

[Unreleased]: https://github.com/joaquinbejar/deribit-http/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/joaquinbejar/deribit-http/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/joaquinbejar/deribit-http/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/joaquinbejar/deribit-http/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/joaquinbejar/deribit-http/compare/v0.3.1...v0.4.0
[0.3.2]: https://github.com/joaquinbejar/deribit-http/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/joaquinbejar/deribit-http/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/joaquinbejar/deribit-http/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/joaquinbejar/deribit-http/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/joaquinbejar/deribit-http/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/joaquinbejar/deribit-http/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/joaquinbejar/deribit-http/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/joaquinbejar/deribit-http/releases/tag/v0.1.1
