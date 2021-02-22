# \PnLApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_pnl_partitioned_get**](PnLApi.md#iserver_account_pnl_partitioned_get) | **get** /iserver/account/pnl/partitioned | PnL for the selected account



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

