//! # Deribit HTTP Client
//!
//! This crate provides a HTTP REST API client for the Deribit trading platform.
//! It implements the common traits from `deribit-base` and provides methods
//! for all REST API endpoints.

pub mod auth;
pub mod client;
pub mod endpoints;

pub use auth::*;
pub use client::*;

// Re-export common types from deribit-base
pub use deribit_base::*;
