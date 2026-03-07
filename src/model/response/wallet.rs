/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 2025
******************************************************************************/
//! Response models for wallet operations.

use crate::model::wallet::AddressBookEntry;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Response containing a list of address book entries.
///
/// Returned by the `/private/get_address_book` endpoint.
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize, Default)]
pub struct AddressBookResponse {
    /// List of address book entries
    pub entries: Vec<AddressBookEntry>,
}

impl AddressBookResponse {
    /// Creates a new address book response.
    #[must_use]
    pub fn new(entries: Vec<AddressBookEntry>) -> Self {
        Self { entries }
    }

    /// Returns true if the response contains no entries.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the number of entries in the response.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_book_response_new() {
        let entries = vec![
            AddressBookEntry::new("bc1qtest1".to_string(), "BTC".to_string()),
            AddressBookEntry::new("bc1qtest2".to_string(), "BTC".to_string()),
        ];
        let response = AddressBookResponse::new(entries);
        assert_eq!(response.len(), 2);
        assert!(!response.is_empty());
    }

    #[test]
    fn test_address_book_response_default() {
        let response = AddressBookResponse::default();
        assert!(response.is_empty());
        assert_eq!(response.len(), 0);
    }

    #[test]
    fn test_address_book_response_serialization() {
        let entries = vec![AddressBookEntry::new(
            "0xtest".to_string(),
            "ETH".to_string(),
        )];
        let response = AddressBookResponse::new(entries);

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"entries\""));
        assert!(json.contains("\"address\":\"0xtest\""));
    }

    #[test]
    fn test_address_book_response_deserialization() {
        let json = r#"{
            "entries": [
                {
                    "address": "bc1qtest",
                    "currency": "BTC",
                    "label": "Test wallet"
                }
            ]
        }"#;

        let response: AddressBookResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response.entries[0].address, "bc1qtest");
        assert_eq!(response.entries[0].currency, "BTC");
    }
}
