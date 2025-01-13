use crate::types::model::{ListModelsParams, ModelError};
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
