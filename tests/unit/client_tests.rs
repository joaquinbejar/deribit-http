/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/9/25
******************************************************************************/

//! Unit tests for DeribitHttpClient

use deribit_http::client::DeribitHttpClient;
use deribit_http::config::HttpConfig;
use deribit_http::error::HttpError;
use std::time::Duration;
use url::Url;

#[tokio::test]
async fn test_client_new_testnet() {
    let client = DeribitHttpClient::new(true);
    assert!(client.base_url().contains("test.deribit.com"));
}

#[tokio::test]
async fn test_client_new_production() {
    let client = DeribitHttpClient::new(false);
    assert!(client.base_url().contains("deribit.com"));
    assert!(!client.base_url().contains("test"));
}

#[tokio::test]
async fn test_client_with_config() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.example.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: false,
        credentials: None,
    };

    let client = DeribitHttpClient::with_config(config.clone()).unwrap();
    assert_eq!(client.base_url(), "https://test.example.com/");
    assert_eq!(client.config().timeout, Duration::from_secs(30));
    assert_eq!(client.config().user_agent, "test-agent");
}

#[tokio::test]
async fn test_client_with_invalid_config() {
    // Test with invalid URL - this will fail at URL parsing
    let result = Url::parse("");
    assert!(result.is_err());

    // Test with valid URL but invalid config
    let config = HttpConfig {
        base_url: Url::parse("https://test.example.com").unwrap(),
        timeout: Duration::from_secs(0), // Invalid zero timeout
        user_agent: "".to_string(),      // Invalid empty user agent
        max_retries: 0,
        testnet: false,
        credentials: None,
    };

    let result = DeribitHttpClient::with_config(config);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_client_config_access() {
    let client = DeribitHttpClient::new(true);
    let config = client.config();
    assert!(!config.base_url.as_str().is_empty());
    assert!(config.timeout > Duration::from_secs(0));
    assert!(!config.user_agent.is_empty());
}

#[tokio::test]
async fn test_client_http_client_access() {
    let client = DeribitHttpClient::new(true);
    let http_client = client.http_client();
    // Just verify we can access the client without panicking
    assert!(!format!("{:?}", http_client).is_empty());
}

#[tokio::test]
async fn test_client_rate_limiter_access() {
    let client = DeribitHttpClient::new(true);
    let rate_limiter = client.rate_limiter();
    // Just verify we can access the rate limiter without panicking
    assert!(!format!("{:?}", rate_limiter).is_empty());
}

#[tokio::test]
async fn test_client_is_authenticated_initially_false() {
    let client = DeribitHttpClient::new(true);
    assert!(!client.is_authenticated().await);
}

#[tokio::test]
async fn test_client_get_auth_token_initially_none() {
    let client = DeribitHttpClient::new(true);
    assert!(client.get_auth_token().await.is_none());
}

#[tokio::test]
async fn test_client_authenticate_api_key_not_implemented() {
    let client = DeribitHttpClient::new(true);
    let result = client.authenticate_api_key("test_key", "test_secret").await;
    assert!(result.is_err());
    match result.unwrap_err() {
        HttpError::AuthenticationFailed(msg) => {
            assert!(msg.contains("not yet implemented"));
        }
        _ => panic!("Expected AuthenticationFailed error"),
    }
}

#[tokio::test]
async fn test_make_authenticated_request_without_auth() {
    let client = DeribitHttpClient::new(true);
    let result = client
        .make_authenticated_request("https://test.deribit.com/api/v2/public/test")
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        HttpError::AuthenticationFailed(msg) => {
            assert!(msg.contains("No valid authentication token"));
        }
        _ => panic!("Expected AuthenticationFailed error"),
    }
}

#[tokio::test]
async fn test_make_authenticated_post_request_without_auth() {
    let client = DeribitHttpClient::new(true);
    let body = serde_json::json!({"test": "data"});
    let result = client
        .make_authenticated_post_request("https://test.deribit.com/api/v2/private/test", &body)
        .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        HttpError::AuthenticationFailed(msg) => {
            assert!(msg.contains("No valid authentication token"));
        }
        _ => panic!("Expected AuthenticationFailed error"),
    }
}

