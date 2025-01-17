use anthropic_ai_sdk::clients::AnthropicClient;
use anthropic_ai_sdk::types::message_batches::{MessageBatchClient, MessageBatchError};
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

    match client.list_message_batches(None).await {
        Ok(batches) => {
            info!("Successfully listed message batches: {:?}", batches);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
