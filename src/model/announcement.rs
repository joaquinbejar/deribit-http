//! Announcement models for Deribit API
//!
//! This module contains types for platform announcements.

use serde::{Deserialize, Serialize};

/// Platform announcement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Announcement {
    /// Announcement ID
    pub id: u64,
    /// Announcement title
    pub title: String,
    /// Announcement body/content in HTML format
    pub body: String,
    /// Publication timestamp in milliseconds
    pub publication_timestamp: u64,
    /// Whether the announcement is important
    pub important: bool,
    /// Optional action URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_announcement_deserialization() {
        let json = r#"{
            "id": 1550058362418,
            "title": "Test Announcement",
            "body": "<p>Test content</p>",
            "publication_timestamp": 1550058362000,
            "important": true,
            "action": "https://deribit.com"
        }"#;

        let announcement: Announcement = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(announcement.id, 1550058362418);
        assert_eq!(announcement.title, "Test Announcement");
        assert!(announcement.important);
        assert!(announcement.action.is_some());
    }

    #[test]
    fn test_announcement_without_action() {
        let json = r#"{
            "id": 123,
            "title": "Simple",
            "body": "Content",
            "publication_timestamp": 1000000,
            "important": false
        }"#;

        let announcement: Announcement = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(announcement.id, 123);
        assert!(announcement.action.is_none());
    }
}
