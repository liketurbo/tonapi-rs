use anyhow::Context;
use async_recursion::async_recursion;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, WebSocketStream};

pub struct WebsocketApi {
    base_url: reqwest::Url,
}

impl WebsocketApi {
    pub fn new() -> Self {
        Self {
            base_url: reqwest::Url::parse("wss://tonapi.io/v2/websocket/").expect("docs url"),
        }
    }

    pub fn transactions_stream(
        &self,
        accounts_n_operations: Option<&[&str]>,
    ) -> TransactionsStream {
        if let Some(acs_n_ops) = accounts_n_operations {
            TransactionsStream::new(self.base_url.clone(), acs_n_ops)
        } else {
            TransactionsStream::new(self.base_url.clone(), &[])
        }
    }

    pub fn traces_stream(&self, accounts: Option<&[&str]>) -> TracesStream {
        if let Some(acs) = accounts {
            TracesStream::new(self.base_url.clone(), acs)
        } else {
            TracesStream::new(self.base_url.clone(), &[])
        }
    }

    pub fn mempool_stream(&self) -> MempoolStream {
        MempoolStream::new(self.base_url.clone())
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WsMethod {
    SubscribeAccount,
    SubscribeTrace,
    SubscribeMempool,
}

pub struct WsStream {
    url: reqwest::Url,
    method: WsMethod,
    params: Option<Vec<String>>,
    ws_stream: Option<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
}

impl WsStream {
    pub fn new(url: reqwest::Url, method: WsMethod, params: Option<&[&str]>) -> Self {
        if let Some(p) = params {
            Self {
                url,
                method,
                params: Some(p.into_iter().map(|s| s.to_string()).collect()),
                ws_stream: None,
            }
        } else {
            Self {
                url,
                method,
                params: None,
                ws_stream: None,
            }
        }
    }

    #[async_recursion]
    pub async fn next(&mut self) -> anyhow::Result<Option<SubscribeEvent<SubscribeEventData>>> {
        if let Some(ws_stream) = self.ws_stream.as_mut() {
            let evt = match ws_stream.next().await {
                Some(e) => e,
                None => return Ok(None),
            };
            let msg = evt.context("get next ws event")?;

            match msg {
                Message::Close(message) => {
                    let msg = message
                        .map(|fr| format!("{} {}", fr.code, fr.reason))
                        .unwrap_or("no message".to_string());
                    info!("server closed connection with {}", msg);
                    return Ok(None);
                }
                Message::Ping(payload) => {
                    debug!("ping from server with {:#?}", payload);
                    return self.next().await;
                }
                Message::Pong(payload) => {
                    debug!("ping from server with {:#?}", payload);
                    unreachable!()
                }
                Message::Binary(payload) => {
                    debug!("binary from server with {:#?}", payload);
                    return Err(anyhow::anyhow!("unexpected binary instead of text"));
                }
                Message::Text(text) => {
                    debug!("text from server with {:#?}", text);
                    let parsed_msg: SubscribeEvent<SubscribeEventData> =
                        serde_json::from_str(&text).expect("json parsing subscribed event");
                    return Ok(Some(parsed_msg));
                }
                Message::Frame(_) => {
                    debug!("raw frame from server");
                    unreachable!()
                }
            }
        } else {
            self.connect().await?;
            self.next().await
        }
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        let (ws_stream, _) = connect_async(self.url.clone()).await?;
        let sub_req = SubscribeRequest {
            // I wonder, what's purpose of id ...
            id: 1,
            jsonrpc: "2.0".to_string(),
            method: self.method.clone(),
            params: self.params.clone(),
        };

        self.ws_stream = Some(ws_stream);
        self.ws_stream
            .as_mut()
            .expect("self.ws_stream set above")
            .send(Message::Text(
                serde_json::to_string(&sub_req).expect("stringify subscribe request"),
            ))
            .await?;

        self.wait_connect().await
    }

    #[async_recursion]
    pub async fn wait_connect(&mut self) -> anyhow::Result<()> {
        let msg = self
            .ws_stream
            .as_mut()
            .expect("ws_stream set in connect(...)")
            .next()
            .await
            .context("fetch initial connect response")?
            .context("get initial connect message")?;

        match msg {
            Message::Close(message) => {
                let msg = message
                    .map(|fr| format!("{} {}", fr.code, fr.reason))
                    .unwrap_or("no message".to_string());
                error!("server closed connection with {}", msg);
                return Err(anyhow::anyhow!("closed connection before init"));
            }
            Message::Ping(payload) => {
                debug!("ping from server with {:#?}", payload);
                return self.wait_connect().await;
            }
            Message::Pong(payload) => {
                debug!("ping from server with {:#?}", payload);
                unreachable!()
            }
            Message::Binary(payload) => {
                debug!("binary from server with {:#?}", payload);
                return Err(anyhow::anyhow!("unexpected binary instead of text"));
            }

            Message::Text(text) => {
                debug!("text from server with {:#?}", text);
                let parsed_msg: SubscribeResponse =
                    serde_json::from_str(&text).expect("json parsing subscribed event");
                debug!("init connection response: {}", parsed_msg.result);
                return Ok(());
            }
            Message::Frame(fr) => {
                debug!("raw frame from server with {:#?}", fr.payload());
                unreachable!()
            }
        }
    }
}

#[derive(Serialize, Debug)]
pub struct SubscribeRequest {
    id: u64,
    // 2.0
    jsonrpc: String,
    // subscribe_account | subscribe_trace | subscribe_mempool
    method: WsMethod,
    // (account_id | account_id;operations)[]
    params: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct SubscribeEvent<P> {
    pub jsonrpc: String,
    pub method: String,
    pub params: P,
}

#[derive(Deserialize, Debug)]
pub enum SubscribeEventData {
    AccountData(TransactionEventData),
    TraceData(TraceEventData),
    MempoolData(MempoolEventData),
}

#[derive(Deserialize, Debug)]
pub struct SubscribeResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    pub result: String,
}

pub struct TransactionsStream {
    ws_stream: WsStream,
}

impl TransactionsStream {
    pub(crate) fn new(url: reqwest::Url, params: &[&str]) -> Self {
        Self {
            ws_stream: WsStream::new(url, WsMethod::SubscribeAccount, Some(params)),
        }
    }

    #[async_recursion]
    pub async fn next(&mut self) -> anyhow::Result<Option<SubscribeEvent<TransactionEventData>>> {
        let evt = self.ws_stream.next().await?;
        let evt = match evt {
            Some(e) => e,
            None => return Ok(None),
        };
        match evt.params {
            SubscribeEventData::AccountData(t_data) => Ok(Some(SubscribeEvent {
                jsonrpc: evt.jsonrpc,
                method: evt.method,
                params: TransactionEventData {
                    account_id: t_data.account_id,
                    lt: t_data.lt,
                    tx_hash: t_data.tx_hash,
                },
            })),
            _ => {
                error!("invalid event.params {:#?}", evt.params);
                Err(anyhow::anyhow!("received wrong data in params"))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TransactionEventData {
    account_id: String,
    lt: u64,
    tx_hash: String,
}

pub struct TracesStream {
    ws_stream: WsStream,
}

impl TracesStream {
    pub(crate) fn new(url: reqwest::Url, params: &[&str]) -> Self {
        Self {
            ws_stream: WsStream::new(url, WsMethod::SubscribeTrace, Some(params)),
        }
    }

    #[async_recursion]
    pub async fn next(&mut self) -> anyhow::Result<Option<SubscribeEvent<TraceEventData>>> {
        let evt = self.ws_stream.next().await?;
        let evt = match evt {
            Some(e) => e,
            None => return Ok(None),
        };
        match evt.params {
            SubscribeEventData::TraceData(t_data) => Ok(Some(SubscribeEvent {
                jsonrpc: evt.jsonrpc,
                method: evt.method,
                params: TraceEventData {
                    accounts: t_data.accounts,
                    hash: t_data.hash,
                },
            })),
            _ => {
                error!("invalid event.params {:#?}", evt.params);
                Err(anyhow::anyhow!("received wrong data in params"))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TraceEventData {
    accounts: Vec<String>,
    hash: String,
}

pub struct MempoolStream {
    ws_stream: WsStream,
}

impl MempoolStream {
    pub(crate) fn new(url: reqwest::Url) -> Self {
        Self {
            ws_stream: WsStream::new(url, WsMethod::SubscribeMempool, None),
        }
    }

    #[async_recursion]
    pub async fn next(&mut self) -> anyhow::Result<Option<SubscribeEvent<MempoolEventData>>> {
        let evt = self.ws_stream.next().await?;
        let evt = match evt {
            Some(e) => e,
            None => return Ok(None),
        };
        match evt.params {
            SubscribeEventData::MempoolData(m_data) => Ok(Some(SubscribeEvent {
                jsonrpc: evt.jsonrpc,
                method: evt.method,
                params: MempoolEventData { boc: m_data.boc },
            })),
            _ => {
                error!("invalid event.params {:#?}", evt.params);
                Err(anyhow::anyhow!("received wrong data in params"))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct MempoolEventData {
    boc: String,
}
