# \NftApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_nft_history**](NftApi.md#get_account_nft_history) | **GET** /v2/accounts/{account_id}/nfts/history | 
[**get_items_from_collection**](NftApi.md#get_items_from_collection) | **GET** /v2/nfts/collections/{account_id}/items | 
[**get_nft_collection**](NftApi.md#get_nft_collection) | **GET** /v2/nfts/collections/{account_id} | 
[**get_nft_collections**](NftApi.md#get_nft_collections) | **GET** /v2/nfts/collections | 
[**get_nft_history_by_id**](NftApi.md#get_nft_history_by_id) | **GET** /v2/nfts/{account_id}/history | 
[**get_nft_item_by_address**](NftApi.md#get_nft_item_by_address) | **GET** /v2/nfts/{account_id} | 
[**get_nft_items_by_addresses**](NftApi.md#get_nft_items_by_addresses) | **POST** /v2/nfts/_bulk | 



## get_account_nft_history

> crate::models::AccountEvents get_account_nft_history(account_id, limit, accept_language, before_lt, start_date, end_date)


Get the transfer nft history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**limit** | **i32** |  | [required] |
**accept_language** | Option<**String**> |  |  |[default to en]
**before_lt** | Option<**i64**> | omit this parameter to get last events |  |
**start_date** | Option<**i64**> |  |  |
**end_date** | Option<**i64**> |  |  |

### Return type

[**crate::models::AccountEvents**](AccountEvents.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_items_from_collection

> crate::models::NftItems get_items_from_collection(account_id, limit, offset)


Get NFT items from collection by collection address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**limit** | Option<**i32**> |  |  |[default to 1000]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**crate::models::NftItems**](NftItems.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_nft_collection

> crate::models::NftCollection get_nft_collection(account_id)


Get NFT collection by collection address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::NftCollection**](NftCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_nft_collections

> crate::models::NftCollections get_nft_collections(limit, offset)


Get NFT collections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 100]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**crate::models::NftCollections**](NftCollections.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_nft_history_by_id

> crate::models::AccountEvents get_nft_history_by_id(account_id, limit, accept_language, before_lt, start_date, end_date)


Get the transfer nfts history for account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**limit** | **i32** |  | [required] |
**accept_language** | Option<**String**> |  |  |[default to en]
**before_lt** | Option<**i64**> | omit this parameter to get last events |  |
**start_date** | Option<**i64**> |  |  |
**end_date** | Option<**i64**> |  |  |

### Return type

[**crate::models::AccountEvents**](AccountEvents.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_nft_item_by_address

> crate::models::NftItem get_nft_item_by_address(account_id)


Get NFT item by its address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::NftItem**](NftItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_nft_items_by_addresses

> crate::models::NftItems get_nft_items_by_addresses(get_accounts_request)


Get NFT items by their addresses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_accounts_request** | Option<[**GetAccountsRequest**](GetAccountsRequest.md)> | a list of account ids |  |

### Return type

[**crate::models::NftItems**](NftItems.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

