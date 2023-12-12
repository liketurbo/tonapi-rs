use async_recursion::async_recursion;
use futures::stream::StreamExt;
use reqwest::RequestBuilder;
use reqwest_eventsource::{Event, EventSource};
use serde::Deserialize;

mod constants {
    include!(concat!(env!("OUT_DIR"), "/constants.rs"));
}

pub struct SseApi {
    connect_request: reqwest::Request,
}

impl SseApi {
    pub fn new(auth_token: Option<&str>) -> Self {
        let client = reqwest::Client::builder()
            .user_agent(constants::USER_AGENT)
            .build()
            .expect("build client");
        let mut builder =
            client.get(reqwest::Url::parse("https://tonapi.io/v2/sse/").expect("docs url"));

        if let Some(a_token) = auth_token {
            builder = builder.bearer_auth(a_token);
        }

        let connect_request = builder.build().expect("build request");

        SseApi { connect_request }
    }

    pub fn transactions_stream(
        &self,
        accounts: Option<&[&str]>,
        operations: Option<&[&str]>,
    ) -> TransactionsStream {
        let mut connect_request = self.connect_request.try_clone().expect("clone request");
        let url = connect_request.url_mut();

        *url = url
            .join("accounts/transactions")
            .expect("accounts/transactions join with base");

        match (accounts, operations) {
            (Some(acs), Some(ops)) => {
                url.query_pairs_mut()
                    .append_pair("accounts", &acs.join(","))
                    .append_pair("operations", &ops.join(","));
            }
            (Some(acs), None) => {
                url.query_pairs_mut()
                    .append_pair("accounts", &acs.join(","));
            }
            (None, Some(ops)) => {
                url.query_pairs_mut()
                    .append_pair("accounts", "ALL")
                    .append_pair("operations", &ops.join(","));
            }
            (None, None) => {
                url.query_pairs_mut().append_pair("accounts", "ALL");
            }
        }

        TransactionsStream::new(reqwest::RequestBuilder::from_parts(
            reqwest::Client::new(),
            connect_request,
        ))
    }

    pub fn traces_stream(&self, accounts: Option<&[&str]>) -> TracesStream {
        let mut connect_request = self.connect_request.try_clone().expect("clone request");
        let url = connect_request.url_mut();

        *url = url
            .join("accounts/traces")
            .expect("accounts/traces join with base");

        match accounts {
            Some(acs) => {
                url.query_pairs_mut()
                    .append_pair("accounts", &acs.join(","));
            }
            None => {
                url.query_pairs_mut().append_pair("accounts", "ALL");
            }
        }

        TracesStream::new(reqwest::RequestBuilder::from_parts(
            reqwest::Client::new(),
            connect_request,
        ))
    }

    pub fn mempool_stream(&self, accounts: Option<&[&str]>) -> MempoolStream {
        let mut connect_request = self.connect_request.try_clone().expect("clone request");
        let url = connect_request.url_mut();

        *url = url.join("mempool").expect("mempool join with base");

        if let Some(acs) = accounts {
            url.query_pairs_mut()
                .append_pair("accounts", &acs.join(","));
        }

        MempoolStream::new(reqwest::RequestBuilder::from_parts(
            reqwest::Client::new(),
            connect_request,
        ))
    }
}

pub struct TransactionsStream {
    es: EventSource,
}

impl TransactionsStream {
    pub(crate) fn new(builder: RequestBuilder) -> Self {
        Self {
            es: EventSource::new(builder).expect("build es"),
        }
    }

    #[async_recursion]
    pub async fn next(&mut self) -> anyhow::Result<Option<TransactionEventData>> {
        let evt = match self.es.next().await {
            Some(e) => e,
            None => return Ok(None),
        };
        let evt = evt?;

        match evt {
            Event::Open => self.next().await,
            Event::Message(e) => {
                let t: TransactionEventData = serde_json::from_str(&e.data)?;
                Ok(Some(t))
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
    es: EventSource,
}

impl TracesStream {
    pub(crate) fn new(builder: RequestBuilder) -> Self {
        Self {
            es: EventSource::new(builder).expect("build es"),
        }
    }

    #[async_recursion]
    pub async fn next(&mut self) -> anyhow::Result<Option<TraceEventData>> {
        let evt = match self.es.next().await {
            Some(e) => e,
            None => return Ok(None),
        };
        let evt = evt?;

        match evt {
            Event::Open => self.next().await,
            Event::Message(e) => {
                let t: TraceEventData = serde_json::from_str(&e.data)?;
                Ok(Some(t))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TraceEventData {
    pub account: Vec<String>,
    pub hash: String,
}

pub struct MempoolStream {
    es: EventSource,
}

impl MempoolStream {
    pub(crate) fn new(builder: RequestBuilder) -> Self {
        Self {
            es: EventSource::new(builder).expect("build es"),
        }
    }

    #[async_recursion]
    pub async fn next(&mut self) -> anyhow::Result<Option<MempoolEventData>> {
        let evt = match self.es.next().await {
            Some(e) => e,
            None => return Ok(None),
        };
        let evt = evt?;

        match evt {
            Event::Open => self.next().await,
            Event::Message(e) => {
                let t: MempoolEventData = serde_json::from_str(&e.data)?;
                Ok(Some(t))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct MempoolEventData {
    pub boc: String,
    pub involved_accounts: Vec<String>,
}
