use tonapi::stream_api::ws::{
    AccountOperations, MempoolStreamParams, TracesStreamParams, TransactionsStreamParams, WsApi,
    WsApiConfig,
};

async fn subscribe_to_transactions(ws: &WsApi) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = ws.transactions_stream(TransactionsStreamParams {
        account_operations: Some(vec![AccountOperations {
            account: "-1:5555555555555555555555555555555555555555555555555555555555555555"
                .to_string(),
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

async fn subscribe_to_traces(ws: &WsApi) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = ws.traces_stream(TracesStreamParams {
        accounts: Some(vec![
            "-1:5555555555555555555555555555555555555555555555555555555555555555".to_string(),
        ]),
    });

    while let Ok(evt) = stream.next().await {
        if let Some(evt) = evt {
            println!("Event: {}", evt.params.hash);
        } else {
            // Stream ended
            break;
        }
    }

    Ok(())
}

async fn subscribe_to_mempool(ws: &WsApi) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = ws.mempool_stream(MempoolStreamParams {
        accounts: Some(vec![
            "-1:5555555555555555555555555555555555555555555555555555555555555555".to_string(),
        ]),
    });

    while let Ok(evt) = stream.next().await {
        if let Some(evt) = evt {
            println!("Boc: {}", evt.params.boc);
        } else {
            // Stream ended
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_api = WsApi::new(WsApiConfig { auth_token: None });

    let _ = subscribe_to_transactions(&ws_api).await;
    let _ = subscribe_to_traces(&ws_api).await;
    let _ = subscribe_to_mempool(&ws_api).await;

    Ok(())
}
