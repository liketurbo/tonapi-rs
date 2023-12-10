# \TracesApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_trace**](TracesApi.md#get_trace) | **GET** /v2/traces/{trace_id} | 



## get_trace

> crate::models::Trace get_trace(trace_id)


Get the trace by trace ID or hash of any transaction in trace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trace_id** | **String** | trace ID or transaction hash in hex (without 0x) or base64url format | [required] |

### Return type

[**crate::models::Trace**](Trace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

