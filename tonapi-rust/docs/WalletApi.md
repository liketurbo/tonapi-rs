# \WalletApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_seqno**](WalletApi.md#get_account_seqno) | **GET** /v2/wallet/{account_id}/seqno | 
[**get_wallet_backup**](WalletApi.md#get_wallet_backup) | **GET** /v2/wallet/backup | 
[**get_wallets_by_public_key**](WalletApi.md#get_wallets_by_public_key) | **GET** /v2/pubkeys/{public_key}/wallets | 
[**set_wallet_backup**](WalletApi.md#set_wallet_backup) | **PUT** /v2/wallet/backup | 
[**ton_connect_proof**](WalletApi.md#ton_connect_proof) | **POST** /v2/wallet/auth/proof | 



## get_account_seqno

> crate::models::Seqno get_account_seqno(account_id)


Get account seqno

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::Seqno**](Seqno.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallet_backup

> crate::models::GetWalletBackup200Response get_wallet_backup(x_ton_connect_auth)


Get backup info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_ton_connect_auth** | **String** |  | [required] |

### Return type

[**crate::models::GetWalletBackup200Response**](getWalletBackup_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wallets_by_public_key

> crate::models::Accounts get_wallets_by_public_key(public_key)


Get wallets by public key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_key** | **String** |  | [required] |

### Return type

[**crate::models::Accounts**](Accounts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_wallet_backup

> set_wallet_backup(x_ton_connect_auth, body)


Set backup info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_ton_connect_auth** | **String** |  | [required] |
**body** | **std::path::PathBuf** | Information for saving backup | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ton_connect_proof

> crate::models::TonConnectProof200Response ton_connect_proof(ton_connect_proof_request)


Account verification and token issuance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ton_connect_proof_request** | [**TonConnectProofRequest**](TonConnectProofRequest.md) | Data that is expected from TON Connect | [required] |

### Return type

[**crate::models::TonConnectProof200Response**](tonConnectProof_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

