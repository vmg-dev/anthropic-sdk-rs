//! Messages API
//!
//! This module contains the implementations for the Anthropic Messages API endpoints.
//! It provides functionality for creating messages and counting tokens.

use crate::clients::AnthropicClient;
use crate::types::message::{
    CountMessageTokensParams, CountMessageTokensResponse, CreateMessageParams,
    CreateMessageResponse, MessageClient, MessageError, StreamEvent,
};
use async_trait::async_trait;
use futures_util::{StreamExt, TryStreamExt};

use crate::clients::API_BASE_URL;

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

    // Updated implementation for create_message_streaming function that fixes the into_async_read error

    async fn create_message_streaming<'a>(
        &'a self,
        body: &'a CreateMessageParams,
    ) -> Result<
        impl futures_util::Stream<Item = Result<StreamEvent, MessageError>> + 'a,
        MessageError,
    > {
        // Ensure that stream parameter is set to true
        if body.stream.is_none() || !body.stream.unwrap() {
            return Err(MessageError::ApiError(
                "Stream parameter must be set to true for streaming".to_string(),
            ));
        }

        let url = format!("{}/messages", API_BASE_URL);

        let client = &self.get_client();
        let request = client
            .request(reqwest::Method::POST, &url)
            .header("x-api-key", &self.get_api_key())
            .json(body);

        let response = request
            .send()
            .await
            .map_err(|e| MessageError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.map_err(|e| {
                MessageError::RequestFailed(format!("Failed to read error response: {}", e))
            })?;
            return Err(MessageError::ApiError(error_text));
        }

        // Create a stream from the response body that reads line by line
        // This approach avoids using into_async_read which isn't available
        let text_stream = response.text_stream();

        // Create a stream that processes each SSE event line
        use futures_util::stream::StreamExt;

        let event_stream = text_stream
            .map(|line_result| -> Result<Option<StreamEvent>, MessageError> {
                let line = line_result.map_err(|e| MessageError::RequestFailed(e.to_string()))?;

                // Skip empty lines
                if line.trim().is_empty() {
                    return Ok(None);
                }

                // Skip the "event:" prefix lines
                if line.starts_with("event:") {
                    return Ok(None);
                }

                // Extract data from "data:" lines
                if line.starts_with("data:") {
                    let data = line["data:".len()..].trim();
                    match serde_json::from_str::<StreamEvent>(data) {
                        Ok(parsed_event) => Ok(Some(parsed_event)),
                        Err(e) => Err(MessageError::ApiError(format!(
                            "Failed to parse SSE event: {}. Event data: {}",
                            e, data
                        ))),
                    }
                } else {
                    // Skip other lines
                    Ok(None)
                }
            })
            .filter_map(|result| async move {
                match result {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(e) => Some(Err(e)),
                }
            });

        Ok(event_stream)
    }
}
