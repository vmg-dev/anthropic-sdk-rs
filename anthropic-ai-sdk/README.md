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

Add this to your `Cargo.toml`:

```
toml
[dependencies]
anthropic-ai-sdk = "0.0.1"
```

## Quick Start

```rust
use anthropic_ai_sdk::clients::AnthropicClient;
use anthropic_ai_sdk::types::model::ModelClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AnthropicClient::new::<MessageError>("ANTHROPIC_API_KEY", "ANTHROPIC_API_VERSION").unwrap();

    let body = CreateMessageParams::new(RequiredMessageParams {
        model: "claude-3-5-sonnet-20240620".to_string(),
        messages: vec![Message::new_text(Role::User, "Hello, Claude")],
        max_tokens: 1024,
    });
    Ok(())
}
```

## Examples

Check out the [examples](../examples) directory for more usage examples:

- [List Models](../examples/list-models/src/main.rs) - How to list available models
- [Count Message Tokens](../examples/count-message-tokens/src/main.rs) - How to count tokens in a message

## API Coverage

- [x] List Models
- [x] Create Message
- [x] Count Message Tokens
- [ ] Stream Message
- [ ] Upload Files
- [ ] Delete Files

## Development

### Prerequisites

- Rust 1.81 or later
- An Anthropic API key

### Running Tests

```bash
cargo test
```

### Running Examples

```basg
Set your API key
export ANTHROPIC_API_KEY="your-api-key"
Run an example
cargo run --example list-models
```


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Anthropic API Documentation](https://docs.anthropic.com/claude/reference/getting-started)
- [Rust Async Book](https://rust-lang.github.io/async-book/)

## Security

If you discover a security vulnerability within this package, please send an e-mail to the maintainers. All security vulnerabilities will be promptly addressed.

## Support

For support questions, please use the [GitHub Issues](https://github.com/yourusername/anthropic-sdk/issues).