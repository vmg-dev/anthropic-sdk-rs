use reqwest::{header, Client as ReqwestClient};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error as StdError;
use tracing::info;

const API_BASE_URL: &str = "https://api.anthropic.com/v1";

/// Anthropic API client
#[derive(Debug, Clone)]
pub struct AnthropicClient {
    client: ReqwestClient,
    api_key: String,
}

impl AnthropicClient {
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
    pub(crate) async fn send_request<T, P, E>(
        &self,
        method: reqwest::Method,
        path: &str,
        params: Option<&P>,
        body: Option<&P>,
    ) -> Result<T, E>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        let url = format!("{}{}", API_BASE_URL, path);
        info!("url: {}", url);

        let mut request = self
            .client
            .request(method, &url)
            .header("x-api-key", &self.api_key);

        // Add query parameters if provided
        if let Some(q) = params {
            request = request.query(q);
        }

        // Add request body if provided
        if let Some(b) = body {
            request = request.json(b);
        }

        let response = request.send().await.map_err(|e| E::from(e.to_string()))?;

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
    pub(crate) async fn get<T, P, E>(&self, path: &str, params: Option<&P>) -> Result<T, E>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request::<T, P, E>(reqwest::Method::GET, path, params, None)
            .await
    }

    /// Helper method for POST requests
    pub(crate) async fn post<T, P, E>(&self, path: &str, body: Option<&P>) -> Result<T, E>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request(reqwest::Method::POST, path, None, body)
            .await
    }
}
