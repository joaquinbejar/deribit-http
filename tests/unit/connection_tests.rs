/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/9/25
******************************************************************************/

//! Unit tests for HttpConnection

use deribit_http::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use url::Url;


#[tokio::test]
async fn test_http_connection_new() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.deribit.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: true,
        credentials: None,
    };

    let connection = HttpConnection::new(config.clone()).unwrap();
    assert_eq!(
        connection.config().base_url.as_str(),
        "https://test.deribit.com/"
    );
    assert_eq!(connection.config().timeout, Duration::from_secs(30));
}

#[tokio::test]
async fn test_http_connection_config_access() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.example.com").unwrap(),
        timeout: Duration::from_secs(60),
        user_agent: "custom-agent".to_string(),
        max_retries: 5,
        testnet: false,
        credentials: None,
    };

    let connection = HttpConnection::new(config.clone()).unwrap();
    let retrieved_config = connection.config();

    assert_eq!(
        retrieved_config.base_url.as_str(),
        "https://test.example.com/"
    );
    assert_eq!(retrieved_config.timeout, Duration::from_secs(60));
    assert_eq!(retrieved_config.user_agent, "custom-agent");
    assert_eq!(retrieved_config.max_retries, 5);
}

#[tokio::test]
async fn test_http_connection_client_access() {
    let config = HttpConfig {
        base_url: Url::parse("https://test.deribit.com").unwrap(),
        timeout: Duration::from_secs(30),
        user_agent: "test-agent".to_string(),
        max_retries: 3,
        testnet: true,
        credentials: None,
    };

    let connection = HttpConnection::new(config).unwrap();
    let client = connection.client();

    // Just verify we can access the client without panicking
    assert!(!format!("{:?}", client).is_empty());
}

#[cfg(test)]
mod mock_tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_send_get_request() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
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

        let connection = HttpConnection::new(config).unwrap();

        let request = HttpRequest {
            method: "GET".to_string(),
            endpoint: format!("{}/test", server.url()),
            headers: HashMap::new(),
            body: None,
        };

        let result = connection.send_request(&request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.body, r#"{"result": "success"}"#);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_post_request() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/test")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": "created"}"#)
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

        let connection = HttpConnection::new(config).unwrap();

        let request = HttpRequest {
            method: "POST".to_string(),
            endpoint: format!("{}/test", server.url()),
            headers: HashMap::new(),
            body: Some(r#"{"data": "test"}"#.to_string()),
        };

        let result = connection.send_request(&request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status, 201);
        assert_eq!(response.body, r#"{"result": "created"}"#);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_put_request() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("PUT", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": "updated"}"#)
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

        let connection = HttpConnection::new(config).unwrap();

        let request = HttpRequest {
            method: "PUT".to_string(),
            endpoint: format!("{}/test", server.url()),
            headers: HashMap::new(),
            body: Some(r#"{"data": "updated"}"#.to_string()),
        };

        let result = connection.send_request(&request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.body, r#"{"result": "updated"}"#);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_delete_request() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/test")
            .with_status(204)
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

        let connection = HttpConnection::new(config).unwrap();

        let request = HttpRequest {
            method: "DELETE".to_string(),
            endpoint: format!("{}/test", server.url()),
            headers: HashMap::new(),
            body: None,
        };

        let result = connection.send_request(&request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status, 204);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_request_with_headers() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .match_header("authorization", "Bearer token123")
            .match_header("x-custom", "custom-value")
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

        let connection = HttpConnection::new(config).unwrap();

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        headers.insert("X-Custom".to_string(), "custom-value".to_string());

        let request = HttpRequest {
            method: "GET".to_string(),
            endpoint: format!("{}/test", server.url()),
            headers,
            body: None,
        };

        let result = connection.send_request(&request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status, 200);
        assert!(response.headers.contains_key("content-type"));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_unsupported_method() {
        let config = HttpConfig {
            base_url: Url::parse("https://test.example.com").unwrap(),
            timeout: Duration::from_secs(30),
            user_agent: "test-agent".to_string(),
            max_retries: 3,
            testnet: false,
            credentials: None,
        };

        let connection = HttpConnection::new(config).unwrap();

        let request = HttpRequest {
            method: "PATCH".to_string(), // Unsupported method
            endpoint: "https://test.example.com/test".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let result = connection.send_request(&request).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            HttpError::RequestFailed(msg) => {
                assert!(msg.contains("Unsupported method: PATCH"));
            }
            _ => panic!("Expected RequestFailed error"),
        }
    }

    #[tokio::test]
    async fn test_send_request_network_error() {
        let config = HttpConfig {
            base_url: Url::parse("https://invalid-url-that-does-not-exist.com").unwrap(),
            timeout: Duration::from_millis(100), // Very short timeout
            user_agent: "test-agent".to_string(),
            max_retries: 3,
            testnet: false,
            credentials: None,
        };

        let connection = HttpConnection::new(config).unwrap();

        let request = HttpRequest {
            method: "GET".to_string(),
            endpoint: "https://invalid-url-that-does-not-exist.com/test".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let result = connection.send_request(&request).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            HttpError::NetworkError(_) => {
                // Expected network error
            }
            _ => panic!("Expected NetworkError"),
        }
    }

    #[tokio::test]
    async fn test_response_headers_parsing() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("x-rate-limit", "100")
            .with_header("x-custom-header", "test-value")
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

        let connection = HttpConnection::new(config).unwrap();

        let request = HttpRequest {
            method: "GET".to_string(),
            endpoint: format!("{}/test", server.url()),
            headers: HashMap::new(),
            body: None,
        };

        let result = connection.send_request(&request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status, 200);
        assert!(response.headers.contains_key("content-type"));
        assert!(response.headers.contains_key("x-rate-limit"));
        assert!(response.headers.contains_key("x-custom-header"));

        mock.assert_async().await;
    }
}
