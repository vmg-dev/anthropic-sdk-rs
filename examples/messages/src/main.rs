use anthropic_sdk::clients::AnthropicClient;
use anthropic_sdk::types::message::{
    CreateMessageParams, Message, MessageClient, MessageContent, MessageError, Role,
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

    let api_key = env::var("ANTHROPIC_API_KEY").unwrap();
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new::<MessageError>(api_key, api_version).unwrap();

    let body = CreateMessageParams {
        model: "claude-3-5-sonnet-20240620".to_string(),
        messages: vec![Message {
            role: Role::User,
            content: "Hello, world!".to_string(),
        }],
        max_tokens: 1024,
        system: None,
        temperature: None,
        stop_sequences: None,
        stream: None,
        top_k: None,
        top_p: None,
        tools: None,
        tool_choice: None,
        metadata: None,
    };

    info!("body: {:?}", body);

    match client.create_message(Some(&body)).await {
        Ok(message) => {
            info!("Successfully created message: {:?}", message.content);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
