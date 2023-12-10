# \ConnectApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_info_by_state_init**](ConnectApi.md#get_account_info_by_state_init) | **POST** /v2/tonconnect/stateinit | 
[**get_ton_connect_payload**](ConnectApi.md#get_ton_connect_payload) | **GET** /v2/tonconnect/payload | 



## get_account_info_by_state_init

> crate::models::AccountInfoByStateInit get_account_info_by_state_init(get_account_info_by_state_init_request)


Get account info by state init

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_account_info_by_state_init_request** | [**GetAccountInfoByStateInitRequest**](GetAccountInfoByStateInitRequest.md) | Data that is expected | [required] |

### Return type

[**crate::models::AccountInfoByStateInit**](AccountInfoByStateInit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ton_connect_payload

> crate::models::GetTonConnectPayload200Response get_ton_connect_payload()


Get a payload for further token receipt

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetTonConnectPayload200Response**](getTonConnectPayload_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

