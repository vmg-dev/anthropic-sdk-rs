use crate::clients::AnthropicClient;
use crate::types::message::{
    CountMessageTokensParams, CreateMessageParams, CreateMessageResponse, MessageClient,
    MessageError,
};
use async_trait::async_trait;

#[async_trait]
impl MessageClient for AnthropicClient {
    /// Create a message
    ///
    /// Returns a list of models that are available through the API.
    /// More recently released models are listed first.
    async fn create_message<'a>(
        &'a self,
        body: Option<&'a CreateMessageParams>,
    ) -> Result<CreateMessageResponse, MessageError> {
        self.post("/messages", body).await
    }

    async fn count_tokens<'a>(
        &'a self,
        body: Option<&'a CountMessageTokensParams>,
    ) -> Result<CreateMessageResponse, MessageError> {
        self.post("/messages/count_tokens", body).await
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    // TODO: Add tests for create_message
//}
//
