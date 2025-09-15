//! HTTP connection management

use crate::config::HttpConfig;
use crate::error::HttpError;
use reqwest::Client;
use std::collections::HashMap;
use crate::model::request::api_request::HttpRequest;
use crate::model::response::api_response::HttpResponse;

/// HTTP connection wrapper
#[derive(Debug, Clone)]
pub struct HttpConnection {
    client: Client,
    config: HttpConfig,
}

impl HttpConnection {
    /// Create a new HTTP connection
    pub fn new(config: HttpConfig) -> Result<Self, HttpError> {
        let client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        Ok(Self { client, config })
    }

    /// Send an HTTP request
    pub async fn send_request(&self, request: &HttpRequest) -> Result<HttpResponse, HttpError> {
        let mut req_builder = match request.method.as_str() {
            "GET" => self.client.get(&request.endpoint),
            "POST" => self.client.post(&request.endpoint),
            "PUT" => self.client.put(&request.endpoint),
            "DELETE" => self.client.delete(&request.endpoint),
            _ => {
                return Err(HttpError::RequestFailed(format!(
                    "Unsupported method: {}",
                    request.method
                )));
            }
        };

        // Add headers
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }

        // Add body if present
        if let Some(body) = &request.body {
            req_builder = req_builder.body(body.clone());
        }

        // Send request
        let response = req_builder
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        // Extract response data
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect::<HashMap<String, String>>();
        let body = response
            .text()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }

    /// Get the configuration
    pub fn config(&self) -> &HttpConfig {
        &self.config
    }

    /// Get the HTTP client
    pub fn client(&self) -> &Client {
        &self.client
    }
}
