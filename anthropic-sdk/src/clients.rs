use crate::types::model::{ListModelsParams, ListModelsResponse, ModelError};
use reqwest::{header, Client as ReqwestClient};
use serde::de::DeserializeOwned;

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

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ModelError::RequestFailed(error_text));
        }

        response
            .json::<T>()
            .await
            .map_err(|e| ModelError::RequestFailed(e.to_string()))
    }
}
