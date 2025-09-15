/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use crate::model::types::Withdrawal;

/// Withdrawals response wrapper
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct WithdrawalsResponse {
    /// Total count of withdrawals
    pub count: u32,
    /// List of withdrawal entries
    pub data: Vec<Withdrawal>,
}
