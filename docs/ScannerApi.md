# \ScannerApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_scanner_params_get**](ScannerApi.md#iserver_scanner_params_get) | **get** /iserver/scanner/params | Scanner Parameters
[**iserver_scanner_run_post**](ScannerApi.md#iserver_scanner_run_post) | **post** /iserver/scanner/run | run scanner to get a list of contracts



## iserver_scanner_params_get

> crate::models::InlineResponse20023 iserver_scanner_params_get()
Scanner Parameters

Returns an object contains four lists contain all parameters for scanners

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20023**](inline_response_200_23.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_scanner_run_post

> Vec<crate::models::InlineResponse20024> iserver_scanner_run_post(body)
run scanner to get a list of contracts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ScannerParams**](ScannerParams.md) | scanner-params request | [required] |

### Return type

[**Vec<crate::models::InlineResponse20024>**](inline_response_200_24.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

