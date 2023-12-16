# tonapi-rs

[![Latest version](https://img.shields.io/crates/v/tonapi.svg)](https://crates.io/crates/tonapi)

This is a SDK, that provides comprehensive support for interacting with the [TonAPI](https://tonapi.io).

## Features

* **Authorization Support**: You can obtain token from [tonconsole.com](https://tonconsole.com).
* **REST API Integration**: Interact with TonAPI RESTful endpoints.
* **Streaming API (SSE, WebSocket)**: Utilize Server-Sent Events (SSE) and WebSocket protocols for real-time data streaming.
* **Comparability with [tonlib-rs](https://github.com/ston-fi/tonlib-rs) Types**: TonAddress

## Usage

### REST

```rust
use tonapi::{
    rest_api::{
        accounts_api::{GetAccountNftItemsParams, GetAccountParams},
        blockchain_api::GetBlockchainAccountTransactionsParams,
        RestApi, RestApiConfig,
    },
    TonAddress,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rest_api = RestApi::new(RestApiConfig { auth_token: None });
    let account = rest_api
        .get_account(GetAccountParams {
            account_id: TonAddress::from_base64_url(
                "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0",
            )?,
        })
        .await?;
    println!("Account: {}", account.balance);

    let res = rest_api
        .get_account_nft_items(GetAccountNftItemsParams {
            account_id: TonAddress::from_base64_url(
                "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0",
            )?,
            collection: None,
            indirect_ownership: None,
            limit: Some(10),
            offset: None,
        })
        .await?;
    println!("Nfts len: {}", res.nft_items.len());

    let res = rest_api
        .get_blockchain_account_transactions(GetBlockchainAccountTransactionsParams {
            account_id: TonAddress::from_base64_url(
                "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0",
            )?,
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
use tonapi::{
    stream_api::sse::{
        SseApi, SseApiConfig, TransactionsStreamParams,
    },
    TonAddress,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sse_api = SseApi::new(SseApiConfig { auth_token: None });
    let mut stream = sse.transactions_stream(TransactionsStreamParams {
        accounts: Some(vec![TonAddress::from_hex_str(
            "-1:5555555555555555555555555555555555555555555555555555555555555555",
        )?]),
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

## WebSocket

```rust
use tonapi::{
    stream_api::ws::{
        AccountOperations, TransactionsStreamParams, WsApi, WsApiConfig,
    },
    TonAddress,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_api = WsApi::new(WsApiConfig { auth_token: None });
    let mut stream = ws.transactions_stream(TransactionsStreamParams {
        account_operations: Some(vec![AccountOperations {
            account: TonAddress::from_hex_str(
                "-1:5555555555555555555555555555555555555555555555555555555555555555",
            )?,
            operations: None,
        }]),
    });

    while let Ok(evt) = stream.next().await {
        if let Some(evt) = evt {
            println!("Event: {}", evt.params.tx_hash);
        } else {
            // Stream ended
            break;
        }
    }

    Ok(())
}

```

## Contributing

Contributions to this library is welcomed! If you'd like to contribute, please feel free to open a pull request on GitHub. Your input is highly appreciated and helps improve the SDK for the entire TON community.
