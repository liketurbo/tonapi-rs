# \AccountsApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_dns_back_resolve**](AccountsApi.md#account_dns_back_resolve) | **GET** /v2/accounts/{account_id}/dns/backresolve | 
[**address_parse**](AccountsApi.md#address_parse) | **GET** /v2/address/{account_id}/parse | 
[**get_account**](AccountsApi.md#get_account) | **GET** /v2/accounts/{account_id} | 
[**get_account_diff**](AccountsApi.md#get_account_diff) | **GET** /v2/accounts/{account_id}/diff | 
[**get_account_dns_expiring**](AccountsApi.md#get_account_dns_expiring) | **GET** /v2/accounts/{account_id}/dns/expiring | 
[**get_account_event**](AccountsApi.md#get_account_event) | **GET** /v2/accounts/{account_id}/events/{event_id} | 
[**get_account_events**](AccountsApi.md#get_account_events) | **GET** /v2/accounts/{account_id}/events | 
[**get_account_jetton_history_by_id**](AccountsApi.md#get_account_jetton_history_by_id) | **GET** /v2/accounts/{account_id}/jettons/{jetton_id}/history | 
[**get_account_jettons_balances**](AccountsApi.md#get_account_jettons_balances) | **GET** /v2/accounts/{account_id}/jettons | 
[**get_account_jettons_history**](AccountsApi.md#get_account_jettons_history) | **GET** /v2/accounts/{account_id}/jettons/history | 
[**get_account_nft_items**](AccountsApi.md#get_account_nft_items) | **GET** /v2/accounts/{account_id}/nfts | 
[**get_account_public_key**](AccountsApi.md#get_account_public_key) | **GET** /v2/accounts/{account_id}/publickey | 
[**get_account_subscriptions**](AccountsApi.md#get_account_subscriptions) | **GET** /v2/accounts/{account_id}/subscriptions | 
[**get_account_traces**](AccountsApi.md#get_account_traces) | **GET** /v2/accounts/{account_id}/traces | 
[**get_accounts**](AccountsApi.md#get_accounts) | **POST** /v2/accounts/_bulk | 
[**reindex_account**](AccountsApi.md#reindex_account) | **POST** /v2/accounts/{account_id}/reindex | 
[**search_accounts**](AccountsApi.md#search_accounts) | **GET** /v2/accounts/search | 



## account_dns_back_resolve

> crate::models::DomainNames account_dns_back_resolve(account_id)


Get account's domains

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::DomainNames**](DomainNames.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## address_parse

> crate::models::AddressParse200Response address_parse(account_id)


parse address and display in all formats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::AddressParse200Response**](addressParse_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account

> crate::models::Account get_account(account_id)


Get human-friendly information about an account without low-level details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_diff

> crate::models::GetAccountDiff200Response get_account_diff(account_id, start_date, end_date)


Get account's balance change

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**start_date** | **i64** |  | [required] |
**end_date** | **i64** |  | [required] |

### Return type

[**crate::models::GetAccountDiff200Response**](getAccountDiff_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_dns_expiring

> crate::models::DnsExpiring get_account_dns_expiring(account_id, period)


Get expiring account .ton dns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**period** | Option<**i32**> | number of days before expiration |  |

### Return type

[**crate::models::DnsExpiring**](DnsExpiring.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_event

> crate::models::AccountEvent get_account_event(account_id, event_id, accept_language, subject_only)


Get event for an account by event_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**event_id** | **String** | event ID or transaction hash in hex (without 0x) or base64url format | [required] |
**accept_language** | Option<**String**> |  |  |[default to en]
**subject_only** | Option<**bool**> | filter actions where requested account is not real subject (for example sender or receiver jettons) |  |[default to false]

### Return type

[**crate::models::AccountEvent**](AccountEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_events

> crate::models::AccountEvents get_account_events(account_id, limit, accept_language, initiator, subject_only, before_lt, start_date, end_date)


Get events for an account. Each event is built on top of a trace which is a series of transactions caused by one inbound message. TonAPI looks for known patterns inside the trace and splits the trace into actions, where a single action represents a meaningful high-level operation like a Jetton Transfer or an NFT Purchase. Actions are expected to be shown to users. It is advised not to build any logic on top of actions because actions can be changed at any time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**limit** | **i32** |  | [required] |
**accept_language** | Option<**String**> |  |  |[default to en]
**initiator** | Option<**bool**> | Show only events that are initiated by this account |  |[default to false]
**subject_only** | Option<**bool**> | filter actions where requested account is not real subject (for example sender or receiver jettons) |  |[default to false]
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


## get_account_jetton_history_by_id

> crate::models::AccountEvents get_account_jetton_history_by_id(account_id, jetton_id, limit, accept_language, before_lt, start_date, end_date)


Get the transfer jetton history for account and jetton

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**jetton_id** | **String** | jetton ID | [required] |
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


## get_account_jettons_balances

> crate::models::JettonsBalances get_account_jettons_balances(account_id, currencies)


Get all Jettons balances by owner address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**currencies** | Option<**String**> | accept ton and all possible fiat currencies, separated by commas |  |

### Return type

[**crate::models::JettonsBalances**](JettonsBalances.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_jettons_history

> crate::models::AccountEvents get_account_jettons_history(account_id, limit, accept_language, before_lt, start_date, end_date)


Get the transfer jettons history for account

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


## get_account_nft_items

> crate::models::NftItems get_account_nft_items(account_id, collection, limit, offset, indirect_ownership)


Get all NFT items by owner address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**collection** | Option<**String**> | nft collection |  |
**limit** | Option<**i32**> |  |  |[default to 1000]
**offset** | Option<**i32**> |  |  |[default to 0]
**indirect_ownership** | Option<**bool**> | Selling nft items in ton implemented usually via transfer items to special selling account. This option enables including items which owned not directly. |  |[default to false]

### Return type

[**crate::models::NftItems**](NftItems.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_public_key

> crate::models::GetAccountPublicKey200Response get_account_public_key(account_id)


Get public key by account id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::GetAccountPublicKey200Response**](getAccountPublicKey_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_subscriptions

> crate::models::Subscriptions get_account_subscriptions(account_id)


Get all subscriptions by wallet address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

[**crate::models::Subscriptions**](Subscriptions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_traces

> crate::models::TraceIds get_account_traces(account_id, limit)


Get traces for account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]

### Return type

[**crate::models::TraceIds**](TraceIDs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_accounts

> crate::models::Accounts get_accounts(get_accounts_request)


Get human-friendly information about several accounts without low-level details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_accounts_request** | Option<[**GetAccountsRequest**](GetAccountsRequest.md)> | a list of account ids |  |

### Return type

[**crate::models::Accounts**](Accounts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reindex_account

> reindex_account(account_id)


Update internal cache for a particular account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_accounts

> crate::models::FoundAccounts search_accounts(name)


Search by account domain name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::FoundAccounts**](FoundAccounts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

