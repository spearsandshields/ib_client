# \SessionApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_auth_status_post**](SessionApi.md#iserver_auth_status_post) | **post** /iserver/auth/status | Authentication Status
[**iserver_reauthenticate_post**](SessionApi.md#iserver_reauthenticate_post) | **post** /iserver/reauthenticate | Tries to re-authenticate to Brokerage
[**logout_post**](SessionApi.md#logout_post) | **post** /logout | Ends the current session
[**sso_validate_get**](SessionApi.md#sso_validate_get) | **get** /sso/validate | Validate SSO
[**tickle_post**](SessionApi.md#tickle_post) | **post** /tickle | Ping the server to keep the session open



## iserver_auth_status_post

> crate::models::AuthStatus iserver_auth_status_post()
Authentication Status

Current Authentication status to the Brokerage system. Market Data and Trading is not possible if not authenticated, e.g. authenticated shows false

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AuthStatus**](authStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_reauthenticate_post

> crate::models::AuthStatus iserver_reauthenticate_post()
Tries to re-authenticate to Brokerage

When using the CP Gateway, this endpoint provides a way to reauthenticate to the Brokerage system as long as there is a valid SSO session, see /sso/validate. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AuthStatus**](authStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout_post

> crate::models::InlineResponse200 logout_post()
Ends the current session

Logs the user out of the gateway session. Any further activity requires re-authentication.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sso_validate_get

> crate::models::InlineResponse20032 sso_validate_get()
Validate SSO

Validates the current session for the SSO user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20032**](inline_response_200_32.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tickle_post

> tickle_post()
Ping the server to keep the session open

If the gateway has not received any requests for several minutes an open session will automatically timeout. The tickle endpoint pings the server to prevent the session from ending.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

