use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::admin::api_keys::{
    AdminClient, AdminError, AdminUpdateApiKeyParams, ApiKeyStatus,
};
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), AdminError> {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");

    let admin_api_key = env::var("ANTHROPIC_ADMIN_KEY").expect("ANTHROPIC_ADMIN_KEY is not set");
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new_admin::<AdminError>(admin_api_key, api_version)?;

    // Get the API key ID from command line arguments
    let args: Vec<String> = env::args().collect();
    let api_key_id = args
        .get(1)
        .expect("Please provide an API key ID as argument");

    // Parse optional new name and status from command line arguments
    let new_name = if args.get(2).map(|s| s.as_str()) == Some("") || args.get(2).map(|s| s.starts_with("active") || s.starts_with("inactive") || s.starts_with("archived")).unwrap_or(false) {
        // If arg 2 is empty string or looks like a status, it's not a name
        None
    } else {
        args.get(2)
    };
    
    let new_status = args.get(3).or_else(|| {
        // Check if arg 2 is a status
        args.get(2).filter(|s| s.starts_with("active") || s.starts_with("inactive") || s.starts_with("archived"))
    }).and_then(|s| match s.as_str() {
        "active" => Some(ApiKeyStatus::Active),
        "inactive" => Some(ApiKeyStatus::Inactive),
        "archived" => Some(ApiKeyStatus::Archived),
        _ => {
            info!("Invalid status. Valid options: active, inactive, archived");
            None
        }
    });

    // Build update parameters
    let mut params = AdminUpdateApiKeyParams::new();
    if let Some(name) = new_name {
        params = params.name(name);
    }
    if let Some(status) = new_status {
        params = params.status(status);
    }

    // Check if at least one parameter is set
    if params.name.is_none() && params.status.is_none() {
        error!("Please provide at least one parameter to update: name and/or status");
        error!("Usage: cargo run -- <api_key_id> [--name <new_name>] [--status <new_status>]");
        error!("Or: cargo run -- <api_key_id> <new_status> (for status only)");
        return Ok(());
    }

    match AdminClient::update_api_key(&client, api_key_id, &params).await {
        Ok(api_key) => {
            info!("Successfully updated API key!");
            info!("  ID: {}", api_key.id);
            info!("  Name: {}", api_key.name);
            info!("  Status: {:?}", api_key.status);
            info!("  Partial Key Hint: {}", api_key.partial_key_hint);
            info!("  Created At: {}", api_key.created_at);
            info!(
                "  Created By: {} (ID: {})",
                api_key.created_by.type_, api_key.created_by.id
            );
            if let Some(workspace_id) = api_key.workspace_id {
                info!("  Workspace ID: {}", workspace_id);
            } else {
                info!("  Workspace ID: default workspace");
            }
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }

    Ok(())
}
