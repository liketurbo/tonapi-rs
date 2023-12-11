mod codegen;

use codegen::apis::{configuration, Error};

pub struct RestApi {
    pub config: configuration::Configuration,
}

impl RestApi {
    pub fn new() -> Self {
        let mut config = configuration::Configuration::default();
        let user_agent = Some(format!(
            "{}@{}",
            env!("CARGO_PKG_REPOSITORY")
                .split("/")
                .last()
                .unwrap_or(env!("CARGO_PKG_NAME")),
            env!("CARGO_PKG_VERSION")
        ));
        config.user_agent = user_agent;
        Self { config }
    }
}

mod accounts_api {
    use codegen::{
        apis::accounts_api::{
            account_dns_back_resolve, address_parse, get_account, get_account_diff,
            get_account_dns_expiring, get_account_event, get_account_events,
            get_account_jetton_history_by_id, get_account_jettons_balances,
            get_account_jettons_history, get_account_nft_items, get_account_public_key,
            get_account_subscriptions, get_account_traces, get_accounts, reindex_account,
            search_accounts, AccountDnsBackResolveError, AccountDnsBackResolveParams,
            AddressParseError, AddressParseParams, GetAccountDiffError, GetAccountDiffParams,
            GetAccountDnsExpiringError, GetAccountDnsExpiringParams, GetAccountError,
            GetAccountEventError, GetAccountEventParams, GetAccountEventsError,
            GetAccountEventsParams, GetAccountJettonHistoryByIdError,
            GetAccountJettonHistoryByIdParams, GetAccountJettonsBalancesError,
            GetAccountJettonsBalancesParams, GetAccountJettonsHistoryError,
            GetAccountJettonsHistoryParams, GetAccountNftItemsError, GetAccountNftItemsParams,
            GetAccountParams, GetAccountPublicKeyError, GetAccountPublicKeyParams,
            GetAccountSubscriptionsError, GetAccountSubscriptionsParams, GetAccountTracesError,
            GetAccountTracesParams, GetAccountsError, GetAccountsParams, ReindexAccountError,
            ReindexAccountParams, SearchAccountsError, SearchAccountsParams,
        },
        models::{
            Account, AccountEvent, AccountEvents, Accounts, AddressParse200Response, DnsExpiring,
            DomainNames, FoundAccounts, GetAccountDiff200Response, GetAccountPublicKey200Response,
            JettonsBalances, NftItems, Subscriptions, TraceIds,
        },
    };

    use super::*;

    impl RestApi {
        pub async fn account_dns_back_resolve(
            &self,
            params: AccountDnsBackResolveParams,
        ) -> Result<DomainNames, Error<AccountDnsBackResolveError>> {
            account_dns_back_resolve(&self.config, params).await
        }

        pub async fn address_parse(
            &self,
            params: AddressParseParams,
        ) -> Result<AddressParse200Response, Error<AddressParseError>> {
            address_parse(&self.config, params).await
        }

        pub async fn get_account(
            &self,
            params: GetAccountParams,
        ) -> Result<Account, Error<GetAccountError>> {
            get_account(&self.config, params).await
        }

        pub async fn get_account_diff(
            &self,
            params: GetAccountDiffParams,
        ) -> Result<GetAccountDiff200Response, Error<GetAccountDiffError>> {
            get_account_diff(&self.config, params).await
        }

        pub async fn get_account_dns_expiring(
            &self,
            params: GetAccountDnsExpiringParams,
        ) -> Result<DnsExpiring, Error<GetAccountDnsExpiringError>> {
            get_account_dns_expiring(&self.config, params).await
        }

        pub async fn get_account_event(
            &self,
            params: GetAccountEventParams,
        ) -> Result<AccountEvent, Error<GetAccountEventError>> {
            get_account_event(&self.config, params).await
        }

        pub async fn get_account_events(
            &self,
            params: GetAccountEventsParams,
        ) -> Result<AccountEvents, Error<GetAccountEventsError>> {
            get_account_events(&self.config, params).await
        }

