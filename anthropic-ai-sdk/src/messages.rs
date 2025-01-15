//! Messages API
//!
//! This module contains the implementations for the Anthropic Messages API endpoints.
//! It provides functionality for creating messages and counting tokens.

use crate::clients::AnthropicClient;
use crate::types::message::{
    CountMessageTokensParams, CountMessageTokensResponse, CreateMessageParams,
    CreateMessageResponse, MessageClient, MessageError,
};
use async_trait::async_trait;

#[async_trait]
impl MessageClient for AnthropicClient {
    /// Creates a message using the specified model
    ///
    /// Creates a message with the given parameters and returns the model's response.
    /// The message can include system prompts, user messages, and other parameters
    /// that control the model's behavior.
    ///
    /// # Arguments
    ///
    /// * `body` - Parameters for creating the message, including the model to use,
    ///   the messages to send, and any additional options
    ///
    /// # Returns
    ///
    /// Returns the model's response on success, including the generated message
    /// and any additional metadata.
    ///
    /// # Errors
    ///
    /// Returns a `MessageError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::clients::AnthropicClient;
    /// use anthropic_ai_sdk::types::message::{MessageClient, MessageError};
    /// use anthropic_ai_sdk::types::message::{
    ///     CreateMessageParams, CreateMessageResponse
    /// };
    /// use tokio;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AnthropicClient::new::<MessageError>(
    ///     "your-api-key",
    ///     "2023-06-01",
    /// )?;
    ///
    /// let params = CreateMessageParams::default();
    /// let response = client.create_message(Some(&params)).await?;
    ///
    /// println!("Response: {:?}", response);
    /// # Ok(())
    /// # }
    /// ```
    async fn create_message<'a>(
        &'a self,
        body: Option<&'a CreateMessageParams>,
    ) -> Result<CreateMessageResponse, MessageError> {
        self.post("/messages", body).await
    }

    /// Counts the number of tokens in a message
    ///
    /// Returns a count of the tokens that would be used by a message with the
    /// given parameters. This can be used to ensure messages stay within token limits.
    ///
    /// # Arguments
    ///
    /// * `body` - Parameters containing the message content to count tokens for
    ///
    /// # Returns
    ///
    /// Returns the token count information on success.
    ///
    /// # Errors
    ///
    /// Returns a `MessageError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use anthropic_ai_sdk::clients::AnthropicClient;
    /// use anthropic_ai_sdk::types::message::{MessageClient, MessageError};
    /// use tokio;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # Ok(())
    /// # }
    /// ```
    async fn count_tokens<'a>(
        &'a self,
        body: Option<&'a CountMessageTokensParams>,
    ) -> Result<CountMessageTokensResponse, MessageError> {
        self.post("/messages/count_tokens", body).await
    }
}
