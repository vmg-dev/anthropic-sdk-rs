//! Message Batches API
//!
//! This module contains the implementations for the Anthropic Message Batches API endpoints.
//! It provides functionality for creating message batches.

use crate::clients::AnthropicClient;
use crate::types::message_batches::{
    CreateMessageBatchParams, MessageBatch, MessageBatchClient, MessageBatchError,
};
use async_trait::async_trait;

#[async_trait]
impl MessageBatchClient for AnthropicClient {
    /// Creates a message batch
    ///
    /// Creates a message batch with the given parameters and returns the model's response.
    ///
    /// # Arguments
    ///
    /// * `body` - Parameters for creating the message batch, including the model to use,
    ///   the messages to send, and any additional options
    ///
    /// # Returns
    ///
    /// Returns the model's response on success, including the generated message
    /// and any additional metadata.
    ///
    /// # Errors
    ///
    /// Returns a `MessageBatchError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::clients::AnthropicClient;
    /// use anthropic_ai_sdk::types::message_batches::{MessageBatchClient, MessageBatchError};
    /// use anthropic_ai_sdk::types::message_batches::{
    ///     CreateMessageBatchParams, MessageBatch
    /// };
    /// use tokio;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AnthropicClient::new::<MessageBatchError>(
    ///     "your-api-key",
    ///     "2023-06-01",
    /// )?;
    ///
    /// let params = CreateMessageBatchParams::default();
    /// let response = client.create_message_batch(&params).await?;
    ///
    /// println!("Response: {:?}", response);
    /// # Ok(())
    /// # }
    /// ```
    async fn create_message_batch<'a>(
        &'a self,
        body: &'a CreateMessageBatchParams,
    ) -> Result<MessageBatch, MessageBatchError> {
        self.post("/messages/batches", Some(body)).await
    }
}
