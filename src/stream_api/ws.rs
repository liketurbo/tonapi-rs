use anyhow::Context;
use async_recursion::async_recursion;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, WebSocketStream};

mod constants {
    include!(concat!(env!("OUT_DIR"), "/constants.rs"));
}

pub struct WsApi {
    connect_params: http::request::Parts,
}

pub struct WsApiConfig {
    pub auth_token: Option<String>,
}

impl WsApi {
    pub fn new(config: WsApiConfig) -> Self {
        let mut request = "wss://tonapi.io/v2/websocket/"
            .into_client_request()
            .expect("docs url");

        request.headers_mut().insert(
            "User-Agent",
            HeaderValue::from_static(constants::USER_AGENT),
        );

        if let Some(a_token) = config.auth_token {
            let bearer_token = format!("Bearer {}", a_token);
            request.headers_mut().insert(
                "Authorization",
                HeaderValue::from_str(&bearer_token)
                    .expect("hope users won't use some crazy auth tokens"),
            );
        }

        Self {
            connect_params: request.into_parts().0,
        }
    }

    pub fn transactions_stream(
        &self,
        accounts_n_operations: Option<&[&str]>,
    ) -> TransactionsStream {
        let acs_n_ops = accounts_n_operations.unwrap_or_else(|| &[]);
        TransactionsStream::new(&self.connect_params, acs_n_ops)
    }

    pub fn traces_stream(&self, accounts: Option<&[&str]>) -> TracesStream {
        let acs = accounts.unwrap_or_else(|| &[]);
        TracesStream::new(&self.connect_params, acs)
    }

    pub fn mempool_stream(&self) -> MempoolStream {
        MempoolStream::new(&self.connect_params)
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
    connect_request: Option<http::Request<()>>,
    subscribe_message: SubscribeMessage,
    raw_ws_stream:
        Option<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
}

impl WsStream {
    pub(crate) fn new(
        connect_params: &http::request::Parts,
        subscribe_method: WsMethod,
        subscribe_params: Option<&[&str]>,
    ) -> Self {
        let subscribe_message = SubscribeMessage {
            // I wonder, what's purpose of id ...
            id: 1,
            jsonrpc: "2.0".to_string(),
            method: subscribe_method,
            params: subscribe_params.map(|a| a.iter().map(|s| s.to_string()).collect()),
        };

        let mut new_req = http::Request::new(());
        *new_req.method_mut() = connect_params.method.clone();
        new_req.headers_mut().extend(connect_params.headers.clone());
        *new_req.uri_mut() = connect_params.uri.clone();

        Self {
            connect_request: Some(new_req),
            subscribe_message,
            raw_ws_stream: None,
        }
    }

    #[async_recursion]
    pub async fn next(&mut self) -> anyhow::Result<Option<SubscribeEvent<SubscribeEventData>>> {
        if let Some(ws_stream) = self.raw_ws_stream.as_mut() {
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
        if let Some(con_req) = self.connect_request.take() {
            let (ws_stream, _) = connect_async(con_req).await?;

            self.raw_ws_stream = Some(ws_stream);
            self.raw_ws_stream
                .as_mut()
                .expect("self.ws_stream set above")
                .send(Message::Text(
                    serde_json::to_string(&self.subscribe_message)
                        .expect("stringify subscribe request"),
                ))
                .await?;

            self.wait_connect().await
        } else {
            Err(anyhow::anyhow!("already called connect"))
        }
    }

    #[async_recursion]
    pub async fn wait_connect(&mut self) -> anyhow::Result<()> {
        let msg = self
            .raw_ws_stream
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
pub struct SubscribeMessage {
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
    pub(crate) fn new(connect_params: &http::request::Parts, subscribe_params: &[&str]) -> Self {
        Self {
            ws_stream: WsStream::new(
                connect_params,
                WsMethod::SubscribeAccount,
                Some(subscribe_params),
            ),
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
    pub account_id: String,
    pub lt: u64,
    pub tx_hash: String,
}

pub struct TracesStream {
    ws_stream: WsStream,
}

impl TracesStream {
    pub(crate) fn new(connect_params: &http::request::Parts, subscribe_params: &[&str]) -> Self {
        Self {
            ws_stream: WsStream::new(
                connect_params,
                WsMethod::SubscribeTrace,
                Some(subscribe_params),
            ),
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
    pub accounts: Vec<String>,
    pub hash: String,
}

pub struct MempoolStream {
    ws_stream: WsStream,
}

impl MempoolStream {
    pub(crate) fn new(connect_params: &http::request::Parts) -> Self {
        Self {
            ws_stream: WsStream::new(connect_params, WsMethod::SubscribeMempool, None),
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
    pub boc: String,
}
