# Anthropic Rust SDK

[![Crates.io](https://img.shields.io/crates/v/anthropic-ai-sdk.svg)](https://crates.io/crates/anthropic-ai-sdk)
[![Documentation](https://docs.rs/anthropic-ai-sdk/badge.svg)](https://docs.rs/anthropic-ai-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An unofficial Rust SDK for the [Anthropic API](https://docs.anthropic.com/claude/reference/getting-started).

## Features

- Full async/await support
- Comprehensive error handling
- Pagination support
- Token counting utilities

## Installation

```bash
cargo add anthropic-ai-sdk
```

## Quick Start

```rust
use anthropic_ai_sdk::clients::AnthropicClient;
use anthropic_ai_sdk::types::message::{
    CreateMessageParams, Message, MessageClient, MessageError, RequiredMessageParams, Role,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let anthropic_api_key = std::env::var("ANTHROPIC_API_KEY").unwrap();
    let client = AnthropicClient::new::<MessageError>(anthropic_api_key, "2023-06-01").unwrap();

    let body = CreateMessageParams::new(RequiredMessageParams {
        model: "claude-3-5-sonnet-20240620".to_string(),
        messages: vec![Message::new_text(Role::User, "Hello, Claude")],
        max_tokens: 1024,
    });

    match client.create_message(Some(&body)).await {
        Ok(message) => {
            println!("Successfully created message: {:?}", message.content);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    Ok(())
}
```

## Examples

Check out the [examples](https://github.com/e-bebe/anthropic-sdk-rs/tree/main/examples) directory for more usage examples:

- Modesl
  - [List Models](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/models/list-models/src/main.rs) - How to list available models
  - [Get a Models](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/models/get-a-models/src/main.rs) - How to get a model
- Messages
  - [Message](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/messages/messages/src/main.rs) - How to create a message
  - [Count Message Tokens](https://github.com/e-bebe/anthropic-sdk-rs/blob/main/examples/messages/count-message-tokens/src/main.rs) - How to count tokens in a message

## API Coverage

- Models
  - [x] List Models
  - [x] Get a Model
- Messages
  - [x] Messages
  - [x] Count Message Tokens
- Message Batches
  - [x] Retrieve a Message Batch
  - [ ] Retrieve Message Batch Results
  - [ ] List Message Batches
  - [ ] Cancel a Message Batch
  - [ ] Delete a Message Batch
- Admin API
  - Organization Member Management
    - [ ] Get User
    - [ ] List Users
    - [ ] Update User
    - [ ] Remove User
  - Organization Invites
    - [ ] Get Invite
    - [ ] List Invites
    - [ ] Create Invite
    - [ ] Delete Invite
  - Workspace Management
    - [ ] Get Workspace
    - [ ] List Workspaces
    - [ ] Update Workspace
    - [ ] Create Workspace
    - [ ] Archive Workspace
  - Workspace Member Management
    - [ ] Get Workspace Member
    - [ ] List Workspace Member
    - [ ] Get Workspace Member
    - [ ] Update Workspace Member
    - [ ] Delete Workspace Member
  - API Keys
    - [ ] Get API Key
    - [ ] List API Keys
    - [ ] Update API Keys

## Development

### Prerequisites

- Rust 1.81 or later
- An Anthropic API key

### Running Tests

```bash
cargo test
```

### Running Examples

Set your API key

```bash
export ANTHROPIC_API_KEY="your-api-key"
```

Run an example

```bash
cd examples/models/list-models
cargo run 
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Anthropic API Documentation](https://docs.anthropic.com/claude/reference/getting-started)

## Security

If you discover a security vulnerability within this package, please send an e-mail to the maintainers. All security vulnerabilities will be promptly addressed.

## Support

For support questions, please use the [GitHub Issues](https://github.com/e-bebe/anthropic-sdk-rs/issues).