# \DnsApi

All URIs are relative to *https://tonapi.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dns_resolve**](DnsApi.md#dns_resolve) | **GET** /v2/dns/{domain_name}/resolve | 
[**get_all_auctions**](DnsApi.md#get_all_auctions) | **GET** /v2/dns/auctions | 
[**get_dns_info**](DnsApi.md#get_dns_info) | **GET** /v2/dns/{domain_name} | 
[**get_domain_bids**](DnsApi.md#get_domain_bids) | **GET** /v2/dns/{domain_name}/bids | 



## dns_resolve

> crate::models::DnsRecord dns_resolve(domain_name)


DNS resolve for domain name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | domain name with .ton or .t.me | [required] |

### Return type

[**crate::models::DnsRecord**](DnsRecord.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_auctions

> crate::models::Auctions get_all_auctions(tld)


Get all auctions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tld** | Option<**String**> | domain filter for current auctions \"ton\" or \"t.me\" |  |

### Return type

[**crate::models::Auctions**](Auctions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dns_info

> crate::models::DomainInfo get_dns_info(domain_name)


Get full information about domain name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | domain name with .ton or .t.me | [required] |

### Return type

[**crate::models::DomainInfo**](DomainInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_bids

> crate::models::DomainBids get_domain_bids(domain_name)


Get domain bids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | domain name with .ton or .t.me | [required] |

### Return type

[**crate::models::DomainBids**](DomainBids.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

