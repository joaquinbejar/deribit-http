/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 7/3/26
******************************************************************************/
//! Trigger order response models

use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A single entry in the trigger order history
///
/// Represents a trigger order event such as creation, activation,
/// execution, or cancellation.
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerOrderHistoryEntry {
    /// Timestamp of the event in milliseconds since Unix epoch
    pub timestamp: i64,
    /// Trigger type: "index_price", "mark_price", or "last_price"
    pub trigger: Option<String>,
    /// Trigger price (only for future trigger orders)
    pub trigger_price: Option<f64>,
    /// Maximum deviation from price peak for trailing trigger orders
    pub trigger_offset: Option<f64>,
    /// ID of the trigger order before triggering
    pub trigger_order_id: String,
    /// Unique order identifier after triggering
    pub order_id: Option<String>,
    /// Order state: "triggered", "cancelled", or "rejected"
    pub order_state: String,
    /// Unique instrument identifier
    pub instrument_name: String,
    /// Type of last request: "cancel" or "trigger:order"
    pub request: Option<String>,
    /// Direction: "buy" or "sell"
    pub direction: String,
    /// Price in base currency
    pub price: Option<f64>,
    /// Order size (USD for perpetual/inverse, base currency for options/linear)
    pub amount: f64,
    /// True for reduce-only orders
    pub reduce_only: Option<bool>,
    /// True for post-only orders
    pub post_only: Option<bool>,
    /// Order type: "limit" or "market"
    pub order_type: Option<String>,
    /// User defined label
    pub label: Option<String>,
    /// True if order can be triggered by another order
    pub linked_order_type: Option<String>,
    /// Unique reference for OCO (one_cancels_others) pair
    pub oco_ref: Option<String>,
    /// Source of the order linked to trigger order
    pub trigger_source: Option<String>,
    /// Last update timestamp in milliseconds since Unix epoch
    pub last_update_timestamp: Option<i64>,
}

/// Response from get_trigger_order_history endpoint
///
/// Contains a list of trigger order history entries and an optional
/// continuation token for pagination.
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerOrderHistoryResponse {
    /// List of trigger order history entries
    pub entries: Vec<TriggerOrderHistoryEntry>,
    /// Continuation token for pagination
    pub continuation: Option<String>,
}

impl TriggerOrderHistoryResponse {
    /// Create a new trigger order history response
    pub fn new(entries: Vec<TriggerOrderHistoryEntry>) -> Self {
        Self {
            entries,
            continuation: None,
        }
    }

    /// Create response with continuation token
    pub fn with_continuation(entries: Vec<TriggerOrderHistoryEntry>, continuation: String) -> Self {
        Self {
            entries,
            continuation: Some(continuation),
        }
    }

    /// Check if there are more results
    pub fn has_more(&self) -> bool {
        self.continuation.is_some()
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the response is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for TriggerOrderHistoryResponse {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_order_history_response_new() {
        let response = TriggerOrderHistoryResponse::new(vec![]);
        assert!(response.entries.is_empty());
        assert!(response.continuation.is_none());
        assert!(!response.has_more());
    }

    #[test]
    fn test_trigger_order_history_response_with_continuation() {
        let response =
            TriggerOrderHistoryResponse::with_continuation(vec![], "token123".to_string());
        assert!(response.entries.is_empty());
        assert_eq!(response.continuation, Some("token123".to_string()));
        assert!(response.has_more());
    }

    #[test]
    fn test_trigger_order_history_entry_deserialization() {
        let json = r#"{
            "timestamp": 1555918941451,
            "trigger": "index_price",
            "trigger_price": 5285.0,
            "trigger_order_id": "SLIS-103",
            "order_id": "671473",
            "order_state": "triggered",
            "instrument_name": "BTC-PERPETUAL",
            "request": "trigger:order",
            "direction": "buy",
            "price": 5179.28,
            "amount": 10.0
        }"#;

        let entry: TriggerOrderHistoryEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.timestamp, 1555918941451);
        assert_eq!(entry.trigger, Some("index_price".to_string()));
        assert_eq!(entry.trigger_price, Some(5285.0));
        assert_eq!(entry.trigger_order_id, "SLIS-103");
        assert_eq!(entry.order_id, Some("671473".to_string()));
        assert_eq!(entry.order_state, "triggered");
        assert_eq!(entry.instrument_name, "BTC-PERPETUAL");
        assert_eq!(entry.direction, "buy");
        assert_eq!(entry.amount, 10.0);
    }
}
