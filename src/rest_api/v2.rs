use super::{
    base::BaseRestApiClient,
    error::TonApiError,
    models::{
        Account, AccountEvent, AccountEvents, AccountInfo, AccountSeqno, AccountStaking, Accounts,
        AllRawShardsInfo, Auctions, AuthToken, BalanceChange, BlockchainAccountInspect,
        BlockchainBlock, BlockchainBlockShards, BlockchainBlocks, BlockchainConfig,
        BlockchainRawAccount, DecodedMessage, DnsExpiring, DnsRecord, DomainBids, DomainInfo,
        DomainNames, Dump, Event, FoundAccounts, GaslessConfig, InscriptionBalances,
        InscriptionOpTemplate, JettonBalance, JettonBalances, JettonHolders, JettonInfo,
        JettonTransferPayload, Jettons, MarketRates, MessageConsequences, MethodExecutionResult,
        Multisig, Multisigs, NftCollection, NftCollections, NftItem, NftItems, OutMsgQueueSizes,
        ParsedAddress, PublicKey, RateChart, Rates, RawAccountState, RawBlockProof,
        RawBlockchainBlock, RawBlockchainBlockHeader, RawBlockchainBlockState, RawBlockchainConfig,
        RawConfig, RawListBlockTransactions, RawMasterchainInfo, RawMasterchainInfoExt,
        RawShardBlockProof, RawShardInfo, RawTime, RawTransactions, ReducedBlocks,
        SendMessageResponse, ServiceStatus, SignRawParams, StakingPoolHistory, StakingPoolInfo,
        StakingPools, StorageProviders, Subscriptions, TonConnectPayload, Trace, TraceIds,
        Transaction, Transactions, Validators,
    },
    query_params::QueryParams,
};
use crate::Network;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_LANGUAGE},
    Client,
};

pub struct RestApiClientV2 {
    base_client: BaseRestApiClient,
}

impl RestApiClientV2 {
    pub fn new(network: Network, api_key: Option<String>) -> Self {
        let client = Client::new();
        let base_url = match network {
            Network::Mainnet => "https://tonapi.io/v2/".to_string(),
            Network::Testnet => "https://testnet.tonapi.io/v2/".to_string(),
        };
        Self {
            base_client: BaseRestApiClient::new(client, base_url, api_key),
        }
    }

    /// Get human-friendly information about several accounts without low-level details.
    ///
    /// # Parameters
    ///
    /// - `account_ids` - a list of account ids
    /// - `currency`
    ///
    /// # Returns
    ///
    /// a list of account ids
    pub async fn get_accounts(
        &self,
        account_ids: &[&str],
        currency: Option<&str>,
    ) -> Result<Accounts, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(currency) = currency {
            params.insert("currency", currency);
        }

        let body = serde_json::json!({
            "account_ids": account_ids
        });

