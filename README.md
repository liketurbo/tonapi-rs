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
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RestApiClientV2::new(Network::Mainnet, None);

    let result = client.get_account(ACCOUNT_ID).await.unwrap();
    println!("Account balance: {}", result.balance);

    Ok(())
}
```

## SSE

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sse_api = SseApi::new(Network::Mainnet, None);

    let mut stream = sse_api.transactions_stream(
        Some(vec![
            "-1:5555555555555555555555555555555555555555555555555555555555555555".to_string(),
        ]),
        None,
    );

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
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_api = WsApi::new(Network::Mainnet, None);

    let mut stream = ws.transactions_stream(Some(vec![AccountOperations {
        account: "-1:5555555555555555555555555555555555555555555555555555555555555555".to_string(),
        operations: None
    }]));

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