        pub async fn get_account_jetton_history_by_id(
            &self,
            params: GetAccountJettonHistoryByIdParams,
        ) -> Result<AccountEvents, Error<GetAccountJettonHistoryByIdError>> {
            get_account_jetton_history_by_id(&self.config, params).await
        }

        pub async fn get_account_jettons_balances(
            &self,
            params: GetAccountJettonsBalancesParams,
        ) -> Result<JettonsBalances, Error<GetAccountJettonsBalancesError>> {
            get_account_jettons_balances(&self.config, params).await
        }

        pub async fn get_account_jettons_history(
            &self,
            params: GetAccountJettonsHistoryParams,
        ) -> Result<AccountEvents, Error<GetAccountJettonsHistoryError>> {
            get_account_jettons_history(&self.config, params).await
        }

        pub async fn get_account_nft_items(
            &self,
            params: GetAccountNftItemsParams,
        ) -> Result<NftItems, Error<GetAccountNftItemsError>> {
            get_account_nft_items(&self.config, params).await
        }

        pub async fn get_account_public_key(
            &self,
            params: GetAccountPublicKeyParams,
        ) -> Result<GetAccountPublicKey200Response, Error<GetAccountPublicKeyError>> {
            get_account_public_key(&self.config, params).await
        }

        pub async fn get_account_subscriptions(
            &self,
            params: GetAccountSubscriptionsParams,
        ) -> Result<Subscriptions, Error<GetAccountSubscriptionsError>> {
            get_account_subscriptions(&self.config, params).await
        }

        pub async fn get_account_traces(
            &self,
            params: GetAccountTracesParams,
        ) -> Result<TraceIds, Error<GetAccountTracesError>> {
            get_account_traces(&self.config, params).await
        }

        pub async fn get_accounts(
            &self,
            params: GetAccountsParams,
        ) -> Result<Accounts, Error<GetAccountsError>> {
            get_accounts(&self.config, params).await
        }

        pub async fn reindex_account(
            &self,
            params: ReindexAccountParams,
        ) -> Result<(), Error<ReindexAccountError>> {
            reindex_account(&self.config, params).await
        }

        pub async fn search_accounts(
            &self,
            params: SearchAccountsParams,
        ) -> Result<FoundAccounts, Error<SearchAccountsError>> {
            search_accounts(&self.config, params).await
        }
    }
}

mod blockchain_api {
    use codegen::{
        apis::blockchain_api::{
            blockchain_account_inspect, exec_get_method_for_blockchain_account,
            get_blockchain_account_transactions, get_blockchain_block,
            get_blockchain_block_transactions, get_blockchain_config,
            get_blockchain_config_from_block, get_blockchain_masterchain_blocks,
            get_blockchain_masterchain_head, get_blockchain_masterchain_shards,
            get_blockchain_masterchain_transactions, get_blockchain_raw_account,
            get_blockchain_transaction, get_blockchain_transaction_by_message_hash,
            get_blockchain_validators, get_raw_blockchain_config,
            get_raw_blockchain_config_from_block, send_blockchain_message,
            BlockchainAccountInspectError, BlockchainAccountInspectParams,
            ExecGetMethodForBlockchainAccountError, ExecGetMethodForBlockchainAccountParams,
            GetBlockchainAccountTransactionsError, GetBlockchainAccountTransactionsParams,
            GetBlockchainBlockError, GetBlockchainBlockParams, GetBlockchainBlockTransactionsError,
            GetBlockchainBlockTransactionsParams, GetBlockchainConfigError,
            GetBlockchainConfigFromBlockError, GetBlockchainConfigFromBlockParams,
            GetBlockchainMasterchainBlocksError, GetBlockchainMasterchainBlocksParams,
            GetBlockchainMasterchainHeadError, GetBlockchainMasterchainShardsError,
            GetBlockchainMasterchainShardsParams, GetBlockchainMasterchainTransactionsError,
            GetBlockchainMasterchainTransactionsParams, GetBlockchainRawAccountError,
            GetBlockchainRawAccountParams, GetBlockchainTransactionByMessageHashError,
            GetBlockchainTransactionByMessageHashParams, GetBlockchainTransactionError,
            GetBlockchainTransactionParams, GetBlockchainValidatorsError,
            GetRawBlockchainConfigError, GetRawBlockchainConfigFromBlockError,
            GetRawBlockchainConfigFromBlockParams, SendBlockchainMessageError,
            SendBlockchainMessageParams,
        },
        models::{
            BlockchainAccountInspect, BlockchainBlock, BlockchainBlockShards, BlockchainBlocks,
            BlockchainConfig, BlockchainRawAccount, MethodExecutionResult, RawBlockchainConfig,
            Transaction, Transactions, Validators,
        },
    };

