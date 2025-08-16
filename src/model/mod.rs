//! Model definitions for HTTP client

// Re-export common models from deribit-base
pub use deribit_base::model::*;

// HTTP-specific models
pub mod http_types;

pub use http_types::*;
