//! Models API
//!
//! This module contains the types and functions for the Anthropic Models API.
//!
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Response structure for the List Models API endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    /// List of model objects
    pub data: Vec<Model>,
    /// First ID in the data list
    pub first_id: Option<String>,
    /// Indicates if there are more results
    pub has_more: bool,
    /// Last ID in the data list
    pub last_id: Option<String>,
}

/// Represents an Anthropic model
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    /// Unique identifier for the model
    pub id: String,
    /// Name of the model (e.g., "claude-3-opus-20240229")
    pub name: String,
    /// Description of the model's capabilities
    pub description: String,
    /// Maximum context window size for the model
    pub context_window: u32,
    /// Whether the model supports system prompts
    pub system_prompt_support: bool,
    /// Model's capabilities and supported features
    pub capabilities: ModelCapabilities,
}

/// Model capabilities and supported features
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelCapabilities {
    /// Maximum tokens per minute rate limit
    pub max_tokens_per_minute: u32,
    /// Whether the model supports streaming responses
    pub streaming: bool,
    /// List of supported message formats
    pub supported_formats: Vec<String>,
}

/// Parameters for listing models
#[derive(Debug, Serialize, Default)]
pub struct ListModelsParams {
    /// Cursor for pagination (before)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_id: Option<String>,
    /// Cursor for pagination (after)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_id: Option<String>,
    /// Number of items per page (1-1000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
}

impl ListModelsParams {
    /// Create a new ListModelsParams with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the before_id parameter
    pub fn before_id(mut self, before_id: impl Into<String>) -> Self {
        self.before_id = Some(before_id.into());
        self
    }

    /// Set the after_id parameter
    pub fn after_id(mut self, after_id: impl Into<String>) -> Self {
        self.after_id = Some(after_id.into());
        self
    }

    /// Set the limit parameter
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit.min(1000));
        self
    }
}

/// Error types for the Models API
#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Invalid pagination parameters")]
    InvalidPagination,
    #[error("Invalid limit value: {0}")]
    InvalidLimit(u16),
    #[error("API request failed: {0}")]
    RequestFailed(String),
}
