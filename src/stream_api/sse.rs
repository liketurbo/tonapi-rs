use async_recursion::async_recursion;
use futures::stream::StreamExt;
use reqwest::RequestBuilder;
use reqwest_eventsource::{Event, EventSource};
use serde::Deserialize;

pub struct SseApi {
    base_url: reqwest::Url,
    auth_token: Option<String>,
}

impl SseApi {
    pub fn new(auth_token: Option<&str>) -> Self {
        SseApi {
            base_url: reqwest::Url::parse("https://tonapi.io/v2/sse/").expect("docs url"),
            auth_token: auth_token.map(|s| s.into()),
        }
    }

    pub fn transactions_stream(
        &self,
        accounts: Option<&[&str]>,
        operations: Option<&[&str]>,
    ) -> TransactionsStream {
        let mut url = self
            .base_url
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

        if let Some(auth_token) = &self.auth_token {
            TransactionsStream::new(reqwest::Client::new().get(url).bearer_auth(auth_token))
        } else {
            TransactionsStream::new(reqwest::Client::new().get(url))
        }
    }

    pub fn traces_stream(&self, accounts: Option<&[&str]>) -> TracesStream {
        let mut url = self
            .base_url
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

        if let Some(auth_token) = &self.auth_token {
            TracesStream::new(reqwest::Client::new().get(url).bearer_auth(auth_token))
        } else {
            TracesStream::new(reqwest::Client::new().get(url))
        }
    }

    pub fn mempool_stream(&self, accounts: Option<&[&str]>) -> MempoolStream {
        let mut url = self
            .base_url
            .join("mempool")
            .expect("mempool join with base");

        if let Some(acs) = accounts {
            url.query_pairs_mut()
                .append_pair("accounts", &acs.join(","));
        }

        if let Some(auth_token) = &self.auth_token {
            MempoolStream::new(reqwest::Client::new().get(url).bearer_auth(auth_token))
        } else {
            MempoolStream::new(reqwest::Client::new().get(url))
        }
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
