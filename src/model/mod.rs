//! Model definitions for HTTP client

// HTTP-specific models
/// Access log models for account history
pub mod access_log;
/// Account-related models and structures
pub mod account;
/// Affiliate program models
pub mod affiliate;
/// Announcement models
pub mod announcement;
/// API key management models
pub mod api_key;
/// Address beneficiary models for wallet endpoints
pub mod beneficiary;
/// Block trade models
pub mod block_trade;
/// Order book models
pub mod book;
/// Currency and expiration models
pub mod currency;
/// Custody account models
pub mod custody;
/// Deposit-related models
pub mod deposit;
/// Email settings models
pub mod email_settings;
/// Fee calculation and structure models
pub mod fee;
/// Funding rate models
pub mod funding;
/// Index price models
pub mod index;
/// Instrument definition models
pub mod instrument;
/// Margin model configuration
pub mod margin_model;
/// Mass quote models
pub mod mass_quote;
/// Option contract models and types
pub mod option;
/// Order models and types
pub mod order;
/// Other miscellaneous models
pub mod other;
/// Portfolio simulation models
pub mod portfolio_simulation;
/// Position models
pub mod position;
/// Request models and structures
pub mod request;
/// Response models and structures
pub mod response;
/// Self-trading configuration models
pub mod self_trading;
/// Settlement models
pub mod settlement;
/// Ticker data models
pub mod ticker;
/// Trade execution models
pub mod trade;
/// Trading products configuration
pub mod trading_products;
/// TradingView chart models
pub mod tradingview;
/// Transaction log models
pub mod transaction;
/// Transfer models
pub mod transfer;
/// Trigger models
pub mod trigger;
/// Common type definitions
pub mod types;
/// User lock models
pub mod user_lock;
/// Wallet models for deposit addresses and address book
pub mod wallet;
/// Withdrawal models
pub mod withdrawal;

pub use access_log::*;
pub use account::*;
pub use affiliate::*;
pub use announcement::*;
pub use api_key::*;
pub use beneficiary::*;
pub use block_trade::*;
pub use book::*;
pub use currency::*;
pub use custody::*;
pub use deposit::*;
pub use email_settings::*;
pub use fee::*;
pub use funding::*;
pub use index::*;
pub use instrument::*;
pub use margin_model::*;
pub use mass_quote::*;
pub use option::*;
pub use order::*;
pub use other::*;
pub use portfolio_simulation::*;
pub use position::*;
pub use request::*;
pub use response::*;
pub use self_trading::*;
pub use settlement::*;
pub use ticker::*;
pub use trade::*;
pub use trading_products::*;
pub use tradingview::*;
pub use transaction::*;
pub use transfer::*;
pub use trigger::*;
pub use types::*;
pub use user_lock::*;
pub use wallet::*;
pub use withdrawal::*;