#[cfg(test)]
mod mock_tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_make_request_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/api/v2/public/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": "success"}"#)
            .create_async()
            .await;

        let config = HttpConfig {
            base_url: Url::parse(&server.url()).unwrap(),
            timeout: Duration::from_secs(30),
            user_agent: "test-agent".to_string(),
            max_retries: 3,
            testnet: false,
            credentials: None,
        };

        let client = DeribitHttpClient::with_config(config).unwrap();
        let url = format!("{}/api/v2/public/test", server.url());
        let result = client.make_request(&url).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.status().is_success());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_exchange_token_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "//public/exchange_token")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("refresh_token".into(), "test_refresh_token".into()),
                mockito::Matcher::UrlEncoded("subject_id".into(), "12345".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"access_token": "new_token", "expires_in": 3600, "refresh_token": "new_refresh", "scope": "read", "token_type": "Bearer"}}"#)
            .create_async()
            .await;

        let config = HttpConfig {
            base_url: Url::parse(&server.url()).unwrap(),
            timeout: Duration::from_secs(30),
            user_agent: "test-agent".to_string(),
            max_retries: 3,
            testnet: false,
            credentials: None,
        };

        let client = DeribitHttpClient::with_config(config).unwrap();
        let result = client
            .exchange_token("test_refresh_token", 12345, None)
            .await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token.access_token, "new_token");
        assert_eq!(token.expires_in, 3600);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_exchange_token_with_scope() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "//public/exchange_token")
            .match_query(mockito::Matcher::Regex(r"refresh_token=test_refresh_token.*subject_id=12345.*scope=read%20write".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"access_token": "new_token", "expires_in": 3600, "refresh_token": "new_refresh", "scope": "read write", "token_type": "Bearer"}}"#)
            .create_async()
            .await;

        let config = HttpConfig {
            base_url: Url::parse(&server.url()).unwrap(),
            timeout: Duration::from_secs(30),
            user_agent: "test-agent".to_string(),
            max_retries: 3,
            testnet: false,
            credentials: None,
        };

        let client = DeribitHttpClient::with_config(config).unwrap();
        let result = client
            .exchange_token("test_refresh_token", 12345, Some("read write"))
            .await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token.scope, "read write");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_fork_token_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "//public/fork_token")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("refresh_token".into(), "test_refresh_token".into()),
                mockito::Matcher::UrlEncoded("session_name".into(), "test_session".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": {"access_token": "forked_token", "expires_in": 3600, "refresh_token": "forked_refresh", "scope": "read", "token_type": "Bearer"}}"#)
            .create_async()
            .await;

        let config = HttpConfig {
            base_url: Url::parse(&server.url()).unwrap(),
            timeout: Duration::from_secs(30),
            user_agent: "test-agent".to_string(),
            max_retries: 3,
            testnet: false,
            credentials: None,
        };

        let client = DeribitHttpClient::with_config(config).unwrap();
        let result = client
            .fork_token("test_refresh_token", "test_session", None)
            .await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token.access_token, "forked_token");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_exchange_token_error_response() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "//public/exchange_token")
            .match_query(mockito::Matcher::Regex(
                r"refresh_token=invalid_token.*subject_id=12345".to_string(),
            ))
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": {"code": 13009, "message": "invalid_token"}}"#)
            .create_async()
            .await;

        let config = HttpConfig {
            base_url: Url::parse(&server.url()).unwrap(),
            timeout: Duration::from_secs(30),
            user_agent: "test-agent".to_string(),
            max_retries: 3,
            testnet: false,
            credentials: None,
        };

        let client = DeribitHttpClient::with_config(config).unwrap();
        let result = client.exchange_token("invalid_token", 12345, None).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            HttpError::AuthenticationFailed(msg) => {
                assert!(msg.contains("Token exchange failed"));
            }
            _ => panic!("Expected AuthenticationFailed error"),
        }

        mock.assert_async().await;
    }
}
