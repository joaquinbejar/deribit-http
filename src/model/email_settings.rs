//! Email settings models for Deribit API
//!
//! This module contains types for email language preferences.

use serde::{Deserialize, Serialize};

/// Supported email languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmailLanguage {
    /// English
    En,
    /// Korean
    Ko,
    /// Chinese
    Zh,
    /// Japanese
    Ja,
    /// Russian
    Ru,
    /// Spanish
    Es,
    /// Portuguese
    Pt,
    /// Turkish
    Tr,
    /// Vietnamese
    Vi,
}

impl EmailLanguage {
    /// Returns the language code as a string
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ko => "ko",
            Self::Zh => "zh",
            Self::Ja => "ja",
            Self::Ru => "ru",
            Self::Es => "es",
            Self::Pt => "pt",
            Self::Tr => "tr",
            Self::Vi => "vi",
        }
    }
}

impl std::fmt::Display for EmailLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_language_serialization() {
        let lang = EmailLanguage::En;
        let json = serde_json::to_string(&lang).expect("Failed to serialize");
        assert_eq!(json, "\"en\"");
    }

    #[test]
    fn test_email_language_deserialization() {
        let json = "\"ko\"";
        let lang: EmailLanguage = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(lang, EmailLanguage::Ko);
    }

    #[test]
    fn test_email_language_as_str() {
        assert_eq!(EmailLanguage::Ja.as_str(), "ja");
        assert_eq!(EmailLanguage::Ru.as_str(), "ru");
    }
}
