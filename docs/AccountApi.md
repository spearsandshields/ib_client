# \AccountApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_pnl_partitioned_get**](AccountApi.md#iserver_account_pnl_partitioned_get) | **get** /iserver/account/pnl/partitioned | PnL for the selected account
[**iserver_account_post**](AccountApi.md#iserver_account_post) | **post** /iserver/account | Switch Account
[**iserver_accounts_get**](AccountApi.md#iserver_accounts_get) | **get** /iserver/accounts | Brokerage Accounts
[**portfolio_account_id_ledger_get**](AccountApi.md#portfolio_account_id_ledger_get) | **get** /portfolio/{accountId}/ledger | Account Ledger
[**portfolio_account_id_meta_get**](AccountApi.md#portfolio_account_id_meta_get) | **get** /portfolio/{accountId}/meta | Account Information
[**portfolio_account_id_summary_get**](AccountApi.md#portfolio_account_id_summary_get) | **get** /portfolio/{accountId}/summary | Account Summary
[**portfolio_accounts_get**](AccountApi.md#portfolio_accounts_get) | **get** /portfolio/accounts | Portfolio Accounts
[**portfolio_subaccounts_get**](AccountApi.md#portfolio_subaccounts_get) | **get** /portfolio/subaccounts | List of Sub-Accounts



## iserver_account_pnl_partitioned_get

> crate::models::InlineResponse20025 iserver_account_pnl_partitioned_get()
PnL for the selected account

Returns an object containing PnL for the selected account and its models (if any). To receive streaming PnL the endpoint /ws can be used. Refer to [Streaming WebSocket Data](https://interactivebrokers.github.io/cpwebapi/RealtimeSubscription.html) for details. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20025**](inline_response_200_25.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_post

> crate::models::InlineResponse2007 iserver_account_post(body)
Switch Account

If an user has multiple accounts, and user wants to get orders, trades, etc. of an account other than currently selected account, then user can update the currently selected account using this API and then can fetch required information for the newly updated account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SetAccount**](SetAccount.md) | account id | [required] |

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_accounts_get

> crate::models::InlineResponse2006 iserver_accounts_get()
Brokerage Accounts

Returns a list of accounts the user has trading access to, their respective aliases and the currently selected account. Note this endpoint must be called before modifying an order or querying open orders.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_ledger_get

> crate::models::InlineResponse20030 portfolio_account_id_ledger_get(account_id)
Account Ledger

Information regarding settled cash, cash balances, etc. in the account's base currency and any other cash balances hold in other currencies.  /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint. The list of supported currencies is available at https://www.interactivebrokers.com/en/index.php?f=3185.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |

### Return type

[**crate::models::InlineResponse20030**](inline_response_200_30.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_meta_get

> Vec<crate::models::Account> portfolio_account_id_meta_get(account_id)
Account Information

Account information related to account Id /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |

### Return type

[**Vec<crate::models::Account>**](account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_summary_get

> crate::models::InlineResponse20029 portfolio_account_id_summary_get(account_id)
Account Summary

Returns information about margin, cash balances and other information related to specified account. See also /portfolio/{accountId}/ledger. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |

### Return type

[**crate::models::InlineResponse20029**](inline_response_200_29.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_accounts_get

> Vec<crate::models::Account> portfolio_accounts_get()
Portfolio Accounts

In non-tiered account structures, returns a list of accounts for which the user can view position and account information. This endpoint must be called prior  to calling other /portfolio endpoints for those accounts. For querying a list of accounts  which the user can trade, see /iserver/accounts. For a list of subaccounts in tiered  account structures (e.g. financial advisor or ibroker accounts) see /portfolio/subaccounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Account>**](account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_subaccounts_get

> crate::models::Account portfolio_subaccounts_get()
List of Sub-Accounts

Used in tiered account structures (such as financial advisor and ibroker accounts)  to return a list of sub-accounts for which the user can view position and  account-related information. This endpoint must be called prior to calling other  /portfolio endpoints for those subaccounts.  To query a list of accounts the user can trade, see /iserver/accounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Account**](account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

