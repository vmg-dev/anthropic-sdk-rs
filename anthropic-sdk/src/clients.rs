use crate::types::model::{ListModelsParams, ModelError};
use reqwest::{header, Client as ReqwestClient};
use serde::de::DeserializeOwned;
use tracing::info;

const API_BASE_URL: &str = "https://api.anthropic.com/v1";

/// Anthropic API client
#[derive(Debug, Clone)]
pub struct AnthropicClient {
    client: ReqwestClient,
    api_key: String,
    api_version: String,
}

impl AnthropicClient {
    pub fn new(
        api_key: impl Into<String>,
        api_version: impl Into<String>,
    ) -> Result<Self, ModelError> {
        let api_version_str = api_version.into();
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "anthropic-version",
            header::HeaderValue::from_str(&api_version_str)
                .map_err(|e| ModelError::RequestFailed(e.to_string()))?,
        );

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| ModelError::RequestFailed(e.to_string()))?;

        Ok(Self {
            client,
            api_key: api_key.into(),
            api_version: api_version_str,
        })
    }

    pub(crate) async fn send_request<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<&ListModelsParams>,
    ) -> Result<T, ModelError> {
        let url = format!("{}{}", API_BASE_URL, path);
        info!("url: {}", url);

        let mut request = self
            .client
            .request(method, &url)
            .header("x-api-key", &self.api_key);

        if let Some(q) = query {
            request = request.query(q);
        }

        let response = request
            .send()
            .await
            .map_err(|e| ModelError::RequestFailed(e.to_string()))?;

        let status = response.status();
        let body = response.text().await.map_err(|e| {
            ModelError::RequestFailed(format!("Failed to get response body: {}", e))
        })?;

        info!("Response status: {}", status);
        info!("Response body: {}", body);

        if !status.is_success() {
            return Err(ModelError::RequestFailed(body));
        }

        // Now try to parse the JSON
        serde_json::from_str(&body).map_err(|e| {
            ModelError::RequestFailed(format!(
                "JSON parsing error: {}. Response body: {}",
                e, body
            ))
        })
    }
}
