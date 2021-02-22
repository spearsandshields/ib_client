# \PortfolioApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**portfolio_account_id_allocation_get**](PortfolioApi.md#portfolio_account_id_allocation_get) | **get** /portfolio/{accountId}/allocation | Account Allocation
[**portfolio_account_id_ledger_get**](PortfolioApi.md#portfolio_account_id_ledger_get) | **get** /portfolio/{accountId}/ledger | Account Ledger
[**portfolio_account_id_meta_get**](PortfolioApi.md#portfolio_account_id_meta_get) | **get** /portfolio/{accountId}/meta | Account Information
[**portfolio_account_id_position_conid_get**](PortfolioApi.md#portfolio_account_id_position_conid_get) | **get** /portfolio/{accountId}/position/{conid} | Position by Conid
[**portfolio_account_id_positions_invalidate_post**](PortfolioApi.md#portfolio_account_id_positions_invalidate_post) | **post** /portfolio/{accountId}/positions/invalidate | Invalidates the backend cache of the Portfolio
[**portfolio_account_id_positions_page_id_get**](PortfolioApi.md#portfolio_account_id_positions_page_id_get) | **get** /portfolio/{accountId}/positions/{pageId} | Portfolio Positions
[**portfolio_account_id_summary_get**](PortfolioApi.md#portfolio_account_id_summary_get) | **get** /portfolio/{accountId}/summary | Account Summary
[**portfolio_accounts_get**](PortfolioApi.md#portfolio_accounts_get) | **get** /portfolio/accounts | Portfolio Accounts
[**portfolio_allocation_post**](PortfolioApi.md#portfolio_allocation_post) | **post** /portfolio/allocation | Account Alloction (All Accounts)
[**portfolio_positions_conid_get**](PortfolioApi.md#portfolio_positions_conid_get) | **get** /portfolio/positions/{conid} | Positions by Conid
[**portfolio_subaccounts_get**](PortfolioApi.md#portfolio_subaccounts_get) | **get** /portfolio/subaccounts | List of Sub-Accounts



## portfolio_account_id_allocation_get

> Vec<serde_json::Value> portfolio_account_id_allocation_get(account_id)
Account Allocation

Information about the account's portfolio allocation by Asset Class, Industry and Category.  /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

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


## portfolio_account_id_position_conid_get

> Vec<serde_json::Value> portfolio_account_id_position_conid_get(account_id, conid)
Position by Conid

Returns a list of all positions matching the conid. For portfolio models the conid could be in more than one model, returning an array with the name of the model it belongs to.  /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**conid** | **i32** | contract id | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_positions_invalidate_post

> serde_json::Value portfolio_account_id_positions_invalidate_post(account_id)
Invalidates the backend cache of the Portfolio

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_positions_page_id_get

> Vec<serde_json::Value> portfolio_account_id_positions_page_id_get(account_id, page_id, model, sort, direction, period)
Portfolio Positions

Returns a list of positions for the given account. The endpoint supports paging, page's default size is 30 positions. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**page_id** | **String** | page id | [required] |[default to 0]
**model** | Option<**String**> | optional |  |
**sort** | Option<**String**> | declare the table to be sorted by which column |  |
**direction** | Option<**String**> | in which order, a means ascending - d means descending |  |
**period** | Option<**String**> | period for pnl column, can be 1D, 7D, 1M... |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

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


## portfolio_allocation_post

> Vec<serde_json::Value> portfolio_allocation_post(body)
Account Alloction (All Accounts)

Similar to /portfolio/{accountId}/allocation but returns a consolidated view of of all the accounts returned by /portfolio/accounts. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InlineObject8**](InlineObject8.md) |  | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_positions_conid_get

> crate::models::InlineResponse20031 portfolio_positions_conid_get(conid)
Positions by Conid

Returns an object of all positions matching the conid for all the selected accounts. For portfolio models the conid could be in more than one model, returning an array with the name of the model it belongs to. /portfolio/accounts or /portfolio/subaccounts must be called prior to this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **i32** | contract id | [required] |

### Return type

[**crate::models::InlineResponse20031**](inline_response_200_31.md)

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

