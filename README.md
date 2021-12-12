# create-github-app-token

[![crates.io](https://img.shields.io/crates/v/create-github-app-token?style=for-the-badge)](https://crates.io/crates/create-github-app-token)
[![docs.rs](https://img.shields.io/docsrs/create-github-app-token?style=for-the-badge)](https://docs.rs/create-github-app-token)
![License](https://img.shields.io/crates/l/create-github-app-token?style=for-the-badge)

This library makes easier to publish GitHub App token.

## Usage

```rust
use create_github_app_token::{errors::Error, publish_token, Token};

async fn fetcher() -> std::result::Result<Token, Error> {
  let token = publish_token(123456, "/home/github/key.pem", "github").await?;

  Ok(token)
}
```