    use super::*;

    impl RestApi {
        pub async fn blockchain_account_inspect(
            &self,
            params: BlockchainAccountInspectParams,
        ) -> Result<BlockchainAccountInspect, Error<BlockchainAccountInspectError>> {
            blockchain_account_inspect(&self.config, params).await
        }

        pub async fn exec_get_method_for_blockchain_account(
            &self,
            params: ExecGetMethodForBlockchainAccountParams,
        ) -> Result<MethodExecutionResult, Error<ExecGetMethodForBlockchainAccountError>> {
            exec_get_method_for_blockchain_account(&self.config, params).await
        }

        pub async fn get_blockchain_account_transactions(
            &self,
            params: GetBlockchainAccountTransactionsParams,
        ) -> Result<Transactions, Error<GetBlockchainAccountTransactionsError>> {
            get_blockchain_account_transactions(&self.config, params).await
        }

        pub async fn get_blockchain_block(
            &self,
            params: GetBlockchainBlockParams,
        ) -> Result<BlockchainBlock, Error<GetBlockchainBlockError>> {
            get_blockchain_block(&self.config, params).await
        }

        pub async fn get_blockchain_block_transactions(
            &self,
            params: GetBlockchainBlockTransactionsParams,
        ) -> Result<Transactions, Error<GetBlockchainBlockTransactionsError>> {
            get_blockchain_block_transactions(&self.config, params).await
        }

        pub async fn get_blockchain_config(
            &self,
        ) -> Result<BlockchainConfig, Error<GetBlockchainConfigError>> {
            get_blockchain_config(&self.config).await
        }

        pub async fn get_blockchain_config_from_block(
            &self,
            params: GetBlockchainConfigFromBlockParams,
        ) -> Result<BlockchainConfig, Error<GetBlockchainConfigFromBlockError>> {
            get_blockchain_config_from_block(&self.config, params).await
        }

        pub async fn get_blockchain_masterchain_blocks(
            &self,
            params: GetBlockchainMasterchainBlocksParams,
        ) -> Result<BlockchainBlocks, Error<GetBlockchainMasterchainBlocksError>> {
            get_blockchain_masterchain_blocks(&self.config, params).await
        }

        pub async fn get_blockchain_masterchain_head(
            &self,
        ) -> Result<BlockchainBlock, Error<GetBlockchainMasterchainHeadError>> {
            get_blockchain_masterchain_head(&self.config).await
        }

        pub async fn get_blockchain_masterchain_shards(
            &self,
            params: GetBlockchainMasterchainShardsParams,
        ) -> Result<BlockchainBlockShards, Error<GetBlockchainMasterchainShardsError>> {
            get_blockchain_masterchain_shards(&self.config, params).await
        }

        pub async fn get_blockchain_masterchain_transactions(
            &self,
            params: GetBlockchainMasterchainTransactionsParams,
        ) -> Result<Transactions, Error<GetBlockchainMasterchainTransactionsError>> {
            get_blockchain_masterchain_transactions(&self.config, params).await
        }

        pub async fn get_blockchain_raw_account(
            &self,
            params: GetBlockchainRawAccountParams,
        ) -> Result<BlockchainRawAccount, Error<GetBlockchainRawAccountError>> {
            get_blockchain_raw_account(&self.config, params).await
        }

        pub async fn get_blockchain_transaction(
            &self,
            params: GetBlockchainTransactionParams,
        ) -> Result<Transaction, Error<GetBlockchainTransactionError>> {
            get_blockchain_transaction(&self.config, params).await
        }

