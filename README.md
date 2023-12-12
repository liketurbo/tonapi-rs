# tonapi-rs

[![Latest version](https://img.shields.io/crates/v/tonapi-sdk-rs.svg)](https://crates.io/crates/tonapi)

## Overview

This is a [TonAPI](https://tonapi.io) Rust SDK partially generated using [OpenAPI Generator](https://openapi-generator.tech/).

## Usage

### REST

```rust
use tonapi::rest_api::{
    accounts_api::{GetAccountNftItemsParams, GetAccountParams},
    blockchain_api::GetBlockchainAccountTransactionsParams,
    RestApi, RestApiConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rest_api = RestApi::new(RestApiConfig { auth_token: None });
    let account = rest_api
        .get_account(GetAccountParams {
            account_id: "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0".into(),
        })
        .await?;
    println!("Account: {}", account.balance);

    let res = rest_api
        .get_account_nft_items(GetAccountNftItemsParams {
            account_id: "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0".into(),
            collection: None,
            indirect_ownership: None,
            limit: Some(10),
            offset: None,
        })
        .await?;
    println!("Nfts len: {}", res.nft_items.len());

    let res = rest_api
        .get_blockchain_account_transactions(GetBlockchainAccountTransactionsParams {
            account_id: "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0".into(),
            after_lt: Some(25758317000002),
            before_lt: None,
            limit: Some(10),
        })
        .await?;
    println!("Transactions len: {}", res.transactions.len());

    Ok(())
}

```

## SSE

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sse_api = SseApi::new(SseApiConfig { auth_token: None });
    let mut stream = sse.transactions_stream(TransactionsStreamParams {
        accounts: Some(vec![
            "-1:5555555555555555555555555555555555555555555555555555555555555555".to_string(),
        ]),
        operations: None,
    });

    while let Ok(evt) = stream.next().await {
        if let Some(evt) = evt {
            println!("Event: {}", evt.tx_hash);
        } else {
            // Stream ended
            break;
        }
    }

    Ok(())
}
```
