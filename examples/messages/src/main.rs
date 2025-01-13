use anthropic_sdk::clients::AnthropicClient;
use anthropic_sdk::types::message::MessageClient;
use anthropic_sdk::types::message::MessageError;
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

    let api_key = env::var("ANTHROPIC_API_KEY").unwrap();
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new::<MessageError>(api_key, api_version).unwrap();

    match client.create_message(None).await {
        Ok(message) => {
            info!("Successfully created message: {:?}", message.content);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