        pub async fn get_blockchain_transaction_by_message_hash(
            &self,
            params: GetBlockchainTransactionByMessageHashParams,
        ) -> Result<Transaction, Error<GetBlockchainTransactionByMessageHashError>> {
            get_blockchain_transaction_by_message_hash(&self.config, params).await
        }

        pub async fn get_blockchain_validators(
            &self,
        ) -> Result<Validators, Error<GetBlockchainValidatorsError>> {
            get_blockchain_validators(&self.config).await
        }

        pub async fn get_raw_blockchain_config(
            &self,
        ) -> Result<RawBlockchainConfig, Error<GetRawBlockchainConfigError>> {
            get_raw_blockchain_config(&self.config).await
        }

        pub async fn get_raw_blockchain_config_from_block(
            &self,
            params: GetRawBlockchainConfigFromBlockParams,
        ) -> Result<RawBlockchainConfig, Error<GetRawBlockchainConfigFromBlockError>> {
            get_raw_blockchain_config_from_block(&self.config, params).await
        }

        pub async fn send_blockchain_message(
            &self,
            params: SendBlockchainMessageParams,
        ) -> Result<(), Error<SendBlockchainMessageError>> {
            send_blockchain_message(&self.config, params).await
        }
    }
}

mod connect_api {
    use codegen::{
        apis::connect_api::{
            get_account_info_by_state_init, get_ton_connect_payload,
            GetAccountInfoByStateInitError, GetAccountInfoByStateInitParams,
            GetTonConnectPayloadError,
        },
        models::{AccountInfoByStateInit, GetTonConnectPayload200Response},
    };

    use super::*;

    impl RestApi {
        pub async fn get_account_info_by_state_init(
            &self,
            params: GetAccountInfoByStateInitParams,
        ) -> Result<AccountInfoByStateInit, Error<GetAccountInfoByStateInitError>> {
            get_account_info_by_state_init(&self.config, params).await
        }

        pub async fn get_ton_connect_payload(
            &self,
        ) -> Result<GetTonConnectPayload200Response, Error<GetTonConnectPayloadError>> {
            get_ton_connect_payload(&self.config).await
        }
    }
}

mod dns_api {
    use codegen::{
        apis::dns_api::{
            dns_resolve, get_all_auctions, get_dns_info, get_domain_bids, DnsResolveError,
            DnsResolveParams, GetAllAuctionsError, GetAllAuctionsParams, GetDnsInfoError,
            GetDnsInfoParams, GetDomainBidsError, GetDomainBidsParams,
        },
        models::{Auctions, DnsRecord, DomainBids, DomainInfo},
    };

    use super::*;

    impl RestApi {
        pub async fn dns_resolve(
            &self,
            params: DnsResolveParams,
        ) -> Result<DnsRecord, Error<DnsResolveError>> {
            dns_resolve(&self.config, params).await
        }

        pub async fn get_all_auctions(
            &self,
            params: GetAllAuctionsParams,
        ) -> Result<Auctions, Error<GetAllAuctionsError>> {
            get_all_auctions(&self.config, params).await
        }

        pub async fn get_dns_info(
            &self,
            params: GetDnsInfoParams,
        ) -> Result<DomainInfo, Error<GetDnsInfoError>> {
            get_dns_info(&self.config, params).await
        }

        pub async fn get_domain_bids(
            &self,
            params: GetDomainBidsParams,
        ) -> Result<DomainBids, Error<GetDomainBidsError>> {
            get_domain_bids(&self.config, params).await
        }
    }
}

mod emulation_api {
    use codegen::{
        apis::emulation_api::{
            decode_message, emulate_message_to_account_event, emulate_message_to_event,
            emulate_message_to_trace, emulate_message_to_wallet, DecodeMessageError,
            DecodeMessageParams, EmulateMessageToAccountEventError,
            EmulateMessageToAccountEventParams, EmulateMessageToEventError,
            EmulateMessageToEventParams, EmulateMessageToTraceError, EmulateMessageToTraceParams,
            EmulateMessageToWalletError, EmulateMessageToWalletParams,
        },
        models::{AccountEvent, DecodedMessage, Event, MessageConsequences, Trace},
    };

    use super::*;

