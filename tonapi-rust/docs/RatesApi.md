# \RatesApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_chart_rates**](RatesApi.md#get_chart_rates) | **GET** /v2/rates/chart | 
[**get_rates**](RatesApi.md#get_rates) | **GET** /v2/rates | 



## get_chart_rates

> crate::models::GetChartRates200Response get_chart_rates(token, currency, start_date, end_date)


Get chart by token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | accept jetton master address | [required] |
**currency** | Option<**String**> |  |  |
**start_date** | Option<**i64**> |  |  |
**end_date** | Option<**i64**> |  |  |

### Return type

[**crate::models::GetChartRates200Response**](getChartRates_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rates

> crate::models::GetRates200Response get_rates(tokens, currencies)


Get the token price to the currency

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tokens** | **String** | accept ton and jetton master addresses, separated by commas | [required] |
**currencies** | **String** | accept ton and all possible fiat currencies, separated by commas | [required] |

### Return type

[**crate::models::GetRates200Response**](getRates_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