        self.base_client
            .post_json(
                "accounts/_bulk".to_string(),
                Some(params),
                Some(&body),
                None,
            )
            .await
    }

    /// Get human-friendly information about an account without low-level details.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// account
    pub async fn get_account(&self, account_id: &str) -> Result<Account, TonApiError> {
        self.base_client
            .get(format!("accounts/{account_id}"), None, None)
            .await
    }

    /// Get account's domains.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// account's domains
    pub async fn account_dns_back_resolve(
        &self,
        account_id: &str,
    ) -> Result<DomainNames, TonApiError> {
        self.base_client
            .get(format!("accounts/{account_id}/dns/backresolve"), None, None)
            .await
    }

    /// Get all Jettons balances by owner address.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `currencies` - accept ton and all possible fiat currencies
    /// - `supported_extensions` - list of supported extensions
    ///
    /// # Returns
    ///
    /// account jettons balances
    pub async fn get_account_jettons_balances(
        &self,
        account_id: &str,
        currencies: Option<&[&str]>,
        supported_extensions: Option<&[&str]>,
    ) -> Result<JettonBalances, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(currencies) = currencies {
            params.insert("currencies", currencies);
        }

        if let Some(supported_extensions) = supported_extensions {
            params.insert("supported_extensions", supported_extensions);
        }

        self.base_client
            .get(format!("accounts/{account_id}/jettons"), Some(params), None)
            .await
    }

    /// Get Jetton balance by owner address.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `jetton_id` - jetton ID
    /// - `currencies` - accept ton and all possible fiat currencies
    ///
    /// # Returns
    ///
    /// account jetton balance
    pub async fn get_account_jetton_balance(
        &self,
        account_id: &str,
        jetton_id: &str,
        currencies: Option<&[&str]>,
    ) -> Result<JettonBalance, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(currencies) = currencies {
            params.insert("currencies", currencies);
        }

        self.base_client
            .get(
                format!("accounts/{account_id}/jettons/{jetton_id}"),
                Some(params),
                None,
            )
            .await
    }

    /// Get the transfer jettons history for account.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `accept_language` - *Default value*: en
    /// - `before_lt` - omit this parameter to get last events
    /// - `limit`
    /// - `start_date`
    /// - `end_date`
    ///
    /// # Returns
    ///
    /// account jettons history
    pub async fn get_account_jettons_history(
        &self,
        account_id: &str,
        accept_language: Option<&str>,
        before_lt: Option<i64>,
        limit: u64,
        start_date: Option<i64>,
        end_date: Option<i64>,
    ) -> Result<AccountEvents, TonApiError> {
        let mut params = QueryParams::from_pairs([("limit", limit)]);

        if let Some(before_lt) = before_lt {
            params.insert("before_lt", before_lt);
        }

        if let Some(start_date) = start_date {
            params.insert("start_date", start_date);
        }

        if let Some(end_date) = end_date {
            params.insert("end_date", end_date);
        }

        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get(
                format!("accounts/{account_id}/jettons/history"),
                Some(params),
                Some(headers),
            )
            .await
    }

    /// Get the transfer jetton history for account and jetton.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `jetton_id` - jetton ID
    /// - `accept_language` - *Default value*: en
    /// - `before_lt` - omit this parameter to get last events
    /// - `limit`
    /// - `start_date`
    /// - `end_date`
    ///
    /// # Returns
    ///
    /// account jetton history
    pub async fn get_account_jetton_history_by_id(
        &self,
        account_id: &str,
        jetton_id: &str,
        accept_language: Option<&str>,
        before_lt: Option<i64>,
        limit: u64,
        start_date: Option<i64>,
        end_date: Option<i64>,
    ) -> Result<AccountEvents, TonApiError> {
        let mut params = QueryParams::from_pairs([("limit", limit)]);

        if let Some(before_lt) = before_lt {
            params.insert("before_lt", before_lt);
        }

        if let Some(start_date) = start_date {
            params.insert("start_date", start_date);
        }

        if let Some(end_date) = end_date {
            params.insert("end_date", end_date);
        }

        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get(
                format!("accounts/{account_id}/jettons/{jetton_id}/history"),
                Some(params),
                Some(headers),
            )
            .await
    }

    /// Get all NFT items by owner address.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `collection` - nft collection
    /// - `limit`
    /// - `offset`
    /// - `indirect_ownership` - Selling nft items in ton implemented usually via transfer items to special selling account. This option enables including items which owned not directly.
    ///
    /// # Returns
    ///
    /// account nft items
    pub async fn get_account_nft_items(
        &self,
        account_id: &str,
        collection: Option<&str>,
        limit: Option<u64>,
        offset: Option<u64>,
        indirect_ownership: Option<bool>,
    ) -> Result<NftItems, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(collection) = collection {
            params.insert("collection", collection);
        }

        if let Some(limit) = limit {
            params.insert("limit", limit);
        }

        if let Some(offset) = offset {
            params.insert("offset", offset);
        }

        if let Some(indirect_ownership) = indirect_ownership {
            params.insert("indirect_ownership", indirect_ownership);
        }

        self.base_client
            .get(format!("accounts/{account_id}/nfts"), Some(params), None)
            .await
    }

    /// Get events for an account. Each event is built on top of a trace which is a series of transactions caused by one inbound message. TonAPI looks for known patterns inside the trace and splits the trace into actions, where a single action represents a meaningful high-level operation like a Jetton Transfer or an NFT Purchase. Actions are expected to be shown to users. It is advised not to build any logic on top of actions because actions can be changed at any time.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `accept_language` - *Default value*: en
    /// - `initiator` - Show only events that are initiated by this account
    /// - `subject_only` - filter actions where requested account is not real subject (for example sender or receiver jettons)
    /// - `before_lt` - omit this parameter to get last events
    /// - `limit`
    /// - `start_date`
    /// - `end_date`
    ///
    /// # Returns
    ///
    /// account's events
    pub async fn get_account_events(
        &self,
        account_id: &str,
        accept_language: Option<&str>,
        initiator: Option<bool>,
        subject_only: Option<bool>,
        before_lt: Option<i64>,
        limit: u64,
        start_date: Option<i64>,
        end_date: Option<i64>,
    ) -> Result<AccountEvents, TonApiError> {
        let mut params = QueryParams::from_pairs([("limit", limit)]);

        if let Some(initiator) = initiator {
            params.insert("initiator", initiator);
        }

        if let Some(subject_only) = subject_only {
            params.insert("subject_only", subject_only);
        }

        if let Some(before_lt) = before_lt {
            params.insert("before_lt", before_lt);
        }

        if let Some(start_date) = start_date {
            params.insert("start_date", start_date);
        }

        if let Some(end_date) = end_date {
            params.insert("end_date", end_date);
        }

        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get(format!("accounts/{account_id}/events"), Some(params), None)
            .await
    }

    /// Get event for an account by event_id.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `event_id` - event ID or transaction hash in hex (without 0x) or base64url format
    /// - `accept_language` - *Default value*: en
    /// - `subject_only` - filter actions where requested account is not real subject (for example sender or receiver jettons)
    ///
    /// # Returns
    ///
    /// account's event
    pub async fn get_account_event(
        &self,
        account_id: &str,
        event_id: &str,
        accept_language: Option<&str>,
        subject_only: Option<bool>,
    ) -> Result<AccountEvent, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(subject_only) = subject_only {
            params.insert("subject_only", subject_only);
        }

        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get(
                format!("accounts/{account_id}/events/{event_id}"),
                Some(params),
                None,
            )
            .await
    }

    /// Get traces for account.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `before_lt` - omit this parameter to get last events
    /// - `limit` - *Default value*: 100
    ///
    /// # Returns
    ///
    /// account's traces
    pub async fn get_account_traces(
        &self,
        account_id: &str,
        before_lt: Option<i64>,
        limit: Option<u64>,
    ) -> Result<TraceIds, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(before_lt) = before_lt {
            params.insert("before_lt", before_lt);
        }

        if let Some(limit) = limit {
            params.insert("limit", limit);
        }

        self.base_client
            .get(format!("accounts/{account_id}/traces"), Some(params), None)
            .await
    }

    /// Get all subscriptions by wallet address.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// account's subscriptions
    pub async fn get_account_subscriptions(
        &self,
        account_id: &str,
    ) -> Result<Subscriptions, TonApiError> {
        self.base_client
            .get(format!("accounts/{account_id}/subscriptions"), None, None)
            .await
    }

    /// Update internal cache for a particular account.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    pub async fn reindex_account(&self, account_id: &str) -> Result<(), TonApiError> {
        self.base_client
            .post_json(
                format!("accounts/{account_id}/reindex"),
                None,
                None::<serde_json::Value>,
                None,
            )
            .await
    }

    /// Search by account domain name.
    ///
    /// # Parameters
    ///
    /// - `name`
    ///
    /// # Returns
    ///
    /// found accounts
    pub async fn search_accounts(&self, name: &str) -> Result<FoundAccounts, TonApiError> {
        let params = QueryParams::from_pairs([("name", name)]);

        self.base_client
            .get(format!("accounts/search"), Some(params), None)
            .await
    }

    /// Get expiring account .ton dns.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `period` - number of days before expiration
    ///
    /// # Returns
    ///
    /// found accounts
    pub async fn get_account_dns_expiring(
        &self,
        account_id: &str,
        period: Option<u64>,
    ) -> Result<DnsExpiring, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(period) = period {
            params.insert("period", period);
        }

        self.base_client
            .get(
                format!("accounts/{account_id}/dns/expiring"),
                Some(params),
                None,
            )
            .await
    }

    /// Get public key by account id.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// account's public key
    pub async fn get_account_public_key(&self, account_id: &str) -> Result<PublicKey, TonApiError> {
        self.base_client
            .get(format!("accounts/{account_id}/publickey"), None, None)
            .await
    }

    /// Get account's multisigs.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// account's multisigs
    pub async fn get_account_multisigs(&self, account_id: &str) -> Result<Multisigs, TonApiError> {
        self.base_client
            .get(format!("accounts/{account_id}/multisigs"), None, None)
            .await
    }

    /// Get account's balance change.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `start_date`
    /// - `end_date`
    ///
    /// # Returns
    ///
    /// account's balance change
    pub async fn get_account_diff(
        &self,
        account_id: &str,
        start_date: i64,
        end_date: i64,
    ) -> Result<BalanceChange, TonApiError> {
        let params = QueryParams::from_pairs([("start_date", start_date), ("end_date", end_date)]);

        self.base_client
            .get(format!("accounts/{account_id}/diff"), Some(params), None)
            .await
    }

    /// Emulate sending message to blockchain.
    ///
    /// # Parameters
    ///
    /// - `accept_language` - *Default value*: en
    /// - `account_id` - account ID
    /// - `ignore_signature_check`
    ///
    /// # Returns
    ///
    /// emulated message to account
    pub async fn emulate_message_to_account_event(
        &self,
        boc: &str,
        accept_language: Option<&str>,
        account_id: &str,
        ignore_signature_check: Option<bool>,
    ) -> Result<AccountEvent, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(ignore_signature_check) = ignore_signature_check {
            params.insert("ignore_signature_check", ignore_signature_check);
        }

        let body = serde_json::json!({
            "boc": boc,
        });

        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .post_json(
                format!("accounts/{account_id}/events/emulate"),
                Some(params),
                Some(body),
                Some(headers),
            )
            .await
    }

    /// Get the transfer nft history.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `accept_language` - *Default value*: en
    /// - `before_lt` - omit this parameter to get last events
    /// - `limit`
    /// - `start_date`
    /// - `end_date`
    ///
    /// # Returns
    ///
    /// emulated message to account
    pub async fn get_account_nft_history(
        &self,
        account_id: &str,
        accept_language: Option<&str>,
        before_lt: Option<i64>,
        limit: u64,
        start_date: Option<i64>,
        end_date: Option<i64>,
    ) -> Result<AccountEvents, TonApiError> {
        let mut params = QueryParams::from_pairs([("limit", limit)]);

        if let Some(before_lt) = before_lt {
            params.insert("before_lt", before_lt);
        }

        if let Some(start_date) = start_date {
            params.insert("start_date", start_date);
        }

        if let Some(end_date) = end_date {
            params.insert("end_date", end_date);
        }

        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get(
                format!("accounts/{account_id}/nfts/history"),
                Some(params),
                Some(headers),
            )
            .await
    }

    /// Get NFT collections.
    ///
    /// # Parameters
    ///
    /// - `limit` - *Default value*: 100
    /// - `offset` - *Default value*: 0
    ///
    /// # Returns
    ///
    /// nft collections
    pub async fn get_nft_collections(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<NftCollections, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(limit) = limit {
            params.insert("limit", limit);
        }

        if let Some(offset) = offset {
            params.insert("offset", offset);
        }

        self.base_client
            .get("nfts/collections".to_string(), Some(params), None)
            .await
    }

    /// Get NFT collection by collection address.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// nft collection
    pub async fn get_nft_collection(&self, account_id: &str) -> Result<NftCollection, TonApiError> {
        self.base_client
            .get(format!("nfts/collections/{account_id}"), None, None)
            .await
    }

    /// Get NFT items from collection by collection address.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `limit` - *Default*: 1000
    /// - `offset` - *Default*: 0
    ///
    /// # Returns
    ///
    /// nft items
    pub async fn get_items_from_collection(
        &self,
        account_id: &str,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<NftItems, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(limit) = limit {
            params.insert("limit", limit);
        }

        if let Some(offset) = offset {
            params.insert("offset", offset);
        }

        self.base_client
            .get(
                format!("nfts/collections/{account_id}/items"),
                Some(params),
                None,
            )
            .await
    }

    /// Get NFT items by their addresses.
    ///
    /// # Parameters
    ///
    /// - `account_ids` - a list of account ids
    ///
    /// # Returns
    ///
    /// nft items
    pub async fn get_nft_items_by_addresses(
        &self,
        account_ids: &[&str],
    ) -> Result<NftItems, TonApiError> {
        let body = serde_json::json!({
            "account_ids": account_ids
        });

        self.base_client
            .post_json("nfts/_bulk".to_string(), None, Some(body), None)
            .await
    }

    /// Get NFT item by its address.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// nft item
    pub async fn get_nft_item_by_address(&self, account_id: &str) -> Result<NftItem, TonApiError> {
        self.base_client
            .get(format!("nfts/{account_id}"), None, None)
            .await
    }

    /// Get the transfer nfts history for account.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `accept_language` - *Default value*: en
    /// - `before_lt` - omit this parameter to get last events
    /// - `limit`
    /// - `start_date`
    /// - `end_date`
    ///
    /// # Returns
    ///
    /// nft history
    pub async fn get_nft_history_by_id(
        &self,
        account_id: &str,
        accept_language: Option<&str>,
        before_lt: Option<i64>,
        limit: u64,
        start_date: Option<i64>,
        end_date: Option<i64>,
    ) -> Result<AccountEvents, TonApiError> {
        let mut params = QueryParams::from_pairs([("limit", limit)]);

        if let Some(before_lt) = before_lt {
            params.insert("before_lt", before_lt);
        }

        if let Some(start_date) = start_date {
            params.insert("start_date", start_date);
        }

        if let Some(end_date) = end_date {
            params.insert("end_date", end_date);
        }

        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get(
                format!("nfts/{account_id}/history"),
                Some(params),
                Some(headers),
            )
            .await
    }

    /// Get a list of all indexed jetton masters in the blockchain.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `limit` - *Default*: 1000
    /// - `offset` - *Default*: 0
    ///
    /// # Returns
    ///
    /// a list of jettons
    pub async fn get_jettons(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Jettons, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(limit) = limit {
            params.insert("limit", limit);
        }

        if let Some(offset) = offset {
            params.insert("offset", offset);
        }

        self.base_client
            .get("jettons".to_string(), Some(params), None)
            .await
    }

    /// Get jetton metadata by jetton master address.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// jetton info
    pub async fn get_jetton_info(&self, account_id: &str) -> Result<JettonInfo, TonApiError> {
        self.base_client
            .get(format!("jettons/{account_id}"), None, None)
            .await
    }

    /// Get jetton's holders.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `limit` - *Default*: 1000
    /// - `offset` - *Default*: 0
    ///
    /// # Returns
    ///
    /// jetton's holders
    pub async fn get_jetton_holders(
        &self,
        account_id: &str,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<JettonHolders, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(limit) = limit {
            params.insert("limit", limit);
        }

        if let Some(offset) = offset {
            params.insert("offset", offset);
        }

        self.base_client
            .get(format!("jettons/{account_id}/holders"), Some(params), None)
            .await
    }

    /// Get jetton's custom payload and state init required for transfer.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `jetton_id` - jetton ID
    ///
    /// # Returns
    ///
    /// jetton's custom payload
    pub async fn get_jetton_transfer_payload(
        &self,
        account_id: &str,
        jetton_id: &str,
    ) -> Result<JettonTransferPayload, TonApiError> {
        self.base_client
            .get(
                format!("jettons/{jetton_id}/transfer/{account_id}/payload"),
                None,
                None,
            )
            .await
    }

    /// Get only jetton transfers in the event.
    ///
    /// # Parameters
    ///
    /// - `event_id` - event ID or transaction hash in hex (without 0x) or base64url format
    /// - `accept_language` - *Default value*: en
    ///
    /// # Returns
    ///
    /// events
    pub async fn get_jettons_events(
        &self,
        event_id: &str,
        accept_language: Option<&str>,
    ) -> Result<Event, TonApiError> {
        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get(format!("events/{event_id}/jettons"), None, Some(headers))
            .await
    }

    /// Get full information about domain name.
    ///
    /// # Parameters
    ///
    /// - `domain_name` - domain name with .ton or .t.me
    ///
    /// # Returns
    ///
    /// domain info
    pub async fn get_dns_info(&self, domain_name: &str) -> Result<DomainInfo, TonApiError> {
        self.base_client
            .get(format!("dns/{domain_name}"), None, None)
            .await
    }

    /// DNS resolve for domain name.
    ///
    /// # Parameters
    ///
    /// - `domain_name` - domain name with .ton or .t.me
    ///
    /// # Returns
    ///
    /// dns record
    pub async fn dns_resolve(&self, domain_name: &str) -> Result<DnsRecord, TonApiError> {
        self.base_client
            .get(format!("dns/{domain_name}/resolve"), None, None)
            .await
    }

    /// Get domain bids.
    ///
    /// # Parameters
    ///
    /// - `domain_name` - domain name with .ton or .t.me
    ///
    /// # Returns
    ///
    /// domain bids
    pub async fn get_domain_bids(&self, domain_name: &str) -> Result<DomainBids, TonApiError> {
        self.base_client
            .get(format!("dns/{domain_name}/bids"), None, None)
            .await
    }

    /// Get all auctions.
    ///
    /// # Parameters
    ///
    /// - `tld` - domain filter for current auctions "ton" or "t.me"
    ///
    /// # Returns
    ///
    /// auctions
    pub async fn get_all_auctions(&self, tld: &str) -> Result<Auctions, TonApiError> {
        let params = QueryParams::from_pairs([("tld", tld)]);

        self.base_client
            .get("dns/auctions".to_string(), Some(params), None)
            .await
    }

    /// Get backup info.
    ///
    /// # Parameters
    ///
    /// - `x_ton_connect_auth`
    ///
    /// # Returns
    ///
    /// get wallet dump
    pub async fn get_wallet_backup(&self, x_ton_connect_auth: &str) -> Result<Dump, TonApiError> {
        let mut headers = HeaderMap::new();

        headers.insert(
            "X-TonConnect-Auth",
            HeaderValue::from_str(x_ton_connect_auth)?,
        );

        self.base_client
            .get("wallet/backup".to_string(), None, Some(headers))
            .await
    }

    /// Set backup info.
    ///
    /// # Parameters
    ///
    /// - `backup`
    /// - `x_ton_connect_auth`
    pub async fn set_wallet_backup(
        &self,
        backup: Vec<u8>,
        x_ton_connect_auth: &str,
    ) -> Result<(), TonApiError> {
        let mut headers = HeaderMap::new();

        headers.insert(
            "X-TonConnect-Auth",
            HeaderValue::from_str(x_ton_connect_auth)?,
        );

        self.base_client
            .put_bytes(
                "wallet/backup".to_string(),
                None,
                Some(backup),
                Some(headers),
            )
            .await
    }

    /// Account verification and token issuance.
    ///
    /// # Parameters
    ///
    /// - `ton_connect` - Data that is expected from TON Connect
    ///
    /// # Returns
    ///
    /// auth token
    pub async fn ton_connect_proof(
        &self,
        ton_connect: serde_json::Value,
    ) -> Result<AuthToken, TonApiError> {
        self.base_client
            .post_json(
                "wallet/auth/proof".to_string(),
                None,
                Some(ton_connect),
                None,
            )
            .await
    }

    /// Get account seqno.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// account seqno
    pub async fn get_account_seqno(&self, account_id: &str) -> Result<AccountSeqno, TonApiError> {
        self.base_client
            .get(format!("wallet/{account_id}/seqno"), None, None)
            .await
    }

    /// Get wallets by public key.
    ///
    /// # Parameters
    ///
    /// - `public_key`
    ///
    /// # Returns
    ///
    /// a list of wallets
    pub async fn get_wallets_by_public_key(
        &self,
        public_key: &str,
    ) -> Result<Accounts, TonApiError> {
        self.base_client
            .get(format!("pubkeys/{public_key}/wallets"), None, None)
            .await
    }

    /// Emulate sending message to blockchain.
    ///
    /// # Parameters
    ///
    /// - `input` - bag-of-cells serialized to base64/hex and additional parameters to configure emulation
    /// - `accept_language` - *Default value*: en
    ///
    /// # Returns
    ///
    /// a list of wallets
    pub async fn emulate_message_to_wallet(
        &self,
        input: serde_json::Value,
        accept_language: Option<&str>,
    ) -> Result<MessageConsequences, TonApiError> {
        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .post_json(
                "wallet/emulate".to_string(),
                None,
                Some(input),
                Some(headers),
            )
            .await
    }

    /// Get the token price in the chosen currency for display only. Don’t use this for financial transactions.
    ///
    /// # Parameters
    ///
    /// - `tokens` - accept ton and jetton master addresses
    /// - `currencies` - accept ton and all possible fiat currencies
    ///
    /// # Returns
    ///
    /// tokens rates
    pub async fn get_rates(
        &self,
        tokens: &[&str],
        currencies: &[&str],
    ) -> Result<Rates, TonApiError> {
        let params = QueryParams::from_pairs([("tokens", tokens), ("currencies", currencies)]);

        self.base_client
            .get("rates".to_string(), Some(params), None)
            .await
    }

    /// Get chart by token.
    ///
    /// # Parameters
    ///
    /// - `token` - accept jetton master address
    /// - `currency`
    /// - `start_date`
    /// - `end_date`
    /// - `points_count`
    ///
    /// # Returns
    ///
    /// token chart
    pub async fn get_chart_rates(
        &self,
        token: &str,
        currency: Option<&str>,
        start_date: Option<i64>,
        end_date: Option<i64>,
        points_count: Option<u64>,
    ) -> Result<RateChart, TonApiError> {
        let mut params = QueryParams::from_pairs([("token", token)]);

        if let Some(currency) = currency {
            params.insert("currency", currency);
        }

        if let Some(start_date) = start_date {
            params.insert("start_date", start_date);
        }

        if let Some(end_date) = end_date {
            params.insert("end_date", end_date);
        }

        if let Some(points_count) = points_count {
            params.insert("points_count", points_count);
        }

        self.base_client
            .get("rates/chart".to_string(), Some(params), None)
            .await
    }

    /// Get the TON price from markets.
    ///
    /// # Returns
    ///
    /// markets rates
    pub async fn get_markets_rates(&self) -> Result<MarketRates, TonApiError> {
        self.base_client
            .get("rates/markets".to_string(), None, None)
            .await
    }

    /// All pools where account participates.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// account's pools
    pub async fn get_account_nominators_pools(
        &self,
        account_id: &str,
    ) -> Result<AccountStaking, TonApiError> {
        self.base_client
            .get(format!("staking/nominator/{account_id}/pools"), None, None)
            .await
    }

    /// Staking pool info.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `accept_language` - *Default value*: en
    ///
    /// # Returns
    ///
    /// staking pool info
    pub async fn get_staking_pool_info(
        &self,
        account_id: &str,
        accept_language: Option<&str>,
    ) -> Result<StakingPoolInfo, TonApiError> {
        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get(format!("staking/pool/{account_id}"), None, Some(headers))
            .await
    }

    /// Pool history.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// pool history
    pub async fn get_staking_pool_history(
        &self,
        account_id: &str,
    ) -> Result<StakingPoolHistory, TonApiError> {
        self.base_client
            .get(format!("staking/pool/{account_id}/history"), None, None)
            .await
    }

    /// All pools available in the network.
    ///
    /// # Parameters
    ///
    /// - `available_for` - account ID
    /// - `include_unverified` - return also pools not from white list - just compatible by interfaces (maybe dangerous!)
    /// - `accept_language` - *Default value*: en
    ///
    /// # Returns
    ///
    /// a list of pools
    pub async fn get_staking_pools(
        &self,
        available_for: Option<&str>,
        include_unverified: Option<bool>,
        accept_language: Option<&str>,
    ) -> Result<StakingPools, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(account_id) = available_for {
            params.insert("available_for", account_id);
        }

        if let Some(include) = include_unverified {
            params.insert("include_unverified", include);
        }

        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get("staking/pools".to_string(), Some(params), Some(headers))
            .await
    }

    /// Get the trace by trace ID or hash of any transaction in the trace.
    ///
    /// # Parameters
    ///
    /// - `trace_id` - trace ID or transaction hash in hex (without 0x) or base64url format
    ///
    /// # Returns
    ///
    /// trace
    pub async fn get_trace(&self, trace_id: &str) -> Result<Trace, TonApiError> {
        self.base_client
            .get(format!("traces/{}", trace_id), None, None)
            .await
    }

    /// Emulate sending message to blockchain.
    ///
    /// # Parameters
    ///
    /// - `boc` - bag-of-cells serialized to hex
    /// - `ignore_signature_check`
    ///
    /// # Returns
    ///
    /// emulated trace
    pub async fn emulate_message_to_trace(
        &self,
        boc: &str,
        ignore_signature_check: Option<bool>,
    ) -> Result<Trace, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(ignore_check) = ignore_signature_check {
            params.insert("ignore_signature_check", ignore_check);
        }

        let body = serde_json::json!({
            "boc": boc
        });

        self.base_client
            .post_json("traces/emulate".to_string(), Some(params), Some(body), None)
            .await
    }

    /// Get an event by event ID or a hash of any transaction in a trace.
    ///
    /// An event is built on top of a trace, which is a series of transactions caused by one inbound message.
    /// The event represents meaningful high-level operations like a Jetton Transfer or an NFT Purchase.
    ///
    /// # Parameters
    ///
    /// - `event_id` - event ID or transaction hash in hex (without 0x) or base64url format
    /// - `accept_language` - *Default value*: en
    ///
    /// # Returns
    ///
    /// event
    pub async fn get_event(
        &self,
        event_id: &str,
        accept_language: Option<&str>,
    ) -> Result<Event, TonApiError> {
        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get(format!("events/{}", event_id), None, Some(headers))
            .await
    }

    /// Emulate sending message to blockchain.
    ///
    /// # Parameters
    ///
    /// - `boc` - bag-of-cells serialized to hex
    /// - `accept_language` - *Default value*: en
    /// - `ignore_signature_check`
    ///
    /// # Returns
    ///
    /// emulated event
    pub async fn emulate_message_to_event(
        &self,
        boc: &str,
        ignore_signature_check: Option<bool>,
        accept_language: Option<&str>,
    ) -> Result<Event, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(ignore_check) = ignore_signature_check {
            params.insert("ignore_signature_check", ignore_check);
        }

        let body = serde_json::json!({
            "boc": boc
        });

        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .post_json(
                "events/emulate".to_string(),
                Some(params),
                Some(body),
                Some(headers),
            )
            .await
    }

    /// Get TON storage providers deployed to the blockchain.
    ///
    /// # Returns
    ///
    /// A list of storage providers
    pub async fn get_storage_providers(&self) -> Result<StorageProviders, TonApiError> {
        self.base_client
            .get("storage/providers".to_string(), None, None)
            .await
    }

    /// Get a payload for further token receipt.
    ///
    /// # Returns
    ///
    /// payload
    pub async fn get_ton_connect_payload(&self) -> Result<TonConnectPayload, TonApiError> {
        self.base_client
            .get("tonconnect/payload".to_string(), None, None)
            .await
    }

    /// Get account info by state init.
    ///
    /// # Parameters
    ///
    /// - `state_init` - Data that is expected, base64
    ///
    /// # Returns
    ///
    /// account info
    pub async fn get_account_info_by_state_init(
        &self,
        state_init: &str,
    ) -> Result<AccountInfo, TonApiError> {
        let body = serde_json::json!({
            "state_init": state_init
        });

        self.base_client
            .post_json("tonconnect/stateinit".to_string(), None, Some(body), None)
            .await
    }

    /// Returns configuration of gasless transfers.
    ///
    /// # Returns
    ///
    /// gasless config
    pub async fn get_gasless_config(&self) -> Result<GaslessConfig, TonApiError> {
        self.base_client
            .get("gasless/config".to_string(), None, None)
            .await
    }

    /// Estimates the cost of the given messages and returns a payload to sign.
    ///
    /// # Parameters
    ///
    /// - `master_id` - The jetton address to pay commission
    /// - `wallet_address`
    /// - `wallet_public_key`
    /// - `messages`
    ///
    /// # Returns
    ///
    /// A `SignRawParams` object containing the payload to sign
    pub async fn gasless_estimate(
        &self,
        master_id: &str,
        wallet_address: &str,
        wallet_public_key: &str,
        messages: &[&str],
    ) -> Result<SignRawParams, TonApiError> {
        let body = serde_json::json!({
            "wallet_address": wallet_address,
            "wallet_public_key": wallet_public_key,
            "messages": messages
        });

        self.base_client
            .post_json(
                format!("gasless/estimate/{master_id}"),
                None,
                Some(body),
                None,
            )
            .await
    }

    /// Submits the signed gasless transaction message to the network.
    ///
    /// # Parameters
    ///
    /// - `wallet_public_key`
    /// - `boc`
    ///
    /// # Returns
    ///
    /// the message has been sent
    pub async fn gasless_send(
        &self,
        wallet_public_key: &str,
        boc: &str,
    ) -> Result<(), TonApiError> {
        let body = serde_json::json!({
            "wallet_public_key": wallet_public_key,
            "boc": boc
        });

        self.base_client
            .post_json("gasless/send".to_string(), None, Some(body), None)
            .await
    }

    /// Get multisig account info.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// multisig account
    pub async fn get_multisig_account(&self, account_id: &str) -> Result<Multisig, TonApiError> {
        self.base_client
            .get(format!("multisig/{account_id}"), None, None)
            .await
    }

    /// Get reduced blockchain blocks data.
    ///
    /// # Parameters
    ///
    /// - `from`
    /// - `to`
    ///
    /// # Returns
    ///
    /// blockchain reduced blocks
    pub async fn get_reduced_blockchain_blocks(
        &self,
        from: i64,
        to: i64,
    ) -> Result<ReducedBlocks, TonApiError> {
        let params = QueryParams::from_pairs([("from", from), ("to", to)]);

        self.base_client
            .get("blockchain/reduced/blocks".to_string(), Some(params), None)
            .await
    }

    /// Get blockchain block data.
    ///
    /// # Parameters
    ///
    /// - `block_id` - block ID
    ///
    /// # Returns
    ///
    /// blockchain block
    pub async fn get_blockchain_block(
        &self,
        block_id: &str,
    ) -> Result<BlockchainBlock, TonApiError> {
        self.base_client
            .get(format!("blockchain/blocks/{block_id}"), None, None)
            .await
    }

    /// Get blockchain block shards.
    ///
    /// # Parameters
    ///
    /// - `masterchain_seqno` - masterchain block seqno
    ///
    /// # Returns
    ///
    /// blockchain block shards
    pub async fn get_blockchain_masterchain_shards(
        &self,
        masterchain_seqno: i32,
    ) -> Result<BlockchainBlockShards, TonApiError> {
        self.base_client
            .get(
                format!("blockchain/masterchain/{masterchain_seqno}/shards"),
                None,
                None,
            )
            .await
    }

    /// Get all blocks in all shards and workchains between target and previous masterchain block according to shards last blocks snapshot in masterchain. We don't recommend to build your app around this method because it has problem with scalability and will work very slow in the future.
    ///
    /// # Parameters
    ///
    /// - `masterchain_seqno` - masterchain block seqno
    ///
    /// # Returns
    ///
    /// blockchain blocks
    pub async fn get_blockchain_masterchain_blocks(
        &self,
        masterchain_seqno: i32,
    ) -> Result<BlockchainBlocks, TonApiError> {
        self.base_client
            .get(
                format!("blockchain/masterchain/{masterchain_seqno}/blocks"),
                None,
                None,
            )
            .await
    }

    /// Get all transactions in all shards and workchains between target and previous masterchain block according to shards last blocks snapshot in masterchain. We don't recommend to build your app around this method because it has problem with scalability and will work very slow in the future.
    ///
    /// # Parameters
    ///
    /// - `masterchain_seqno` - masterchain block seqno
    ///
    /// # Returns
    ///
    /// blockchain transactions
    pub async fn get_blockchain_masterchain_transactions(
        &self,
        masterchain_seqno: i32,
    ) -> Result<Transactions, TonApiError> {
        self.base_client
            .get(
                format!("blockchain/masterchain/{masterchain_seqno}/transactions"),
                None,
                None,
            )
            .await
    }

    /// Get blockchain config from a specific block, if present.
    ///
    /// # Parameters
    ///
    /// - `masterchain_seqno` - masterchain block seqno
    ///
    /// # Returns
    ///
    /// blockchain config
    pub async fn get_blockchain_config_from_block(
        &self,
        masterchain_seqno: i32,
    ) -> Result<BlockchainConfig, TonApiError> {
        self.base_client
            .get(
                format!("blockchain/masterchain/{masterchain_seqno}/config"),
                None,
                None,
            )
            .await
    }

    /// Get raw blockchain config from a specific block, if present.
    ///
    /// # Parameters
    ///
    /// - `masterchain_seqno` - masterchain block seqno
    ///
    /// # Returns
    ///
    /// blockchain config
    pub async fn get_raw_blockchain_config_from_block(
        &self,
        masterchain_seqno: i32,
    ) -> Result<RawBlockchainConfig, TonApiError> {
        self.base_client
            .get(
                format!("blockchain/masterchain/{masterchain_seqno}/config/raw"),
                None,
                None,
            )
            .await
    }

    /// Get transactions from block.
    ///
    /// # Parameters
    ///
    /// - `block_id` - block ID
    ///
    /// # Returns
    ///
    /// blockchain block transactions
    pub async fn get_blockchain_block_transactions(
        &self,
        block_id: &str,
    ) -> Result<Transactions, TonApiError> {
        self.base_client
            .get(
                format!("blockchain/blocks/{block_id}/transactions"),
                None,
                None,
            )
            .await
    }

    /// Get transaction data.
    ///
    /// # Parameters
    ///
    /// - `transaction_id` - transaction ID
    ///
    /// # Returns
    ///
    /// blockchain transaction
    pub async fn get_blockchain_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<Transaction, TonApiError> {
        self.base_client
            .get(
                format!("blockchain/transactions/{transaction_id}"),
                None,
                None,
            )
            .await
    }

    /// Get transaction data by message hash.
    ///
    /// # Parameters
    ///
    /// - `msg_id` - message ID
    ///
    /// # Returns
    ///
    /// transaction by message hash
    pub async fn get_blockchain_transaction_by_message_hash(
        &self,
        msg_id: &str,
    ) -> Result<Transaction, TonApiError> {
        self.base_client
            .get(
                format!("blockchain/messages/{msg_id}/transaction"),
                None,
                None,
            )
            .await
    }

    /// Get blockchain validators.
    ///
    /// # Returns
    ///
    /// blockchain validators
    pub async fn get_blockchain_validators(&self) -> Result<Validators, TonApiError> {
        self.base_client
            .get("blockchain/validators".to_string(), None, None)
            .await
    }

    /// Get the last known masterchain block.
    ///
    /// # Returns
    ///
    /// blockchain masterchain head
    pub async fn get_blockchain_masterchain_head(&self) -> Result<BlockchainBlock, TonApiError> {
        self.base_client
            .get("blockchain/masterchain-head".to_string(), None, None)
            .await
    }

    /// Get low-level information about an account taken directly from the blockchain.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// raw account
    pub async fn get_blockchain_raw_account(
        &self,
        account_id: &str,
    ) -> Result<BlockchainRawAccount, TonApiError> {
        self.base_client
            .get(format!("blockchain/accounts/{account_id}"), None, None)
            .await
    }

    /// Get account transactions.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `after_lt` - omit this parameter to get last transactions
    /// - `before_lt` - omit this parameter to get last transactions
    /// - `limit` - *Default value*: 100
    /// - `sort_order` - *Available values*: desc, asc *Default value*: desc
    ///
    /// # Returns
    ///
    /// blockchain account transactions
    pub async fn get_blockchain_account_transactions(
        &self,
        account_id: &str,
        after_lt: Option<i64>,
        before_lt: Option<i64>,
        limit: Option<u64>,
        sort_order: Option<&str>,
    ) -> Result<Transactions, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(after_lt) = after_lt {
            params.insert("after_lt", after_lt);
        }

        if let Some(before_lt) = before_lt {
            params.insert("before_lt", before_lt);
        }

        if let Some(limit) = limit {
            params.insert("limit", limit);
        }

        if let Some(sort_order) = sort_order {
            params.insert("sort_order", sort_order);
        }

        self.base_client
            .get(
                format!("blockchain/accounts/{account_id}/transactions"),
                Some(params),
                None,
            )
            .await
    }

    /// Execute get method for account.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `method_name` - contract get method name
    /// - `args`
    ///
    /// # Returns
    ///
    /// method execution result
    pub async fn exec_get_method_for_blockchain_account(
        &self,
        account_id: &str,
        method_name: &str,
        args: Option<&[&str]>,
    ) -> Result<MethodExecutionResult, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(args) = args {
            params.insert("args", args);
        }

        self.base_client
            .get(
                format!("blockchain/accounts/{account_id}/methods/{method_name}"),
                Some(params),
                None,
            )
            .await
    }

    /// Send a message to blockchain.
    ///
    /// # Parameters
    ///
    /// - `boc` - both a single boc and a batch of boc serialized in base64/hex are accepted
    /// - `batch` - both a single boc and a batch of boc serialized in base64/hex are accepted
    pub async fn send_blockchain_message(
        &self,
        boc: &str,
        batch: &[&str],
    ) -> Result<(), TonApiError> {
        let body = serde_json::json!({
            "boc": boc,
            "batch": batch
        });

        self.base_client
            .post_json("blockchain/message".to_string(), None, Some(body), None)
            .await
    }

    /// Get the blockchain configuration.
    ///
    /// # Returns
    ///
    /// blockchain config
    pub async fn get_blockchain_config(&self) -> Result<BlockchainConfig, TonApiError> {
        self.base_client
            .get("blockchain/config".to_string(), None, None)
            .await
    }

    /// Get the raw blockchain configuration.
    ///
    /// # Returns
    ///
    /// blockchain config
    pub async fn get_raw_blockchain_config(&self) -> Result<RawBlockchainConfig, TonApiError> {
        self.base_client
            .get("blockchain/config/raw".to_string(), None, None)
            .await
    }

    /// Blockchain account inspect.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// blockchain account inspect
    pub async fn blockchain_account_inspect(
        &self,
        account_id: &str,
    ) -> Result<BlockchainAccountInspect, TonApiError> {
        self.base_client
            .get(
                format!("blockchain/accounts/{account_id}/inspect"),
                None,
                None,
            )
            .await
    }

    /// Get raw masterchain info.
    ///
    /// # Returns
    ///
    /// raw masterchain info
    pub async fn get_raw_masterchain_info(&self) -> Result<RawMasterchainInfo, TonApiError> {
        self.base_client
            .get("liteserver/get_masterchain_info".to_string(), None, None)
            .await
    }

    /// Get raw masterchain info ext.
    ///
    /// # Parameters
    ///
    /// - `mode` - mode
    ///
    /// # Returns
    ///
    /// raw masterchain info ext
    pub async fn get_raw_masterchain_info_ext(
        &self,
        mode: i32,
    ) -> Result<RawMasterchainInfoExt, TonApiError> {
        let params = QueryParams::from_pairs([("mode", mode)]);

        self.base_client
            .get(
                "liteserver/get_masterchain_info_ext".to_string(),
                Some(params),
                None,
            )
            .await
    }

    /// Get raw time.
    ///
    /// # Returns
    ///
    /// raw time
    pub async fn get_raw_time(&self) -> Result<RawTime, TonApiError> {
        self.base_client
            .get("liteserver/get_time".to_string(), None, None)
            .await
    }

    /// Get raw blockchain block.
    ///
    /// # Parameters
    ///
    /// - `block_id` - block ID: (workchain,shard,seqno,root_hash,file_hash)
    ///
    /// # Returns
    ///
    /// raw blockchain block
    pub async fn get_raw_blockchain_block(
        &self,
        block_id: &str,
    ) -> Result<RawBlockchainBlock, TonApiError> {
        self.base_client
            .get(format!("liteserver/get_block/{block_id}"), None, None)
            .await
    }

    /// Get raw blockchain block state.
    ///
    /// # Parameters
    ///
    /// - `block_id` - block ID: (workchain,shard,seqno,root_hash,file_hash)
    ///
    /// # Returns
    ///
    /// raw blockchain block state
    pub async fn get_raw_blockchain_block_state(
        &self,
        block_id: &str,
    ) -> Result<RawBlockchainBlockState, TonApiError> {
        self.base_client
            .get(format!("liteserver/get_state/{block_id}"), None, None)
            .await
    }

    /// Get raw blockchain block header.
    ///
    /// # Parameters
    ///
    /// - `block_id` - block ID: (workchain,shard,seqno,root_hash,file_hash)
    /// - `mode` - mode
    ///
    /// # Returns
    ///
    /// raw blockchain block header
    pub async fn get_raw_blockchain_block_header(
        &self,
        block_id: &str,
        mode: i32,
    ) -> Result<RawBlockchainBlockHeader, TonApiError> {
        let params = QueryParams::from_pairs([("mode", mode)]);

        self.base_client
            .get(
                format!("liteserver/get_block_header/{block_id}"),
                Some(params),
                None,
            )
            .await
    }

    /// Send raw message to blockchain.
    ///
    /// # Parameters
    ///
    /// - `body` - Data that is expected
    ///
    /// # Returns
    ///
    /// code
    pub async fn send_raw_message(&self, body: &str) -> Result<SendMessageResponse, TonApiError> {
        let body = serde_json::json!({
            "body": body,
        });

        self.base_client
            .post_json(
                "liteserver/send_message".to_string(),
                None,
                Some(body),
                None,
            )
            .await
    }

    /// Get raw account state.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `target_block_id` - target block: (workchain,shard,seqno,root_hash,file_hash)
    ///
    /// # Returns
    ///
    /// raw account state
    pub async fn get_raw_account_state(
        &self,
        account_id: &str,
        target_block_id: Option<&str>,
    ) -> Result<RawAccountState, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(target_block_id) = target_block_id {
            params.insert("target_block_id", target_block_id);
        }

        self.base_client
            .get(
                format!("liteserver/get_account_state/{account_id}"),
                Some(params),
                None,
            )
            .await
    }

    /// Get raw shard info.
    ///
    /// # Parameters
    ///
    /// - `block_id` - block ID: (workchain,shard,seqno,root_hash,file_hash)
    /// - `workchain` - workchain
    /// - `shard` - shard
    /// - `exact` - exact
    ///
    /// # Returns
    ///
    /// raw shard info
    pub async fn get_raw_shard_info(
        &self,
        block_id: &str,
        workchain: Option<i32>,
        shard: Option<&str>,
        exact: Option<bool>,
    ) -> Result<RawShardInfo, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(workchain) = workchain {
            params.insert("workchain", workchain);
        }

        if let Some(shard) = shard {
            params.insert("shard", shard);
        }

        if let Some(exact) = exact {
            params.insert("exact", exact);
        }

        self.base_client
            .get(
                format!("liteserver/get_shard_info/{block_id}"),
                Some(params),
                None,
            )
            .await
    }

    /// Get all raw shards info.
    ///
    /// # Parameters
    ///
    /// - `block_id` - block ID: (workchain,shard,seqno,root_hash,file_hash)
    ///
    /// # Returns
    ///
    /// all raw shards info
    pub async fn get_all_raw_shards_info(
        &self,
        block_id: &str,
    ) -> Result<AllRawShardsInfo, TonApiError> {
        self.base_client
            .get(
                format!("liteserver/get_all_shards_info/{block_id}"),
                None,
                None,
            )
            .await
    }

    /// Get raw transactions.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `count` - count
    /// - `lt` - lt
    /// - `hash` - hash
    ///
    /// # Returns
    ///
    /// raw transactions
    pub async fn get_raw_transactions(
        &self,
        account_id: &str,
        count: i32,
        lt: i64,
        hash: &str,
    ) -> Result<RawTransactions, TonApiError> {
        let params = QueryParams::from_pairs([
            ("count", count.to_string().as_str()),
            ("lt", lt.to_string().as_str()),
            ("hash", hash),
        ]);

        self.base_client
            .get(
                format!("liteserver/get_transactions/{account_id}"),
                Some(params),
                None,
            )
            .await
    }

    /// Get raw list block transactions.
    ///
    /// # Parameters
    ///
    /// - `block_id` - block ID: (workchain,shard,seqno,root_hash,file_hash)
    /// - `mode` - mode
    /// - `count` - count
    /// - `account_id` - account ID
    /// - `lt` - lt
    ///
    /// # Returns
    ///
    /// a list of raw block transactions
    pub async fn get_raw_list_block_transactions(
        &self,
        block_id: &str,
        mode: i32,
        count: i32,
        account_id: Option<&str>,
        lt: Option<i64>,
    ) -> Result<RawListBlockTransactions, TonApiError> {
        let mut params = QueryParams::from_pairs([("mode", mode), ("count", count)]);

        if let Some(account_id) = account_id {
            params.insert("account_id", account_id);
        }

        if let Some(lt) = lt {
            params.insert("lt", lt);
        }

        self.base_client
            .get(
                format!("liteserver/list_block_transactions/{block_id}"),
                Some(params),
                None,
            )
            .await
    }

    /// Get raw block proof.
    ///
    /// # Parameters
    ///
    /// - `known_block_id` - known block: (workchain,shard,seqno,root_hash,file_hash)
    /// - `target_block_id` - target block: (workchain,shard,seqno,root_hash,file_hash)
    /// - `mode` - mode
    ///
    /// # Returns
    ///
    /// raw block proof
    pub async fn get_raw_block_proof(
        &self,
        known_block_id: &str,
        target_block_id: Option<&str>,
        mode: i32,
    ) -> Result<RawBlockProof, TonApiError> {
        let mut params = QueryParams::from_pairs([
            ("known_block_id", known_block_id),
            ("mode", mode.to_string().as_str()),
        ]);

        if let Some(target_block_id) = target_block_id {
            params.insert("target_block_id", target_block_id);
        }

        self.base_client
            .get("liteserver/get_block_proof".to_string(), Some(params), None)
            .await
    }

    /// Get raw config.
    ///
    /// # Parameters
    ///
    /// - `block_id` - block ID: (workchain,shard,seqno,root_hash,file_hash)
    /// - `mode` - mode
    ///
    /// # Returns
    ///
    /// A `RawConfig` object containing the raw config data
    pub async fn get_raw_config(
        &self,
        block_id: &str,
        mode: i32,
    ) -> Result<RawConfig, TonApiError> {
        let params = QueryParams::from_pairs([("mode", mode)]);

        self.base_client
            .get(
                format!("liteserver/get_config_all/{block_id}"),
                Some(params),
                None,
            )
            .await
    }

    /// Get raw shard block proof.
    ///
    /// # Parameters
    ///
    /// - `block_id` - block ID: (workchain,shard,seqno,root_hash,file_hash)
    ///
    /// # Returns
    ///
    /// raw shard block proof
    pub async fn get_raw_shard_block_proof(
        &self,
        block_id: &str,
    ) -> Result<RawShardBlockProof, TonApiError> {
        self.base_client
            .get(
                format!("liteserver/get_shard_block_proof/{block_id}"),
                None,
                None,
            )
            .await
    }

    /// Get out msg queue sizes.
    ///
    /// # Returns
    ///
    /// out msg queue sizes
    pub async fn get_out_msg_queue_sizes(&self) -> Result<OutMsgQueueSizes, TonApiError> {
        self.base_client
            .get("liteserver/get_out_msg_queue_sizes".to_string(), None, None)
            .await
    }

    /// Decode a given message. Only external incoming messages can be decoded currently.
    ///
    /// # Parameters
    ///
    /// - `boc` - bag-of-cells serialized to hex
    ///
    /// # Returns
    ///
    /// decoded message
    pub async fn decode_message(&self, boc: &str) -> Result<DecodedMessage, TonApiError> {
        self.base_client
            .post_json("message/decode".to_string(), None, Some(boc), None)
            .await
    }

    /// Get all inscriptions by owner address. It's experimental API and can be dropped in the future.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `limit` - Default value : 1000
    /// - `offset` - Default value : 0
    ///
    /// # Returns
    ///
    /// account inscriptions
    pub async fn get_account_inscriptions(
        &self,
        account_id: &str,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<InscriptionBalances, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(limit) = limit {
            params.insert("limit", limit);
        }

        if let Some(offset) = offset {
            params.insert("offset", offset);
        }

        self.base_client
            .get(
                format!("experimental/accounts/{}/inscriptions", account_id),
                Some(params),
                None,
            )
            .await
    }

    /// Get the transfer inscriptions history for account. It's experimental API and can be dropped in the future.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `accept_language` - *Default value*: en
    /// - `before_lt` - omit this parameter to get last events
    /// - `limit` - *Default value*: 100
    ///
    /// # Returns
    ///
    /// account inscriptions history
    pub async fn get_account_inscriptions_history(
        &self,
        account_id: &str,
        accept_language: Option<&str>,
        before_lt: Option<i64>,
        limit: Option<u64>,
    ) -> Result<AccountEvents, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(before_lt) = before_lt {
            params.insert("before_lt", before_lt);
        }

        if let Some(limit) = limit {
            params.insert("limit", limit);
        }

        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get(
                format!("experimental/accounts/{}/inscriptions/history", account_id),
                Some(params),
                Some(headers),
            )
            .await
    }

    /// Get the transfer inscriptions history for account. It's experimental API and can be dropped in the future.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    /// - `ticker`
    /// - `before_lt` - omit this parameter to get last events
    /// - `limit` - *Default value*: 100
    /// - `accept_language` - The language to use for the response (optional)
    ///
    /// # Returns
    ///
    /// account inscriptions history
    pub async fn get_account_inscriptions_history_by_ticker(
        &self,
        account_id: &str,
        ticker: &str,
        before_lt: Option<i64>,
        limit: Option<u64>,
        accept_language: Option<&str>,
    ) -> Result<AccountEvents, TonApiError> {
        let mut params = QueryParams::new();

        if let Some(before_lt) = before_lt {
            params.insert("before_lt", before_lt);
        }

        if let Some(limit) = limit {
            params.insert("limit", limit);
        }

        let mut headers = HeaderMap::new();

        if let Some(language) = accept_language {
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(language)?);
        }

        self.base_client
            .get(
                format!(
                    "experimental/accounts/{}/inscriptions/{}/history",
                    account_id, ticker
                ),
                Some(params),
                Some(headers),
            )
            .await
    }

    /// Return comment for making operation with inscription. please don't use it if you don't know what you are doing.
    ///
    /// # Parameters
    ///
    /// - `type` - Available values : ton20, gram20
    /// - `destination`
    /// - `comment`
    /// - `operation` - Available values : transfer
    /// - `amount`
    /// - `ticker`
    /// - `who`
    ///
    /// # Returns
    ///
    /// inscription op template
    pub async fn get_inscription_op_template(
        &self,
        type_: &str,
        destination: Option<&str>,
        comment: Option<&str>,
        operation: &str,
        amount: &str,
        ticker: &str,
        who: &str,
    ) -> Result<InscriptionOpTemplate, TonApiError> {
        let mut params = QueryParams::from_pairs([
            ("type", type_),
            ("operation", operation),
            ("amount", amount),
            ("ticker", ticker),
            ("who", who),
        ]);

        if let Some(destination) = destination {
            params.insert("destination", destination);
        }

        if let Some(comment) = comment {
            params.insert("comment", comment);
        }

        self.base_client
            .get(
                "experimental/inscriptions/op-template".to_string(),
                Some(params),
                None,
            )
            .await
    }

    /// Status.
    ///
    /// # Returns
    ///
    /// status
    pub async fn get_status(&self) -> Result<ServiceStatus, TonApiError> {
        self.base_client.get("status".to_string(), None, None).await
    }

    /// Parse address and display in all formats.
    ///
    /// # Parameters
    ///
    /// - `account_id` - account ID
    ///
    /// # Returns
    ///
    /// all forms and info
    pub async fn parse_address(&self, account_id: &str) -> Result<ParsedAddress, TonApiError> {
        self.base_client
            .get(format!("address/{}/parse", account_id), None, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const API_KEY: Option<String> = None;
    const ACCOUNT_ID: &str = "0QCbOix87iy37AwRCWaYhJHzc2gXE_WnAG5vVEAySNT7zClz";
    const ACCOUNT_ID_MAINNET: &str = "EQCD39VS5jcptHL8vMjEXrzGaRcCVYto7HUn4bpAOg8xqB2N";
    const JETTON_ID: &str = "kQDZADmMBA5A10sLWNssTjH-2FS8aix2UixT48Xt3j_g0NZY";
    const JETTON_ID_MAINNET: &str = "EQBCFwW8uFUh-amdRmNY9NyeDEaeDYXd9ggJGsicpqVcHq7B";
    const EVENT_ID: &str = "df5d00116aebaab3a60c4d4551663deb3e5ae334d229e541913cd9cac6811981";
    const EVENT_ID_MAINNET: &str =
        "68656e74d18b10309e41e057191abcfc42f973c82bc84326985cdbf7bf89b126";
    const DOMAIN_NAME_1: &str = "fragment.ton";
    const DOMAIN_NAME_2: &str = "foundation.ton";
    const NFT_COLLECTION_ID: &str = "EQDD6a-rK5jMrrhoZUhPOkByWRIHmCdf1f0-PI-1Cdl3NRBV";
    const NFT_ID: &str = "EQDhmGLnSk463FC1G6RVgfQs0JJ6OIlBKoQ6fRVb9-JIPQ2R";
    const NFT_ID_MAINNET: &str = "EQBSZKEvqoiuPUCFz-CHtpVxAwg1F8PyjZhWAJL2yeujn0_H";
    const TRACE_ID_HEX: &str = "97264395BD65A255A429B11326C84128B7D70FFED7949ABAE3036D506BA38621";
    const EVENT_ID_HEX: &str = "53388440417dc044d00e99d89b591acc28f100332a004f180e4f14b876620c13";
    const MASTERCHAIN_SEQNO: i32 = 123456;
    const BLOCK_ID: &str = "(-1,8000000000000000,4234234)";
    const TRANSACTION_ID: &str = "97264395BD65A255A429B11326C84128B7D70FFED7949ABAE3036D506BA38621";
    const MESSAGE_ID: &str = "EAC465A0DC51E844B12BBD0040308801FA19B8D1BD49208AA929E2CAAEE9D401";

    #[tokio::test]
    #[ignore]
    async fn test_get_accounts_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_accounts(&[ACCOUNT_ID], None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_account(ACCOUNT_ID).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_account_dns_back_resolve_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.account_dns_back_resolve(ACCOUNT_ID).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_jettons_balances_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client
            .get_account_jettons_balances(ACCOUNT_ID, None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_jetton_balance_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client
            .get_account_jetton_balance(ACCOUNT_ID, JETTON_ID, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_jettons_history_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client
            .get_account_jettons_history(ACCOUNT_ID, None, None, 10, None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_jetton_history_by_id_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client
            .get_account_jetton_history_by_id(ACCOUNT_ID, JETTON_ID, None, None, 10, None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_nft_items_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client
            .get_account_nft_items(ACCOUNT_ID, None, None, None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_events_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client
            .get_account_events(ACCOUNT_ID, None, None, None, None, 10, None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_event_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client
            .get_account_event(ACCOUNT_ID, EVENT_ID, None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_traces_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_account_traces(ACCOUNT_ID, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_subscriptions_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_account_subscriptions(ACCOUNT_ID).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_reindex_account_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.reindex_account(ACCOUNT_ID).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_search_accounts_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.search_accounts(DOMAIN_NAME_1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_dns_expiring_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_account_dns_expiring(ACCOUNT_ID, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_public_key_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_account_public_key(ACCOUNT_ID).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_multisigs_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_account_multisigs(ACCOUNT_ID).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_diff_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client
            .get_account_diff(ACCOUNT_ID, 1514746800, 1672513200)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_nft_history_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client
            .get_account_nft_history(ACCOUNT_ID, None, None, 10, None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_nft_collections_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_nft_collections(None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_nft_collection_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_nft_collection(NFT_COLLECTION_ID).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_items_from_collection_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client
            .get_items_from_collection(NFT_COLLECTION_ID, None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_nft_items_by_addresses_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_nft_items_by_addresses(&[NFT_ID]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_nft_item_by_address_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_nft_item_by_address(NFT_ID_MAINNET).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_nft_history_by_id_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client
            .get_nft_history_by_id(ACCOUNT_ID, None, None, 10, None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_jettons_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_jettons(None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_jetton_info_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_jetton_info(JETTON_ID_MAINNET).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_jetton_holders_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client
            .get_jetton_holders(JETTON_ID_MAINNET, None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_jetton_transfer_payload_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client
            .get_jetton_transfer_payload(ACCOUNT_ID_MAINNET, JETTON_ID_MAINNET)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_jettons_events_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_jettons_events(EVENT_ID_MAINNET, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_dns_info_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_dns_info(DOMAIN_NAME_1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_dns_resolve_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.dns_resolve(DOMAIN_NAME_1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_domain_bids_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_domain_bids(DOMAIN_NAME_2).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_all_auctions_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_all_auctions(DOMAIN_NAME_2).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_seqno_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_account_seqno(ACCOUNT_ID).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_rates_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_rates(&["ton"], &["ton", "usd", "rub"]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_chart_rates_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_chart_rates("ton", None, None, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_markets_rates_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_markets_rates().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_nominators_pools_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client
            .get_account_nominators_pools(ACCOUNT_ID_MAINNET)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_staking_pools_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_staking_pools(None, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_trace_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_trace(TRACE_ID_HEX).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_event_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_event(EVENT_ID_HEX, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_storage_providers_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_storage_providers().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_ton_connect_payload_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_ton_connect_payload().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_gasless_config_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_gasless_config().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_reduced_blockchain_blocks_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_reduced_blockchain_blocks(1000, 1001).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_block_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);

        let result = client.get_blockchain_block(BLOCK_ID).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_masterchain_shards_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);

        let result = client
            .get_blockchain_masterchain_shards(MASTERCHAIN_SEQNO)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_masterchain_blocks_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);

        let result = client
            .get_blockchain_masterchain_blocks(MASTERCHAIN_SEQNO)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_masterchain_transactions_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client
            .get_blockchain_masterchain_transactions(MASTERCHAIN_SEQNO)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_block_transactions_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_blockchain_block_transactions(BLOCK_ID).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_transaction_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_blockchain_transaction(TRANSACTION_ID).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_transaction_by_message_hash_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client
            .get_blockchain_transaction_by_message_hash(MESSAGE_ID)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_validators_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_blockchain_validators().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_masterchain_head_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_blockchain_masterchain_head().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_raw_account_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_blockchain_raw_account(ACCOUNT_ID_MAINNET).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_account_transactions_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client
            .get_blockchain_account_transactions(
                ACCOUNT_ID,
                Some(39787624000003),
                None,
                Some(100),
                Some("desc"),
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_exec_get_method_for_blockchain_account_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client
            .exec_get_method_for_blockchain_account(ACCOUNT_ID_MAINNET, "get_wallet_address", None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_blockchain_config_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_blockchain_config().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_raw_blockchain_config_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_raw_blockchain_config().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_raw_masterchain_info_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_raw_masterchain_info().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_raw_masterchain_info_ext_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_raw_masterchain_info_ext(0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_raw_time_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);
        let result = client.get_raw_time().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_out_msg_queue_sizes_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_out_msg_queue_sizes().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_account_inscriptions_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client
            .get_account_inscriptions(ACCOUNT_ID_MAINNET, Some(10), Some(0))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_status_does_not_fail() {
        let client = RestApiClientV2::new(Network::Mainnet, API_KEY);
        let result = client.get_status().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_parse_address_does_not_fail() {
        let client = RestApiClientV2::new(Network::Testnet, API_KEY);

        let result = client.parse_address(ACCOUNT_ID).await;
        assert!(result.is_ok());
    }
}