    impl RestApi {
        pub async fn decode_message(
            &self,
            params: DecodeMessageParams,
        ) -> Result<DecodedMessage, Error<DecodeMessageError>> {
            decode_message(&self.config, params).await
        }

        pub async fn emulate_message_to_account_event(
            &self,
            params: EmulateMessageToAccountEventParams,
        ) -> Result<AccountEvent, Error<EmulateMessageToAccountEventError>> {
            emulate_message_to_account_event(&self.config, params).await
        }

        pub async fn emulate_message_to_event(
            &self,
            params: EmulateMessageToEventParams,
        ) -> Result<Event, Error<EmulateMessageToEventError>> {
            emulate_message_to_event(&self.config, params).await
        }

        pub async fn emulate_message_to_trace(
            &self,
            params: EmulateMessageToTraceParams,
        ) -> Result<Trace, Error<EmulateMessageToTraceError>> {
            emulate_message_to_trace(&self.config, params).await
        }

        pub async fn emulate_message_to_wallet(
            &self,
            params: EmulateMessageToWalletParams,
        ) -> Result<MessageConsequences, Error<EmulateMessageToWalletError>> {
            emulate_message_to_wallet(&self.config, params).await
        }
    }
}

mod events_api {
    use codegen::{
        apis::events_api::{get_event, GetEventError, GetEventParams},
        models::Event,
    };

    use super::*;

    impl RestApi {
        pub async fn get_event(
            &self,
            params: GetEventParams,
        ) -> Result<Event, Error<GetEventError>> {
            get_event(&self.config, params).await
        }
    }
}

mod jettons_api {
    use codegen::{
        apis::jettons_api::{
            get_jetton_holders, get_jetton_info, get_jettons, get_jettons_events,
            GetJettonHoldersError, GetJettonHoldersParams, GetJettonInfoError, GetJettonInfoParams,
            GetJettonsError, GetJettonsEventsError, GetJettonsEventsParams, GetJettonsParams,
        },
        models::{Event, JettonHolders, JettonInfo, Jettons},
    };

    use super::*;

    impl RestApi {
        pub async fn get_jetton_holders(
            &self,
            params: GetJettonHoldersParams,
        ) -> Result<JettonHolders, Error<GetJettonHoldersError>> {
            get_jetton_holders(&self.config, params).await
        }

        pub async fn get_jetton_info(
            &self,
            params: GetJettonInfoParams,
        ) -> Result<JettonInfo, Error<GetJettonInfoError>> {
            get_jetton_info(&self.config, params).await
        }

        pub async fn get_jettons(
            &self,
            params: GetJettonsParams,
        ) -> Result<Jettons, Error<GetJettonsError>> {
            get_jettons(&self.config, params).await
        }

        pub async fn get_jettons_events(
            &self,
            params: GetJettonsEventsParams,
        ) -> Result<Event, Error<GetJettonsEventsError>> {
            get_jettons_events(&self.config, params).await
        }
    }
}

mod lite_server_api {
    use codegen::{
        apis::lite_server_api::{
            get_all_raw_shards_info, get_raw_account_state, get_raw_block_proof,
            get_raw_blockchain_block, get_raw_blockchain_block_header,
            get_raw_blockchain_block_state, get_raw_config, get_raw_list_block_transactions,
            get_raw_masterchain_info, get_raw_masterchain_info_ext, get_raw_shard_block_proof,
            get_raw_shard_info, get_raw_time, get_raw_transactions, send_raw_message,
            GetAllRawShardsInfoError, GetAllRawShardsInfoParams, GetRawAccountStateError,
            GetRawAccountStateParams, GetRawBlockProofError, GetRawBlockProofParams,
            GetRawBlockchainBlockError, GetRawBlockchainBlockHeaderError,
            GetRawBlockchainBlockHeaderParams, GetRawBlockchainBlockParams,
            GetRawBlockchainBlockStateError, GetRawBlockchainBlockStateParams, GetRawConfigError,
            GetRawConfigParams, GetRawListBlockTransactionsError,
            GetRawListBlockTransactionsParams, GetRawMasterchainInfoError,
            GetRawMasterchainInfoExtError, GetRawMasterchainInfoExtParams,
            GetRawShardBlockProofError, GetRawShardBlockProofParams, GetRawShardInfoError,
            GetRawShardInfoParams, GetRawTimeError, GetRawTransactionsError,
            GetRawTransactionsParams, SendRawMessageError, SendRawMessageParams,
        },
        models::{
            GetAllRawShardsInfo200Response, GetRawAccountState200Response,
            GetRawBlockProof200Response, GetRawBlockchainBlock200Response,
            GetRawBlockchainBlockHeader200Response, GetRawBlockchainBlockState200Response,
            GetRawConfig200Response, GetRawListBlockTransactions200Response,
            GetRawMasterchainInfo200Response, GetRawMasterchainInfoExt200Response,
            GetRawShardBlockProof200Response, GetRawShardInfo200Response, GetRawTime200Response,
            GetRawTransactions200Response, SendRawMessage200Response,
        },
    };

