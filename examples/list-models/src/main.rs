use anthropic_sdk::clients::AnthropicClient;
use anthropic_sdk::types::model::ModelClient;

#[tokio::main]
async fn main() {
    let client = AnthropicClient::new(
        "sk-ant-api03-000000000000000000000000-0000000000",
        "2024-06-01",
    )
    .unwrap();

    let models = client.list_models(None).await.unwrap();
    println!("{:?}", models);
}
