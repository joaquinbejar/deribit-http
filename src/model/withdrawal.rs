/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Withdrawal priority information
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalPriority {
    /// Priority name (e.g., "very_low", "low", "medium", "high", "very_high")
    pub name: String,
    /// Priority value (fee multiplier)
    pub value: f64,
}

impl WithdrawalPriority {
    /// Create a new withdrawal priority
    pub fn new(name: String, value: f64) -> Self {
        Self { name, value }
    }

    /// Create a very low priority
    pub fn very_low() -> Self {
        Self::new("very_low".to_string(), 0.15)
    }

    /// Create a low priority
    pub fn low() -> Self {
        Self::new("low".to_string(), 0.5)
    }

    /// Create a medium priority
    pub fn medium() -> Self {
        Self::new("medium".to_string(), 1.0)
    }

    /// Create a high priority
    pub fn high() -> Self {
        Self::new("high".to_string(), 1.2)
    }

    /// Create a very high priority
    pub fn very_high() -> Self {
        Self::new("very_high".to_string(), 1.5)
    }
}