//! Admin API
//!
//! This module contains the implementations for the Anthropic Admin API endpoints.
//! It provides functionality for managing API keys and other administrative tasks.

use crate::client::AnthropicClient;
use crate::types::admin::api_keys::{
    AdminClient, AdminError, ListApiKeysParams, ListApiKeysResponse,
};
use async_trait::async_trait;

#[async_trait]
impl AdminClient for AnthropicClient {
    /// Lists API keys
    ///
    /// Retrieves a list of API keys with optional filtering and pagination.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// Returns a list of API keys and pagination information on success.
    ///
    /// # Errors
    ///
    /// Returns an `AdminError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::client::AnthropicClient;
    /// use anthropic_ai_sdk::types::admin::{AdminClient, AdminError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), AdminError> {
    ///     let client = AnthropicClient::new::<AdminError>(
    ///         "your-admin-api-key",
    ///         "2023-06-01",
    ///     )?;
    ///
    ///     // List all API keys
    ///     let api_keys = client.list_api_keys(None).await?;
    ///     for api_key in api_keys.data {
    ///         println!("API Key: {} ({})", api_key.name, api_key.id);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn list_api_keys<'a>(
        &'a self,
        params: Option<&'a ListApiKeysParams>,
    ) -> Result<ListApiKeysResponse, AdminError> {
        self.get("/organizations/api_keys", params).await
    }
}
