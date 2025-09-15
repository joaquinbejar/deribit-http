/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/9/25
******************************************************************************/

//! Integration tests for the Deribit HTTP client
//!
//! This module contains comprehensive integration tests for all aspects of the Deribit HTTP client,
//! including authentication, API endpoints, error handling, and network resilience.

pub mod account_management;
pub mod auth;
pub mod connectivity;
pub mod debug_response;
pub mod error_handling;
pub mod market_data;
pub mod order_management;
pub mod position_management;
pub mod session;
pub mod wallet;
