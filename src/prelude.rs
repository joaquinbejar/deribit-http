//! Prelude module for deribit-http
//!
//! This module re-exports commonly used types and traits for convenience.
//! Import everything with a single `use deribit_http::prelude::*;` statement.

// Re-export main client
pub use crate::client::DeribitHttpClient;

// Re-export configuration types
pub use crate::config::{ApiCredentials, HttpConfig};

// Re-export error types
pub use crate::error::HttpError;

// Re-export authentication types
pub use crate::auth::{ApiKeyAuth, AuthManager, AuthRequest};

// Re-export message types
pub use crate::message::{HttpMessageBuilder, HttpRequestBuilder, HttpResponseHandler};

// Re-export session types
pub use crate::session::HttpSession;

// Re-export rate limiting types
pub use crate::rate_limit::{RateLimitCategory, RateLimiter, categorize_endpoint};

// Re-export constants
pub use crate::constants::{DEFAULT_TIMEOUT, MAX_RETRIES, PRODUCTION_BASE_URL, TESTNET_BASE_URL};

// Re-export logging utilities
pub use crate::logger::setup_logger;

// Re-export connection types
pub use crate::connection::*;

// Re-export all model types
pub use crate::model::*;

// Re-export utility functions
pub use crate::utils::*;

// Re-export commonly used external types
pub use serde_json::{Value, json};
