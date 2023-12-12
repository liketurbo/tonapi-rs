use tonapi::stream_api::{WsApi, WsApiConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_api = WsApi::new(WsApiConfig { auth_token: None });
    let accounts = ["-1:5555555555555555555555555555555555555555555555555555555555555555"];
    let mut stream = ws_api.transactions_stream(Some(&accounts));

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
