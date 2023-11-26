# tonapi-sdk-rs

[![Latest version](https://img.shields.io/crates/v/tonapi-sdk-rs.svg)](https://crates.io/crates/tonapi-sdk-rs)

## Overview

This is a Rust SDK generated using [OpenAPI Generator](https://openapi-generator.tech/) designed to access the [TonAPI REST endpoints](https://tonapi.io/api-v2).

## Usage

```rust
use tonapi_sdk_rs::apis::{accounts_api::get_account, accounts_api::get_account_nft_items, blockchain_api::get_blockchain_account_transactions,  configuration::Configuration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();

    let account = get_account(&config, "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0").await?;

    let nfts_items = get_account_nft_items(
        &config,
        "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0",
        None,
        Some(10),
        None,
        None,
    )
    .await?;

    let transactions = get_blockchain_account_transactions(
        &config,
        "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0",
        Some(25758317000002),
        None,
        Some(10),
    )
    .await?;

    Ok(())
}
```
