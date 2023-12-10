# \EventsApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_event**](EventsApi.md#get_event) | **GET** /v2/events/{event_id} | 



## get_event

> crate::models::Event get_event(event_id, accept_language)


Get an event either by event ID or a hash of any transaction in a trace. An event is built on top of a trace which is a series of transactions caused by one inbound message. TonAPI looks for known patterns inside the trace and splits the trace into actions, where a single action represents a meaningful high-level operation like a Jetton Transfer or an NFT Purchase. Actions are expected to be shown to users. It is advised not to build any logic on top of actions because actions can be changed at any time.

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

