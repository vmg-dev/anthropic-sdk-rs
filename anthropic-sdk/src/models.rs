use crate::types::client::AnthropicClient;
use crate::types::model::{ListModelsParams, ListModelsResponse, ModelError};

/// Models API endpoints implementation
impl AnthropicClient {
    /// List available models
    ///
    /// Returns a list of models that are available through the API.
    /// More recently released models are listed first.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for pagination and limiting results
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_sdk::AnthropicClient;
    /// use anthropic_sdk::types::model::ListModelsParams;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = AnthropicClient::new("your-api-key", "2023-06-01")?;
    ///     
    ///     // List models with default parameters
    ///     let models = client.list_models(None).await?;
    ///     
    ///     // List models with custom parameters
    ///     let params = ListModelsParams {
    ///         limit: Some(5),
    ///         after_id: Some("model-id".to_string()),
    ///         before_id: None,
    ///     };
    ///     let models = client.list_models(Some(&params)).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_models(
        &self,
        params: Option<&ListModelsParams>,
    ) -> Result<ListModelsResponse, ModelError> {
        self.send_request(reqwest::Method::GET, "/models", params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add tests for list_models
}
