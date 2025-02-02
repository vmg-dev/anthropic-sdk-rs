//! Anthropic API client implementation
//!
//! This module provides the main client for interacting with the Anthropic API.
//! It handles authentication, request construction, and response parsing.

use reqwest::{header, Client as ReqwestClient};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error as StdError;

/// Base URL for the Anthropic API
const API_BASE_URL: &str = "https://api.anthropic.com/v1";

/// Anthropic API client
///
/// The main client for making requests to the Anthropic API.
/// Handles authentication and provides methods for making API requests.
///
/// # Examples
///
/// ```no_run
/// use anthropic_ai_sdk::clients::AnthropicClient;
/// use anthropic_ai_sdk::types::model::ModelError;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = AnthropicClient::new::<ModelError>(
///     "your-api-key",
///     "2023-06-01",
/// )?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct AnthropicClient {
    /// The underlying HTTP client for making requests
    client: ReqwestClient,
    /// The API key used for authentication with Anthropic's services
    api_key: String,
}

impl AnthropicClient {
    /// Creates a new Anthropic API client with the specified credentials
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Anthropic API key for authentication
    /// * `api_version` - The API version to use (e.g., "2023-06-01")
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The API version header cannot be created
    /// - The HTTP client cannot be initialized
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anthropic_ai_sdk::clients::AnthropicClient;
    /// # use anthropic_ai_sdk::types::model::ModelError;
    /// let client = AnthropicClient::new::<ModelError>(
    ///     "your-api-key",
    ///     "2023-06-01",
    /// ).unwrap();
    /// ```
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

    /// Sends a request to the Anthropic API with the specified parameters
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type that can be deserialized from JSON
    /// * `Q` - The query parameters type that can be serialized
    /// * `B` - The request body type that can be serialized
    /// * `E` - The error type that can be created from a string
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use for the request
    /// * `path` - The API endpoint path (will be appended to the base URL)
    /// * `query` - Optional query parameters to include in the URL
    /// * `body` - Optional request body to send
    ///
    /// # Returns
    ///
    /// Returns the deserialized response on success, or an error if:
    /// - The request fails to send
    /// - The response indicates an error (non-2xx status)
    /// - The response body cannot be parsed
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

        let mut request = self
            .client
            .request(method, &url)
            .header("x-api-key", &self.api_key);

        // Add query parameters if provided
        if let Some(q) = query {
            request = request.query(q);
        }

        // Add request body if provided
        if let Some(b) = body {
            let _json = serde_json::to_string_pretty(b)
                .map_err(|e| E::from(format!("Failed to serialize body: {}", e)))?;
            request = request.json(b);
        }

        let response = request.send().await.map_err(|e| E::from(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| E::from(format!("Failed to get response body: {}", e)))?;

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

    /// Sends a GET request to the specified endpoint
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `Q` - The query parameters type
    /// * `E` - The error type
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `query` - Optional query parameters
    pub(crate) async fn get<T, Q, E>(&self, path: &str, query: Option<&Q>) -> Result<T, E>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request::<T, Q, (), E>(reqwest::Method::GET, path, query, None)
            .await
    }

    /// Sends a POST request to the specified endpoint
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `B` - The request body type
    /// * `E` - The error type
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `body` - Optional request body
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
