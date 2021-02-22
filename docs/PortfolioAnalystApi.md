# \PortfolioAnalystApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pa_performance_post**](PortfolioAnalystApi.md#pa_performance_post) | **post** /pa/performance | Account Performance
[**pa_summary_post**](PortfolioAnalystApi.md#pa_summary_post) | **post** /pa/summary | Account Balance's Summary
[**pa_transactions_post**](PortfolioAnalystApi.md#pa_transactions_post) | **post** /pa/transactions | Position's Transaction History



## pa_performance_post

> crate::models::Performance pa_performance_post(body)
Account Performance

Returns the performance (MTM) for the given accounts, if more than one account is passed, the result is consolidated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InlineObject9**](InlineObject9.md) |  | [required] |

### Return type

[**crate::models::Performance**](performance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pa_summary_post

> crate::models::Summary pa_summary_post(body)
Account Balance's Summary

Returns a summary of all account balances for the given accounts, if more than one account is passed, the result is consolidated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InlineObject10**](InlineObject10.md) |  | [required] |

### Return type

[**crate::models::Summary**](summary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pa_transactions_post

> crate::models::Transactions pa_transactions_post(body)
Position's Transaction History

transaction history for a given number of conids and accounts. Types of transactions include dividend payments, buy and sell transactions, transfers. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InlineObject11**](InlineObject11.md) |  | [required] |

### Return type

[**crate::models::Transactions**](transactions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

