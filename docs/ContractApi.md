# \ContractApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_contract_conid_info_and_rules_get**](ContractApi.md#iserver_contract_conid_info_and_rules_get) | **get** /iserver/contract/{conid}/info-and-rules | Info and Rules
[**iserver_contract_conid_info_get**](ContractApi.md#iserver_contract_conid_info_get) | **get** /iserver/contract/{conid}/info | Contract Details
[**iserver_secdef_info_get**](ContractApi.md#iserver_secdef_info_get) | **get** /iserver/secdef/info | Secdef Info
[**iserver_secdef_search_post**](ContractApi.md#iserver_secdef_search_post) | **post** /iserver/secdef/search | Search by Symbol or Name
[**iserver_secdef_strikes_get**](ContractApi.md#iserver_secdef_strikes_get) | **get** /iserver/secdef/strikes | Search Strikes
[**trsrv_futures_get**](ContractApi.md#trsrv_futures_get) | **get** /trsrv/futures | Security Futures by Symbol
[**trsrv_secdef_post**](ContractApi.md#trsrv_secdef_post) | **post** /trsrv/secdef | Secdef by Conid
[**trsrv_secdef_schedule_get**](ContractApi.md#trsrv_secdef_schedule_get) | **get** /trsrv/secdef/schedule | Get trading schedule for symbol
[**trsrv_stocks_get**](ContractApi.md#trsrv_stocks_get) | **get** /trsrv/stocks | Security Stocks by Symbol



## iserver_contract_conid_info_and_rules_get

> crate::models::InlineResponse20022 iserver_contract_conid_info_and_rules_get(conid, is_buy)
Info and Rules

Returns trading related rules and info for contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** | IBKR contract identifier | [required] |
**is_buy** | **bool** | Side of the market rules apply too. Set to true for Buy Orders, set to false for Sell Orders | [required] |

### Return type

[**crate::models::InlineResponse20022**](inline_response_200_22.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_contract_conid_info_get

> crate::models::Contract iserver_contract_conid_info_get(conid)
Contract Details

Using the Contract Identifier get contract info. You can use this to prefill your order before you submit an order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** | contract id | [required] |

### Return type

[**crate::models::Contract**](contract.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_secdef_info_get

> serde_json::Value iserver_secdef_info_get(conid, sectype, month, exchange, strike, right)
Secdef Info

Provides Contract Details of Futures, Options, Warrants, Cash and CFDs based on conid. To get the strike price for Options/Warrants use \"/iserver/secdef/strikes\" endpoint. Must call /secdef/search for the underlying contract first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** | underlying contract id | [required] |
**sectype** | **String** | FUT/OPT/WAR/CASH/CFD | [required] |
**month** | Option<**String**> | contract month, only required for FUT/OPT/WAR in the format MMMYY, example JAN00 |  |
**exchange** | Option<**String**> | optional, default is SMART |  |
**strike** | Option<**String**> | optional, only required for OPT/WAR |  |
**right** | Option<**String**> | C for call, P for put |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_secdef_search_post

> Vec<crate::models::InlineResponse20020> iserver_secdef_search_post(symbol)
Search by Symbol or Name

Search by underlying or company name. Relays back what derivative contract(s) it has. This endpoint must be called before using /secdef/info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | [**InlineObject6**](InlineObject6.md) |  | [required] |

### Return type

[**Vec<crate::models::InlineResponse20020>**](inline_response_200_20.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_secdef_strikes_get

> crate::models::InlineResponse20021 iserver_secdef_strikes_get(conid, sectype, month, exchange)
Search Strikes

Query strikes for Options/Warrants. For available contract months and exchanges use \"/iserver/secdef/search\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** | contract id | [required] |
**sectype** | **String** | OPT/WAR | [required] |
**month** | **String** | contract month | [required] |
**exchange** | Option<**String**> | optional, default is SMART |  |

### Return type

[**crate::models::InlineResponse20021**](inline_response_200_21.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trsrv_futures_get

> crate::models::InlineResponse20027 trsrv_futures_get(symbols)
Security Futures by Symbol

Returns a list of non-expired future contracts for given symbol(s)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbols** | **String** | list of case-sensitive symbols separated by comma | [required] |

### Return type

[**crate::models::InlineResponse20027**](inline_response_200_27.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trsrv_secdef_post

> Vec<serde_json::Value> trsrv_secdef_post(body)
Secdef by Conid

Returns a list of security definitions for the given conids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InlineObject7**](InlineObject7.md) |  | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trsrv_secdef_schedule_get

> crate::models::InlineResponse20026 trsrv_secdef_schedule_get(asset_class, symbol, exchange)
Get trading schedule for symbol

Returns the trading schedule up to a month for the requested contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_class** | **String** | specify the asset class of the contract. Available values-- Stock: STK, Option: OPT, Future: FUT, Contract For Difference: CFD, Warrant: WAR, Forex: SWP, Mutual Fund: FND, Bond: BND, Inter-Commodity Spreads: ICS  | [required] |
**symbol** | **String** | Underlying Symbol for specified contract, for example 'AAPL' for US Stock - Apple Inc. | [required] |
**exchange** | Option<**String**> | Native exchange for contract, for example 'NASDAQ' for US Stock - Apple Inc. |  |

### Return type

[**crate::models::InlineResponse20026**](inline_response_200_26.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trsrv_stocks_get

> crate::models::InlineResponse20028 trsrv_stocks_get(symbols)
Security Stocks by Symbol

Returns an object contains all stock contracts for given symbol(s)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbols** | **String** | list of upper-sensitive symbols separated by comma | [required] |

### Return type

[**crate::models::InlineResponse20028**](inline_response_200_28.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

