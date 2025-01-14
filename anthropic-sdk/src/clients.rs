//! Anthropic API client implementation
//!
//! This module provides the main client for interacting with the Anthropic API.

use reqwest::{header, Client as ReqwestClient};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error as StdError;
use tracing::info;

const API_BASE_URL: &str = "https://api.anthropic.com/v1";

/// Anthropic API client
///
/// The main client for making requests to the Anthropic API.
#[derive(Debug, Clone)]
pub struct AnthropicClient {
    /// The underlying HTTP client
    client: ReqwestClient,
    /// The API key for authentication
    api_key: String,
}

impl AnthropicClient {
    /// Create a new Anthropic API client
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Anthropic API key
    /// * `api_version` - The API version to use (e.g., "2023-06-01")
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be initialized
    pub fn new<E>(api_key: impl Into<String>, api_version: impl Into<String>) -> Result<Self, E>
    where
        E: StdError + From<String>,
    {
        let api_version_str = api_version.into();
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "anthropic-version",
            header::HeaderValue::from_str(&api_version_str).map_err(|e| E::from(e.to_string()))?,
        );

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| E::from(e.to_string()))?;

        Ok(Self {
            client,
            api_key: api_key.into(),
        })
    }

    /// Generic request sender that can handle different parameter types
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `Q` - The query parameters type
    /// * `B` - The request body type
    /// * `E` - The error type
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use
    /// * `path` - The API endpoint path
    /// * `query` - Optional query parameters
    /// * `body` - Optional request body
    pub(crate) async fn send_request<T, Q, B, E>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<T, E>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        let url = format!("{}{}", API_BASE_URL, path);
        info!("url: {}", url);

        let mut request = self
            .client
            .request(method, &url)
            .header("x-api-key", &self.api_key);

        info!("request: {:?}", request);

        // Add query parameters if provided
        if let Some(q) = query {
            info!("start adding query");
            request = request.query(q);
        }

        // Add request body if provided
        if let Some(b) = body {
            info!("start serializing body");
            let json = serde_json::to_string_pretty(b)
                .map_err(|e| E::from(format!("Failed to serialize body: {}", e)))?;
            info!("Request body JSON: {}", json);
            request = request.json(b);
        }

        let response = request.send().await.map_err(|e| E::from(e.to_string()))?;
        info!("response: {:?}", response);

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| E::from(format!("Failed to get response body: {}", e)))?;

        info!("Response status: {}", status);
        info!("Response body: {}", body);

        if !status.is_success() {
            return Err(E::from(body));
        }

        // Parse the JSON response
        serde_json::from_str(&body).map_err(|e| {
            E::from(format!(
                "JSON parsing error: {}. Response body: {}",
                e, body
            ))
        })
    }

    /// Helper method for GET requests
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `Q` - The query parameters type
    /// * `E` - The error type
    pub(crate) async fn get<T, Q, E>(&self, path: &str, query: Option<&Q>) -> Result<T, E>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request::<T, Q, (), E>(reqwest::Method::GET, path, query, None)
            .await
    }

    /// Helper method for POST requests
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `B` - The request body type
    /// * `E` - The error type
    pub(crate) async fn post<T, B, E>(&self, path: &str, body: Option<&B>) -> Result<T, E>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request::<T, (), B, E>(reqwest::Method::POST, path, None, body)
            .await
    }
}
