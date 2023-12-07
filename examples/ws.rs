use tonapi::stream_api::ws::WebsocketApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ws_api = WebsocketApi::transactions_stream();

    let accounts = ["-1:5555555555555555555555555555555555555555555555555555555555555555"];
    // let mut stream = sse.transactions_stream(Some(&accounts), None);
    ws_api.next().await.unwrap();

    while let Ok(evt) = ws_api.next().await {
        if let Some(evt) = evt {
            println!("Event: {:?}", evt);
        } else {
            // Stream ended
            break;
        }
    }

    Ok(())
}
