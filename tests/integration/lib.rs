/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/9/25
******************************************************************************/

//! Integration tests library for deribit-http
//!
//! This library provides common utilities and test modules for integration testing
//! of the Deribit HTTP client functionality.

pub mod account_management;
pub mod auth;
pub mod connectivity;
pub mod error_handling;
pub mod market_data;
pub mod order_management;
pub mod position_management;
pub mod session;
pub mod wallet;

// Re-export commonly used types and functions for tests
pub use assert_json_diff;
pub use deribit_base::error;
pub use deribit_base::model;
pub use deribit_http::client::DeribitHttpClient;
pub use deribit_http::config::HttpConfig;
pub use mockito;
pub use pretty_assertions;
pub use serde_json;
pub use serial_test;
pub use tokio;
