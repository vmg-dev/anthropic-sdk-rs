use anthropic_ai_sdk::client::AnthropicClient;
use anthropic_ai_sdk::types::admin::api_keys::{
    AdminClient, AdminError, ApiKeyStatus, ListApiKeysParams,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), AdminError> {
    let admin_api_key = env::var("ANTHROPIC_ADMIN_KEY").expect("ANTHROPIC_ADMIN_KEY is not set");
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new_admin::<AdminError>(admin_api_key, api_version)?;

    // List all active API keys
    let params = ListApiKeysParams::new()
        .limit(10)
        .status(ApiKeyStatus::Active);

    let api_keys = client.list_api_keys(Some(&params)).await?;
    for api_key in api_keys.data {
        println!(
            "API Key: {} ({}) - Created by: {} - Hint: {}",
            api_key.name, api_key.id, api_key.created_by.id, api_key.partial_key_hint
        );
    }

    Ok(())
}
