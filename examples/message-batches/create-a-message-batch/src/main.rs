use anthropic_ai_sdk::clients::AnthropicClient;
use anthropic_ai_sdk::types::message_batches::{
    CreateMessageBatchParams, MessageBatch, MessageBatchClient, MessageBatchError,
};
use std::env;
use tracing::{error, info};

/// cd anthropic-sdk-rs/examples/messages
/// cargo run

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

    let body = CreateMessageBatchParams {
        requests: vec![MessageBatch::new_text(Role::User, "Hello, Claude")],
    };

    info!("body: {:?}", body);

    match client.create_message_batch(&body).await {
        Ok(message) => {
            info!("Successfully created message batch: {:?}", message);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