    use super::*;

    impl RestApi {
        pub async fn get_all_raw_shards_info(
            &self,
            params: GetAllRawShardsInfoParams,
        ) -> Result<GetAllRawShardsInfo200Response, Error<GetAllRawShardsInfoError>> {
            get_all_raw_shards_info(&self.config, params).await
        }

        pub async fn get_raw_account_state(
            &self,
            params: GetRawAccountStateParams,
        ) -> Result<GetRawAccountState200Response, Error<GetRawAccountStateError>> {
            get_raw_account_state(&self.config, params).await
        }

        pub async fn get_raw_block_proof(
            &self,
            params: GetRawBlockProofParams,
        ) -> Result<GetRawBlockProof200Response, Error<GetRawBlockProofError>> {
            get_raw_block_proof(&self.config, params).await
        }

        pub async fn get_raw_blockchain_block(
            &self,
            params: GetRawBlockchainBlockParams,
        ) -> Result<GetRawBlockchainBlock200Response, Error<GetRawBlockchainBlockError>> {
            get_raw_blockchain_block(&self.config, params).await
        }

        pub async fn get_raw_blockchain_block_header(
            &self,
            params: GetRawBlockchainBlockHeaderParams,
        ) -> Result<GetRawBlockchainBlockHeader200Response, Error<GetRawBlockchainBlockHeaderError>>
        {
            get_raw_blockchain_block_header(&self.config, params).await
        }

        pub async fn get_raw_blockchain_block_state(
            &self,
            params: GetRawBlockchainBlockStateParams,
        ) -> Result<GetRawBlockchainBlockState200Response, Error<GetRawBlockchainBlockStateError>>
        {
            get_raw_blockchain_block_state(&self.config, params).await
        }

        pub async fn get_raw_config(
            &self,
            params: GetRawConfigParams,
        ) -> Result<GetRawConfig200Response, Error<GetRawConfigError>> {
            get_raw_config(&self.config, params).await
        }

        pub async fn get_raw_list_block_transactions(
            &self,
            params: GetRawListBlockTransactionsParams,
        ) -> Result<GetRawListBlockTransactions200Response, Error<GetRawListBlockTransactionsError>>
        {
            get_raw_list_block_transactions(&self.config, params).await
        }

        pub async fn get_raw_masterchain_info(
            &self,
        ) -> Result<GetRawMasterchainInfo200Response, Error<GetRawMasterchainInfoError>> {
            get_raw_masterchain_info(&self.config).await
        }

        pub async fn get_raw_masterchain_info_ext(
            &self,
            params: GetRawMasterchainInfoExtParams,
        ) -> Result<GetRawMasterchainInfoExt200Response, Error<GetRawMasterchainInfoExtError>>
        {
            get_raw_masterchain_info_ext(&self.config, params).await
        }

        pub async fn get_raw_shard_block_proof(
            &self,
            params: GetRawShardBlockProofParams,
        ) -> Result<GetRawShardBlockProof200Response, Error<GetRawShardBlockProofError>> {
            get_raw_shard_block_proof(&self.config, params).await
        }

        pub async fn get_raw_shard_info(
            &self,
            params: GetRawShardInfoParams,
        ) -> Result<GetRawShardInfo200Response, Error<GetRawShardInfoError>> {
            get_raw_shard_info(&self.config, params).await
        }

