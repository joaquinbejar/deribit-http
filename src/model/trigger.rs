/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Trigger type for stop orders
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    /// Index price trigger
    IndexPrice,
    /// Mark price trigger
    MarkPrice,
    /// Last price trigger
    LastPrice,
}

/// Trigger fill condition for linked orders
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerFillCondition {
    /// Trigger on first hit of the trigger price
    FirstHit,
    /// Trigger only when the order is completely filled
    CompleteFill,
    /// Trigger incrementally as the order is filled
    Incremental,
}
