# \LiteServerApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_raw_shards_info**](LiteServerApi.md#get_all_raw_shards_info) | **GET** /v2/liteserver/get_all_shards_info/{block_id} | 
[**get_raw_account_state**](LiteServerApi.md#get_raw_account_state) | **GET** /v2/liteserver/get_account_state/{account_id} | 
[**get_raw_block_proof**](LiteServerApi.md#get_raw_block_proof) | **GET** /v2/liteserver/get_block_proof | 
[**get_raw_blockchain_block**](LiteServerApi.md#get_raw_blockchain_block) | **GET** /v2/liteserver/get_block/{block_id} | 
[**get_raw_blockchain_block_header**](LiteServerApi.md#get_raw_blockchain_block_header) | **GET** /v2/liteserver/get_block_header/{block_id} | 
[**get_raw_blockchain_block_state**](LiteServerApi.md#get_raw_blockchain_block_state) | **GET** /v2/liteserver/get_state/{block_id} | 
[**get_raw_config**](LiteServerApi.md#get_raw_config) | **GET** /v2/liteserver/get_config_all/{block_id} | 
[**get_raw_list_block_transactions**](LiteServerApi.md#get_raw_list_block_transactions) | **GET** /v2/liteserver/list_block_transactions/{block_id} | 
[**get_raw_masterchain_info**](LiteServerApi.md#get_raw_masterchain_info) | **GET** /v2/liteserver/get_masterchain_info | 
[**get_raw_masterchain_info_ext**](LiteServerApi.md#get_raw_masterchain_info_ext) | **GET** /v2/liteserver/get_masterchain_info_ext | 
[**get_raw_shard_block_proof**](LiteServerApi.md#get_raw_shard_block_proof) | **GET** /v2/liteserver/get_shard_block_proof/{block_id} | 
[**get_raw_shard_info**](LiteServerApi.md#get_raw_shard_info) | **GET** /v2/liteserver/get_shard_info/{block_id} | 
[**get_raw_time**](LiteServerApi.md#get_raw_time) | **GET** /v2/liteserver/get_time | 
[**get_raw_transactions**](LiteServerApi.md#get_raw_transactions) | **GET** /v2/liteserver/get_transactions/{account_id} | 
[**send_raw_message**](LiteServerApi.md#send_raw_message) | **POST** /v2/liteserver/send_message | 



## get_all_raw_shards_info

> crate::models::GetAllRawShardsInfo200Response get_all_raw_shards_info(block_id)


Get all raw shards info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_id** | **String** | block ID: (workchain,shard,seqno,root_hash,file_hash) | [required] |

### Return type

[**crate::models::GetAllRawShardsInfo200Response**](getAllRawShardsInfo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_account_state

> crate::models::GetRawAccountState200Response get_raw_account_state(account_id, target_block)


Get raw account state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**target_block** | Option<**String**> | target block: (workchain,shard,seqno,root_hash,file_hash) |  |

### Return type

[**crate::models::GetRawAccountState200Response**](getRawAccountState_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_block_proof

> crate::models::GetRawBlockProof200Response get_raw_block_proof(known_block, mode, target_block)


Get raw block proof

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**known_block** | **String** | known block: (workchain,shard,seqno,root_hash,file_hash) | [required] |
**mode** | **i32** | mode | [required] |
**target_block** | Option<**String**> | target block: (workchain,shard,seqno,root_hash,file_hash) |  |

### Return type

[**crate::models::GetRawBlockProof200Response**](getRawBlockProof_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_blockchain_block

> crate::models::GetRawBlockchainBlock200Response get_raw_blockchain_block(block_id)


Get raw blockchain block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_id** | **String** | block ID: (workchain,shard,seqno,root_hash,file_hash) | [required] |

### Return type

[**crate::models::GetRawBlockchainBlock200Response**](getRawBlockchainBlock_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_blockchain_block_header

> crate::models::GetRawBlockchainBlockHeader200Response get_raw_blockchain_block_header(block_id, mode)


Get raw blockchain block header

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_id** | **String** | block ID: (workchain,shard,seqno,root_hash,file_hash) | [required] |
**mode** | **i32** | mode | [required] |

### Return type

[**crate::models::GetRawBlockchainBlockHeader200Response**](getRawBlockchainBlockHeader_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_blockchain_block_state

> crate::models::GetRawBlockchainBlockState200Response get_raw_blockchain_block_state(block_id)


Get raw blockchain block state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_id** | **String** | block ID: (workchain,shard,seqno,root_hash,file_hash) | [required] |

### Return type

[**crate::models::GetRawBlockchainBlockState200Response**](getRawBlockchainBlockState_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_config

> crate::models::GetRawConfig200Response get_raw_config(block_id, mode)


Get raw config

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_id** | **String** | block ID: (workchain,shard,seqno,root_hash,file_hash) | [required] |
**mode** | **i32** | mode | [required] |

### Return type

[**crate::models::GetRawConfig200Response**](getRawConfig_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_list_block_transactions

> crate::models::GetRawListBlockTransactions200Response get_raw_list_block_transactions(block_id, mode, count, account_id, lt)


Get raw list block transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_id** | **String** | block ID: (workchain,shard,seqno,root_hash,file_hash) | [required] |
**mode** | **i32** | mode | [required] |
**count** | **i32** | count | [required] |
**account_id** | Option<**String**> | account ID |  |
**lt** | Option<**i32**> | lt |  |

### Return type

[**crate::models::GetRawListBlockTransactions200Response**](getRawListBlockTransactions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_masterchain_info

> crate::models::GetRawMasterchainInfo200Response get_raw_masterchain_info()


Get raw masterchain info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetRawMasterchainInfo200Response**](getRawMasterchainInfo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_masterchain_info_ext

> crate::models::GetRawMasterchainInfoExt200Response get_raw_masterchain_info_ext(mode)


Get raw masterchain info ext

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mode** | **i32** | mode | [required] |

### Return type

[**crate::models::GetRawMasterchainInfoExt200Response**](getRawMasterchainInfoExt_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_shard_block_proof

> crate::models::GetRawShardBlockProof200Response get_raw_shard_block_proof(block_id)


Get raw shard block proof

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_id** | **String** | block ID: (workchain,shard,seqno,root_hash,file_hash) | [required] |

### Return type

[**crate::models::GetRawShardBlockProof200Response**](getRawShardBlockProof_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_shard_info

> crate::models::GetRawShardInfo200Response get_raw_shard_info(block_id, workchain, shard, exact)


Get raw shard info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_id** | **String** | block ID: (workchain,shard,seqno,root_hash,file_hash) | [required] |
**workchain** | **i32** | workchain | [required] |
**shard** | **i32** | shard | [required] |
**exact** | **bool** | exact | [required] |

### Return type

[**crate::models::GetRawShardInfo200Response**](getRawShardInfo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_time

> crate::models::GetRawTime200Response get_raw_time()


Get raw time

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetRawTime200Response**](getRawTime_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_transactions

> crate::models::GetRawTransactions200Response get_raw_transactions(account_id, count, lt, hash)


Get raw transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**count** | **i32** | count | [required] |
**lt** | **i32** | lt | [required] |
**hash** | **String** | hash | [required] |

### Return type

[**crate::models::GetRawTransactions200Response**](getRawTransactions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_raw_message

> crate::models::SendRawMessage200Response send_raw_message(send_raw_message_request)


Send raw message to blockchain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_raw_message_request** | [**SendRawMessageRequest**](SendRawMessageRequest.md) | Data that is expected | [required] |

### Return type

[**crate::models::SendRawMessage200Response**](sendRawMessage_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

