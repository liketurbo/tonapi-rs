# \JettonsApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_jetton_holders**](JettonsApi.md#get_jetton_holders) | **GET** /v2/jettons/{account_id}/holders | 
[**get_jetton_info**](JettonsApi.md#get_jetton_info) | **GET** /v2/jettons/{account_id} | 
[**get_jettons**](JettonsApi.md#get_jettons) | **GET** /v2/jettons | 
[**get_jettons_events**](JettonsApi.md#get_jettons_events) | **GET** /v2/events/{event_id}/jettons | 



## get_jetton_holders

> crate::models::JettonHolders get_jetton_holders(account_id, limit, offset)


Get jetton's holders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**limit** | Option<**i32**> |  |  |[default to 1000]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**crate::models::JettonHolders**](JettonHolders.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jetton_info

> crate::models::JettonInfo get_jetton_info(account_id)


Get jetton metadata by jetton master address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::JettonInfo**](JettonInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jettons

> crate::models::Jettons get_jettons(limit, offset)


Get a list of all indexed jetton masters in the blockchain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 100]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**crate::models::Jettons**](Jettons.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jettons_events

> crate::models::Event get_jettons_events(event_id, accept_language)


Get only jetton transfers in the event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | event ID or transaction hash in hex (without 0x) or base64url format | [required] |
**accept_language** | Option<**String**> |  |  |[default to en]

### Return type

[**crate::models::Event**](Event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

