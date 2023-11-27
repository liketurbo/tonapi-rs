# \BlockchainApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**blockchain_account_inspect**](BlockchainApi.md#blockchain_account_inspect) | **GET** /v2/blockchain/accounts/{account_id}/inspect | 
[**exec_get_method_for_blockchain_account**](BlockchainApi.md#exec_get_method_for_blockchain_account) | **GET** /v2/blockchain/accounts/{account_id}/methods/{method_name} | 
[**get_blockchain_account_transactions**](BlockchainApi.md#get_blockchain_account_transactions) | **GET** /v2/blockchain/accounts/{account_id}/transactions | 
[**get_blockchain_block**](BlockchainApi.md#get_blockchain_block) | **GET** /v2/blockchain/blocks/{block_id} | 
[**get_blockchain_block_transactions**](BlockchainApi.md#get_blockchain_block_transactions) | **GET** /v2/blockchain/blocks/{block_id}/transactions | 
[**get_blockchain_config**](BlockchainApi.md#get_blockchain_config) | **GET** /v2/blockchain/config | 
[**get_blockchain_config_from_block**](BlockchainApi.md#get_blockchain_config_from_block) | **GET** /v2/blockchain/masterchain/{masterchain_seqno}/config | 
[**get_blockchain_masterchain_head**](BlockchainApi.md#get_blockchain_masterchain_head) | **GET** /v2/blockchain/masterchain-head | 
[**get_blockchain_masterchain_shards**](BlockchainApi.md#get_blockchain_masterchain_shards) | **GET** /v2/blockchain/masterchain/{masterchain_seqno}/shards | 
[**get_blockchain_raw_account**](BlockchainApi.md#get_blockchain_raw_account) | **GET** /v2/blockchain/accounts/{account_id} | 
[**get_blockchain_transaction**](BlockchainApi.md#get_blockchain_transaction) | **GET** /v2/blockchain/transactions/{transaction_id} | 
[**get_blockchain_transaction_by_message_hash**](BlockchainApi.md#get_blockchain_transaction_by_message_hash) | **GET** /v2/blockchain/messages/{msg_id}/transaction | 
[**get_blockchain_validators**](BlockchainApi.md#get_blockchain_validators) | **GET** /v2/blockchain/validators | 
[**get_raw_blockchain_config**](BlockchainApi.md#get_raw_blockchain_config) | **GET** /v2/blockchain/config/raw | 
[**get_raw_blockchain_config_from_block**](BlockchainApi.md#get_raw_blockchain_config_from_block) | **GET** /v2/blockchain/masterchain/{masterchain_seqno}/config/raw | 
[**send_blockchain_message**](BlockchainApi.md#send_blockchain_message) | **POST** /v2/blockchain/message | 



## blockchain_account_inspect

> crate::models::BlockchainAccountInspect blockchain_account_inspect(account_id)


Blockchain account inspect

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::BlockchainAccountInspect**](BlockchainAccountInspect.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exec_get_method_for_blockchain_account

> crate::models::MethodExecutionResult exec_get_method_for_blockchain_account(account_id, method_name, args)


Execute get method for account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**method_name** | **String** | contract get method name | [required] |
**args** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::MethodExecutionResult**](MethodExecutionResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_account_transactions

> crate::models::Transactions get_blockchain_account_transactions(account_id, after_lt, before_lt, limit)


Get account transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**after_lt** | Option<**i64**> | omit this parameter to get last transactions |  |
**before_lt** | Option<**i64**> | omit this parameter to get last transactions |  |
**limit** | Option<**i32**> |  |  |[default to 100]

### Return type

[**crate::models::Transactions**](Transactions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_block

> crate::models::BlockchainBlock get_blockchain_block(block_id)


Get blockchain block data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_id** | **String** | block ID | [required] |

### Return type

[**crate::models::BlockchainBlock**](BlockchainBlock.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_block_transactions

> crate::models::Transactions get_blockchain_block_transactions(block_id)


Get transactions from block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_id** | **String** | block ID | [required] |

### Return type

[**crate::models::Transactions**](Transactions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_config

> crate::models::BlockchainConfig get_blockchain_config()


Get blockchain config

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BlockchainConfig**](BlockchainConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_config_from_block

> crate::models::BlockchainConfig get_blockchain_config_from_block(masterchain_seqno)


Get blockchain config from a specific block, if present.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**masterchain_seqno** | **i32** | masterchain block seqno | [required] |

### Return type

[**crate::models::BlockchainConfig**](BlockchainConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_masterchain_head

> crate::models::BlockchainBlock get_blockchain_masterchain_head()


Get last known masterchain block

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BlockchainBlock**](BlockchainBlock.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_masterchain_shards

> crate::models::BlockchainBlockShards get_blockchain_masterchain_shards(masterchain_seqno)


Get blockchain block shards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**masterchain_seqno** | **i32** | masterchain block seqno | [required] |

### Return type

[**crate::models::BlockchainBlockShards**](BlockchainBlockShards.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_raw_account

> crate::models::BlockchainRawAccount get_blockchain_raw_account(account_id)


Get low-level information about an account taken directly from the blockchain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::BlockchainRawAccount**](BlockchainRawAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_transaction

> crate::models::Transaction get_blockchain_transaction(transaction_id)


Get transaction data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** | transaction ID | [required] |

### Return type

[**crate::models::Transaction**](Transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_transaction_by_message_hash

> crate::models::Transaction get_blockchain_transaction_by_message_hash(msg_id)


Get transaction data by message hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**msg_id** | **String** | message ID | [required] |

### Return type

[**crate::models::Transaction**](Transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_validators

> crate::models::Validators get_blockchain_validators()


Get blockchain validators

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Validators**](Validators.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_blockchain_config

> crate::models::RawBlockchainConfig get_raw_blockchain_config()


Get raw blockchain config

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RawBlockchainConfig**](RawBlockchainConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_blockchain_config_from_block

> crate::models::RawBlockchainConfig get_raw_blockchain_config_from_block(masterchain_seqno)


Get raw blockchain config from a specific block, if present.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**masterchain_seqno** | **i32** | masterchain block seqno | [required] |

### Return type

[**crate::models::RawBlockchainConfig**](RawBlockchainConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_blockchain_message

> send_blockchain_message(send_blockchain_message_request)


Send message to blockchain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_blockchain_message_request** | [**SendBlockchainMessageRequest**](SendBlockchainMessageRequest.md) | both a single boc and a batch of boc serialized in base64 are accepted | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

