//! Message Batches API
//!
//! This module contains the implementations for the Anthropic Message Batches API endpoints.
//! It provides functionality for creating message batches.

use crate::clients::AnthropicClient;
use crate::types::message_batches::{
    CreateMessageBatchParams, ListMessageBatchesParams, ListMessageBatchesResponse, MessageBatch,
    MessageBatchClient, MessageBatchError, RetrieveMessageBatchParams,
    RetrieveMessageBatchResponse,
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
    /// use anthropic_ai_sdk::types::message_batches::{
    ///     CreateMessageBatchParams, Message, MessageBatchClient, MessageBatchError, MessageRequest,
    ///     MessageRequestParams,
    /// };
    ///
    /// use tokio;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AnthropicClient::new::<MessageBatchError>(
    ///     "your-api-key",
    ///     "2023-06-01",
    /// )?;
    ///
    /// let messages = vec![Message::new("user", "Hello!")];
    /// let request_params = MessageRequestParams::new("claude-3-haiku", messages, 100)
    ///     .with_system("You are a helpful assistant");
    /// let request = MessageRequest::new(request_params).with_custom_id("req1");
    /// let batch_params = CreateMessageBatchParams::new(vec![request]);
    /// let response = client.create_message_batch(&batch_params).await?;
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

    /// List message batches
    ///
    /// List all message batches
    ///
    /// # Returns
    ///
    /// Returns a list of message batches
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
    /// use anthropic_ai_sdk::types::message_batches::{MessageBatch, MessageBatchClient, MessageBatchError};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AnthropicClient::new::<MessageBatchError>("your-api-key", "2023-06-01")?;
    /// let batches = client.list_message_batches(None).await?;
    /// println!("Batches: {:?}", batches);
    /// # Ok(())
    /// # }
    /// ```
    async fn list_message_batches<'a>(
        &'a self,
        params: Option<&'a ListMessageBatchesParams>,
    ) -> Result<ListMessageBatchesResponse, MessageBatchError> {
        self.get::<ListMessageBatchesResponse, ListMessageBatchesParams, MessageBatchError>(
            "/messages/batches",
            params,
        )
        .await
    }

    /// Retrieve a message batch
    ///
    /// Retrieve a message batch by ID
    ///
    /// # Returns
    ///
    /// Returns a message batch
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
    /// use anthropic_ai_sdk::types::message_batches::{MessageBatch, MessageBatchClient, MessageBatchError, RetrieveMessageBatchParams};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AnthropicClient::new::<MessageBatchError>("your-api-key", "2023-06-01")?;
    /// let batch = client.retrieve_message_batch(&RetrieveMessageBatchParams::new("batch_id")).await?;
    /// println!("Batch: {:?}", batch);
    /// # Ok(())
    /// # }
    /// ```
    async fn retrieve_message_batch<'a>(
        &'a self,
        params: &'a RetrieveMessageBatchParams,
    ) -> Result<RetrieveMessageBatchResponse, MessageBatchError> {
        self.get::<RetrieveMessageBatchResponse, RetrieveMessageBatchParams, MessageBatchError>(
            &format!("/messages/batches/{}", params.id),
            None,
        )
        .await
    }
}
