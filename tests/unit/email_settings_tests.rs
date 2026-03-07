//! Unit tests for email settings models

use deribit_http::model::email_settings::EmailLanguage;

#[test]
fn test_email_language_as_str_en() {
    let lang = EmailLanguage::En;
    assert_eq!(lang.as_str(), "en");
}

#[test]
fn test_email_language_as_str_zh() {
    let lang = EmailLanguage::Zh;
    assert_eq!(lang.as_str(), "zh");
}

#[test]
fn test_email_language_as_str_ru() {
    let lang = EmailLanguage::Ru;
    assert_eq!(lang.as_str(), "ru");
}

#[test]
fn test_email_language_as_str_ko() {
    let lang = EmailLanguage::Ko;
    assert_eq!(lang.as_str(), "ko");
}

#[test]
fn test_email_language_as_str_ja() {
    let lang = EmailLanguage::Ja;
    assert_eq!(lang.as_str(), "ja");
}

#[test]
fn test_email_language_as_str_es() {
    let lang = EmailLanguage::Es;
    assert_eq!(lang.as_str(), "es");
}

#[test]
fn test_email_language_as_str_pt() {
    let lang = EmailLanguage::Pt;
    assert_eq!(lang.as_str(), "pt");
}

#[test]
fn test_email_language_as_str_tr() {
    let lang = EmailLanguage::Tr;
    assert_eq!(lang.as_str(), "tr");
}

#[test]
fn test_email_language_as_str_vi() {
    let lang = EmailLanguage::Vi;
    assert_eq!(lang.as_str(), "vi");
}

#[test]
fn test_email_language_display_en() {
    let lang = EmailLanguage::En;
    assert_eq!(format!("{}", lang), "en");
}

#[test]
fn test_email_language_display_zh() {
    let lang = EmailLanguage::Zh;
    assert_eq!(format!("{}", lang), "zh");
}

#[test]
fn test_email_language_serialization() {
    let lang = EmailLanguage::En;
    let json = serde_json::to_string(&lang).expect("Failed to serialize");
    assert_eq!(json, "\"en\"");
}

#[test]
fn test_email_language_deserialization() {
    let json = "\"ru\"";
    let lang: EmailLanguage = serde_json::from_str(json).expect("Failed to parse");
    assert_eq!(lang, EmailLanguage::Ru);
}

#[test]
fn test_email_language_clone() {
    let lang = EmailLanguage::Zh;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

#[test]
fn test_email_language_copy() {
    let lang = EmailLanguage::Ru;
    let copied = lang;
    assert_eq!(lang, copied);
}

#[test]
fn test_email_language_equality() {
    assert_eq!(EmailLanguage::En, EmailLanguage::En);
    assert_ne!(EmailLanguage::En, EmailLanguage::Zh);
}
