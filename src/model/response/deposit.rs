/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use crate::model::deposit::Deposit;

/// Deposits response wrapper
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct DepositsResponse {
    /// Total count of deposits
    pub count: u32,
    /// List of deposit entries
    pub data: Vec<Deposit>,
}