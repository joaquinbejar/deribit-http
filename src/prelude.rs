//! Prelude module for deribit-http
//!
//! This module re-exports commonly used types and traits for convenience.

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

pub use crate::logger::setup_logger;

pub use crate::connection::*;
pub use crate::model::*;
pub use crate::utils::*;
