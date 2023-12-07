# \EmulationApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**decode_message**](EmulationApi.md#decode_message) | **POST** /v2/message/decode | 
[**emulate_message_to_account_event**](EmulationApi.md#emulate_message_to_account_event) | **POST** /v2/accounts/{account_id}/events/emulate | 
[**emulate_message_to_event**](EmulationApi.md#emulate_message_to_event) | **POST** /v2/events/emulate | 
[**emulate_message_to_trace**](EmulationApi.md#emulate_message_to_trace) | **POST** /v2/traces/emulate | 
[**emulate_message_to_wallet**](EmulationApi.md#emulate_message_to_wallet) | **POST** /v2/wallet/emulate | 



## decode_message

> crate::models::DecodedMessage decode_message(decode_message_request)


Decode a given message. Only external incoming messages can be decoded currently.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**decode_message_request** | [**DecodeMessageRequest**](DecodeMessageRequest.md) | bag-of-cells serialized to base64 | [required] |

### Return type

[**crate::models::DecodedMessage**](DecodedMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emulate_message_to_account_event

> crate::models::AccountEvent emulate_message_to_account_event(account_id, decode_message_request, accept_language)


Emulate sending message to blockchain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**decode_message_request** | [**DecodeMessageRequest**](DecodeMessageRequest.md) | bag-of-cells serialized to base64 | [required] |
**accept_language** | Option<**String**> |  |  |[default to en]

### Return type

[**crate::models::AccountEvent**](AccountEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emulate_message_to_event

> crate::models::Event emulate_message_to_event(decode_message_request, accept_language, ignore_signature_check)


Emulate sending message to blockchain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**decode_message_request** | [**DecodeMessageRequest**](DecodeMessageRequest.md) | bag-of-cells serialized to base64 | [required] |
**accept_language** | Option<**String**> |  |  |[default to en]
**ignore_signature_check** | Option<**bool**> |  |  |

### Return type

[**crate::models::Event**](Event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emulate_message_to_trace

> crate::models::Trace emulate_message_to_trace(decode_message_request, ignore_signature_check)


Emulate sending message to blockchain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**decode_message_request** | [**DecodeMessageRequest**](DecodeMessageRequest.md) | bag-of-cells serialized to base64 | [required] |
**ignore_signature_check** | Option<**bool**> |  |  |

### Return type

[**crate::models::Trace**](Trace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emulate_message_to_wallet

> crate::models::MessageConsequences emulate_message_to_wallet(emulate_message_to_wallet_request, accept_language)


Emulate sending message to blockchain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emulate_message_to_wallet_request** | [**EmulateMessageToWalletRequest**](EmulateMessageToWalletRequest.md) | bag-of-cells serialized to base64 and additional parameters to configure emulation | [required] |
**accept_language** | Option<**String**> |  |  |[default to en]

### Return type

[**crate::models::MessageConsequences**](MessageConsequences.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