        pub async fn get_raw_time(&self) -> Result<GetRawTime200Response, Error<GetRawTimeError>> {
            get_raw_time(&self.config).await
        }

        pub async fn get_raw_transactions(
            &self,
            params: GetRawTransactionsParams,
        ) -> Result<GetRawTransactions200Response, Error<GetRawTransactionsError>> {
            get_raw_transactions(&self.config, params).await
        }

        pub async fn send_raw_message(
            &self,
            params: SendRawMessageParams,
        ) -> Result<SendRawMessage200Response, Error<SendRawMessageError>> {
            send_raw_message(&self.config, params).await
        }
    }
}

mod nft_api {
    use codegen::{
        apis::nft_api::{
            get_account_nft_history, get_items_from_collection, get_nft_collection,
            get_nft_collections, get_nft_history_by_id, get_nft_item_by_address,
            get_nft_items_by_addresses, GetAccountNftHistoryError, GetAccountNftHistoryParams,
            GetItemsFromCollectionError, GetItemsFromCollectionParams, GetNftCollectionError,
            GetNftCollectionParams, GetNftCollectionsError, GetNftCollectionsParams,
            GetNftHistoryByIdError, GetNftHistoryByIdParams, GetNftItemByAddressError,
            GetNftItemByAddressParams, GetNftItemsByAddressesError, GetNftItemsByAddressesParams,
        },
        models::{AccountEvents, NftCollection, NftCollections, NftItem, NftItems},
    };

    use super::*;

    impl RestApi {
        pub async fn get_account_nft_history(
            &self,
            params: GetAccountNftHistoryParams,
        ) -> Result<AccountEvents, Error<GetAccountNftHistoryError>> {
            get_account_nft_history(&self.config, params).await
        }

        pub async fn get_items_from_collection(
            &self,
            params: GetItemsFromCollectionParams,
        ) -> Result<NftItems, Error<GetItemsFromCollectionError>> {
            get_items_from_collection(&self.config, params).await
        }

        pub async fn get_nft_collection(
            &self,
            params: GetNftCollectionParams,
        ) -> Result<NftCollection, Error<GetNftCollectionError>> {
            get_nft_collection(&self.config, params).await
        }

        pub async fn get_nft_collections(
            &self,
            params: GetNftCollectionsParams,
        ) -> Result<NftCollections, Error<GetNftCollectionsError>> {
            get_nft_collections(&self.config, params).await
        }

        pub async fn get_nft_history_by_id(
            &self,
            params: GetNftHistoryByIdParams,
        ) -> Result<AccountEvents, Error<GetNftHistoryByIdError>> {
            get_nft_history_by_id(&self.config, params).await
        }

        pub async fn get_nft_item_by_address(
            &self,
            params: GetNftItemByAddressParams,
        ) -> Result<NftItem, Error<GetNftItemByAddressError>> {
            get_nft_item_by_address(&self.config, params).await
        }

        pub async fn get_nft_items_by_addresses(
            &self,
            params: GetNftItemsByAddressesParams,
        ) -> Result<NftItems, Error<GetNftItemsByAddressesError>> {
            get_nft_items_by_addresses(&self.config, params).await
        }
    }
}

mod rates_api {
    use codegen::{
        apis::rates_api::{
            get_chart_rates, get_rates, GetChartRatesError, GetChartRatesParams, GetRatesError,
            GetRatesParams,
        },
        models::{GetChartRates200Response, GetRates200Response},
    };

    use super::*;

    impl RestApi {
        pub async fn get_chart_rates(
            &self,
            params: GetChartRatesParams,
        ) -> Result<GetChartRates200Response, Error<GetChartRatesError>> {
            get_chart_rates(&self.config, params).await
        }

        pub async fn get_rates(
            &self,
            params: GetRatesParams,
        ) -> Result<GetRates200Response, Error<GetRatesError>> {
            get_rates(&self.config, params).await
        }
    }
}

