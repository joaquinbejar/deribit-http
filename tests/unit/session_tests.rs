/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/9/25
******************************************************************************/

//! Unit tests for HttpSession

use deribit_http::config::HttpConfig;
use deribit_http::model::http_types::AuthToken;
use deribit_http::session::http_session::HttpSession;
use std::time::Duration;
use url::Url;

#[tokio::test]
async fn test_http_session_new() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.deribit.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: true,
        credentials: None,
    };

    let session = HttpSession::new(config.clone());
    assert_eq!(
        session.config().base_url.as_str(),
        "https://test.deribit.com/"
    );
    assert_eq!(session.config().timeout, Duration::from_secs(30));
}

#[tokio::test]
async fn test_session_config_access() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.example.com").unwrap(),
        timeout: Duration::from_secs(60),
        user_agent: "custom-agent".to_string(),
        max_retries: 5,
        testnet: false,
        credentials: None,
    };

    let session = HttpSession::new(config.clone());
    let retrieved_config = session.config();

    assert_eq!(
        retrieved_config.base_url.as_str(),
        "https://test.example.com/"
    );
    assert_eq!(retrieved_config.timeout, Duration::from_secs(60));
    assert_eq!(retrieved_config.user_agent, "custom-agent");
    assert_eq!(retrieved_config.max_retries, 5);
}

#[tokio::test]
async fn test_session_initially_not_authenticated() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.deribit.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: true,
        credentials: None,
    };

    let session = HttpSession::new(config);
    assert!(!session.is_authenticated().await);
    assert!(session.auth_token().await.is_none());
    assert!(session.authorization_header().await.is_none());
}

#[tokio::test]
async fn test_set_and_get_auth_token() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.deribit.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: true,
        credentials: None,
    };

    let session = HttpSession::new(config);

    let token = AuthToken {
        access_token: "test_access_token".to_string(),
        expires_in: 3600,
        refresh_token: Some("test_refresh_token".to_string()),
        scope: "read".to_string(),
        token_type: "Bearer".to_string(),
    };

    session.set_auth_token(token.clone()).await;

    assert!(session.is_authenticated().await);
    let retrieved_token = session.auth_token().await;
    assert!(retrieved_token.is_some());

    let retrieved_token = retrieved_token.unwrap();
    assert_eq!(retrieved_token.access_token, "test_access_token");
    assert_eq!(retrieved_token.expires_in, 3600);
    assert_eq!(
        retrieved_token.refresh_token,
        Some("test_refresh_token".to_string())
    );
    assert_eq!(retrieved_token.scope, "read");
    assert_eq!(retrieved_token.token_type, "Bearer");
}

#[tokio::test]
async fn test_authorization_header() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.deribit.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: true,
        credentials: None,
    };

    let session = HttpSession::new(config);

    // Initially no authorization header
    assert!(session.authorization_header().await.is_none());

    let token = AuthToken {
        access_token: "test_access_token_123".to_string(),
        expires_in: 3600,
        refresh_token: Some("test_refresh_token".to_string()),
        scope: "read write".to_string(),
        token_type: "Bearer".to_string(),
    };

    session.set_auth_token(token).await;

    let auth_header = session.authorization_header().await;
    assert!(auth_header.is_some());
    assert_eq!(auth_header.unwrap(), "Bearer test_access_token_123");
}

#[tokio::test]
async fn test_authorization_header_different_token_types() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.deribit.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: true,
        credentials: None,
    };

    let session = HttpSession::new(config);

    // Test with different token type
    let token = AuthToken {
        access_token: "custom_token_456".to_string(),
        expires_in: 7200,
        refresh_token: Some("refresh_456".to_string()),
        scope: "admin".to_string(),
        token_type: "Custom".to_string(),
    };

    session.set_auth_token(token).await;

    let auth_header = session.authorization_header().await;
    assert!(auth_header.is_some());
    assert_eq!(auth_header.unwrap(), "Custom custom_token_456");
}

#[tokio::test]
async fn test_clear_auth_token() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.deribit.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: true,
        credentials: None,
    };

    let session = HttpSession::new(config);

    let token = AuthToken {
        access_token: "test_access_token".to_string(),
        expires_in: 3600,
        refresh_token: Some("test_refresh_token".to_string()),
        scope: "read".to_string(),
        token_type: "Bearer".to_string(),
    };

    // Set token
    session.set_auth_token(token).await;
    assert!(session.is_authenticated().await);
    assert!(session.auth_token().await.is_some());

    // Clear token
    session.clear_auth_token().await;
    assert!(!session.is_authenticated().await);
    assert!(session.auth_token().await.is_none());
    assert!(session.authorization_header().await.is_none());
}

#[tokio::test]
async fn test_is_token_expired() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.deribit.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: true,
        credentials: None,
    };

    let session = HttpSession::new(config);

    // Currently always returns false (TODO in implementation)
    assert!(!session.is_token_expired().await);

    let token = AuthToken {
        access_token: "test_access_token".to_string(),
        expires_in: 3600,
        refresh_token: Some("test_refresh_token".to_string()),
        scope: "read".to_string(),
        token_type: "Bearer".to_string(),
    };

    session.set_auth_token(token).await;

    // Still returns false as implementation is not complete
    assert!(!session.is_token_expired().await);
}

#[tokio::test]
async fn test_session_clone() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.deribit.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: true,
        credentials: None,
    };

    let session1 = HttpSession::new(config);
    let session2 = session1.clone();

    let token = AuthToken {
        access_token: "shared_token".to_string(),
        expires_in: 3600,
        refresh_token: Some("shared_refresh".to_string()),
        scope: "read".to_string(),
        token_type: "Bearer".to_string(),
    };

    // Set token in first session
    session1.set_auth_token(token).await;

    // Both sessions should share the same token (Arc<Mutex<>>)
    assert!(session1.is_authenticated().await);
    assert!(session2.is_authenticated().await);

    let token1 = session1.auth_token().await;
    let token2 = session2.auth_token().await;

    assert!(token1.is_some());
    assert!(token2.is_some());
    assert_eq!(token1.unwrap().access_token, token2.unwrap().access_token);
}

#[tokio::test]
async fn test_concurrent_token_access() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.deribit.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: true,
        credentials: None,
    };

    let session = HttpSession::new(config);
    let session_clone = session.clone();

    let token = AuthToken {
        access_token: "concurrent_token".to_string(),
        expires_in: 3600,
        refresh_token: Some("concurrent_refresh".to_string()),
        scope: "read".to_string(),
        token_type: "Bearer".to_string(),
    };

    // Test concurrent access
    let handle1 = tokio::spawn(async move {
        session.set_auth_token(token).await;
        session.is_authenticated().await
    });

    let handle2 = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(10)).await;
        session_clone.is_authenticated().await
    });

    let (result1, result2) = tokio::join!(handle1, handle2);

    assert!(result1.unwrap());
    // result2 might be true or false depending on timing, but shouldn't panic
    let _ = result2.unwrap();
}
