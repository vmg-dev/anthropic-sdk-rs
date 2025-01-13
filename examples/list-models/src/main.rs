use anthropic_sdk::clients::AnthropicClient;
use anthropic_sdk::types::model::ModelClient;
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

    let client =
        AnthropicClient::new(env::var("ANTHROPIC_API_KEY").unwrap(), "2023-06-01").unwrap();

    match client.list_models(None).await {
        Ok(models) => {
            info!("Successfully retrieved models:");
            for model in models.data {
                info!("- {} ({})", model.display_name, model.id);
            }
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}
