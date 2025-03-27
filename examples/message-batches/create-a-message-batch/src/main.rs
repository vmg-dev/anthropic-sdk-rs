use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::message_batches::{
    CreateMessageBatchParams, Message, MessageBatchClient, MessageBatchError, MessageRequest,
    MessageRequestParams,
};
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");

    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY is not set");
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new::<MessageBatchError>(api_key, api_version).unwrap();

    let messages = vec![Message::new("user", "Hello!")];
    let request_params = MessageRequestParams::new("claude-3-haiku", messages, 100)
        .with_system("You are a helpful assistant");

    let request = MessageRequest::new(request_params).with_custom_id("req1");

    let batch_params = CreateMessageBatchParams::new(vec![request]);
    match client.create_message_batch(&batch_params).await {
        Ok(batch) => {
            info!("Successfully created message batch: {:?}", batch);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
