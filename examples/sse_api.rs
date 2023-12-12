use tonapi::stream_api::{SseApi, SseApiConfig};

async fn subscribe_to_transactions(sse: &SseApi) -> Result<(), Box<dyn std::error::Error>> {
    let accounts = ["-1:5555555555555555555555555555555555555555555555555555555555555555"];
    let mut stream = sse.transactions_stream(Some(&accounts), None);

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

async fn subscribe_to_traces(sse: &SseApi) -> Result<(), Box<dyn std::error::Error>> {
    let accounts = ["-1:5555555555555555555555555555555555555555555555555555555555555555"];
    let mut stream = sse.traces_stream(Some(&accounts));

    while let Ok(evt) = stream.next().await {
        if let Some(evt) = evt {
            println!("Event: {}", evt.hash);
        } else {
            // Stream ended
            break;
        }
    }

    Ok(())
}

async fn subscribe_to_mempool(sse: &SseApi) -> Result<(), Box<dyn std::error::Error>> {
    let accounts = ["-1:5555555555555555555555555555555555555555555555555555555555555555"];
    let mut stream = sse.mempool_stream(Some(&accounts));

    while let Ok(evt) = stream.next().await {
        if let Some(evt) = evt {
            println!("Boc: {}", evt.boc);
        } else {
            // Stream ended
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sse_api = SseApi::new(SseApiConfig { auth_token: None });

    subscribe_to_transactions(&sse_api).await;
    subscribe_to_traces(&sse_api).await;
    subscribe_to_mempool(&sse_api).await;

    Ok(())
}
