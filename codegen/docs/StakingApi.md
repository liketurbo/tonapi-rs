# \StakingApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_nominators_pools**](StakingApi.md#get_account_nominators_pools) | **GET** /v2/staking/nominator/{account_id}/pools | 
[**get_staking_pool_history**](StakingApi.md#get_staking_pool_history) | **GET** /v2/staking/pool/{account_id}/history | 
[**get_staking_pool_info**](StakingApi.md#get_staking_pool_info) | **GET** /v2/staking/pool/{account_id} | 
[**get_staking_pools**](StakingApi.md#get_staking_pools) | **GET** /v2/staking/pools | 



## get_account_nominators_pools

> crate::models::AccountStaking get_account_nominators_pools(account_id)


All pools where account participates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::AccountStaking**](AccountStaking.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_staking_pool_history

> crate::models::GetStakingPoolHistory200Response get_staking_pool_history(account_id)


Pool history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::GetStakingPoolHistory200Response**](getStakingPoolHistory_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_staking_pool_info

> crate::models::GetStakingPoolInfo200Response get_staking_pool_info(account_id, accept_language)


Stacking pool info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**accept_language** | Option<**String**> |  |  |[default to en]

### Return type

[**crate::models::GetStakingPoolInfo200Response**](getStakingPoolInfo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_staking_pools

> crate::models::GetStakingPools200Response get_staking_pools(available_for, include_unverified, accept_language)


All pools available in network

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**available_for** | Option<**String**> | account ID |  |
**include_unverified** | Option<**bool**> | return also pools not from white list - just compatible by interfaces (maybe dangerous!) |  |
**accept_language** | Option<**String**> |  |  |[default to en]

### Return type

[**crate::models::GetStakingPools200Response**](getStakingPools_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