mod staking_api {
    use codegen::{
        apis::staking_api::{
            get_account_nominators_pools, get_staking_pool_history, get_staking_pool_info,
            get_staking_pools, GetAccountNominatorsPoolsError, GetAccountNominatorsPoolsParams,
            GetStakingPoolHistoryError, GetStakingPoolHistoryParams, GetStakingPoolInfoError,
            GetStakingPoolInfoParams, GetStakingPoolsError, GetStakingPoolsParams,
        },
        models::{
            AccountStaking, GetStakingPoolHistory200Response, GetStakingPoolInfo200Response,
            GetStakingPools200Response,
        },
    };

    use super::*;

    impl RestApi {
        pub async fn get_account_nominators_pools(
            &self,
            params: GetAccountNominatorsPoolsParams,
        ) -> Result<AccountStaking, Error<GetAccountNominatorsPoolsError>> {
            get_account_nominators_pools(&self.config, params).await
        }

        pub async fn get_staking_pool_history(
            &self,
            params: GetStakingPoolHistoryParams,
        ) -> Result<GetStakingPoolHistory200Response, Error<GetStakingPoolHistoryError>> {
            get_staking_pool_history(&self.config, params).await
        }

        pub async fn get_staking_pool_info(
            &self,
            params: GetStakingPoolInfoParams,
        ) -> Result<GetStakingPoolInfo200Response, Error<GetStakingPoolInfoError>> {
            get_staking_pool_info(&self.config, params).await
        }

        pub async fn get_staking_pools(
            &self,
            params: GetStakingPoolsParams,
        ) -> Result<GetStakingPools200Response, Error<GetStakingPoolsError>> {
            get_staking_pools(&self.config, params).await
        }
    }
}

mod storage_api {
    use codegen::{
        apis::storage_api::{get_storage_providers, GetStorageProvidersError},
        models::GetStorageProviders200Response,
    };

    use super::*;

    impl RestApi {
        pub async fn get_storage_providers(
            &self,
        ) -> Result<GetStorageProviders200Response, Error<GetStorageProvidersError>> {
            get_storage_providers(&self.config).await
        }
    }
}

mod traces_api {
    use super::*;

    use codegen::{
        apis::traces_api::{get_trace, GetTraceError, GetTraceParams},
        models::Trace,
    };

    impl RestApi {
        pub async fn get_trace(
            &self,
            params: GetTraceParams,
        ) -> Result<Trace, Error<GetTraceError>> {
            get_trace(&self.config, params).await
        }
    }
}

mod wallet_api {
    use codegen::{
        apis::wallet_api::{
            get_account_seqno, get_wallet_backup, get_wallets_by_public_key, set_wallet_backup,
            ton_connect_proof, GetAccountSeqnoError, GetAccountSeqnoParams, GetWalletBackupError,
            GetWalletBackupParams, GetWalletsByPublicKeyError, GetWalletsByPublicKeyParams,
            SetWalletBackupError, SetWalletBackupParams, TonConnectProofError,
            TonConnectProofParams,
        },
        models::{Accounts, GetWalletBackup200Response, Seqno, TonConnectProof200Response},
    };

    use super::*;

    impl RestApi {
        pub async fn get_account_seqno(
            &self,
            params: GetAccountSeqnoParams,
        ) -> Result<Seqno, Error<GetAccountSeqnoError>> {
            get_account_seqno(&self.config, params).await
        }

        pub async fn get_wallet_backup(
            &self,
            params: GetWalletBackupParams,
        ) -> Result<GetWalletBackup200Response, Error<GetWalletBackupError>> {
            get_wallet_backup(&self.config, params).await
        }

        pub async fn get_wallets_by_public_key(
            &self,
            params: GetWalletsByPublicKeyParams,
        ) -> Result<Accounts, Error<GetWalletsByPublicKeyError>> {
            get_wallets_by_public_key(&self.config, params).await
        }

        pub async fn set_wallet_backup(
            &self,
            params: SetWalletBackupParams,
        ) -> Result<(), Error<SetWalletBackupError>> {
            set_wallet_backup(&self.config, params).await
        }

        pub async fn ton_connect_proof(
            &self,
            params: TonConnectProofParams,
        ) -> Result<TonConnectProof200Response, Error<TonConnectProofError>> {
            ton_connect_proof(&self.config, params).await
        }
    }
}
