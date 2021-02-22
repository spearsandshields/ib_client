# \FYIApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fyi_deliveryoptions_device_id_delete**](FYIApi.md#fyi_deliveryoptions_device_id_delete) | **delete** /fyi/deliveryoptions/{deviceId} | Delete a device
[**fyi_deliveryoptions_device_post**](FYIApi.md#fyi_deliveryoptions_device_post) | **post** /fyi/deliveryoptions/device | Enable/Disable device option
[**fyi_deliveryoptions_email_put**](FYIApi.md#fyi_deliveryoptions_email_put) | **put** /fyi/deliveryoptions/email | Enable/Disable email option
[**fyi_deliveryoptions_get**](FYIApi.md#fyi_deliveryoptions_get) | **get** /fyi/deliveryoptions | Get delivery options
[**fyi_disclaimer_typecode_get**](FYIApi.md#fyi_disclaimer_typecode_get) | **get** /fyi/disclaimer/{typecode} | Get disclaimer for a certain kind of fyi
[**fyi_disclaimer_typecode_put**](FYIApi.md#fyi_disclaimer_typecode_put) | **put** /fyi/disclaimer/{typecode} | Mark disclaimer read
[**fyi_notifications_get**](FYIApi.md#fyi_notifications_get) | **get** /fyi/notifications | Get a list of notifications
[**fyi_notifications_more_get**](FYIApi.md#fyi_notifications_more_get) | **get** /fyi/notifications/more | Get more notifications based on a certain one
[**fyi_notifications_notification_id_put**](FYIApi.md#fyi_notifications_notification_id_put) | **put** /fyi/notifications/{notificationId} | Get a list of notifications
[**fyi_settings_get**](FYIApi.md#fyi_settings_get) | **get** /fyi/settings | Get a list of subscriptions
[**fyi_settings_typecode_post**](FYIApi.md#fyi_settings_typecode_post) | **post** /fyi/settings/{typecode} | Enable/Disable certain subscription
[**fyi_unreadnumber_get**](FYIApi.md#fyi_unreadnumber_get) | **get** /fyi/unreadnumber | Get unread number of fyis. The HTTP method POST is also supported.



## fyi_deliveryoptions_device_id_delete

> serde_json::Value fyi_deliveryoptions_device_id_delete(device_id)
Delete a device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | device ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_deliveryoptions_device_post

> crate::models::InlineResponse2004 fyi_deliveryoptions_device_post(body)
Enable/Disable device option

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InlineObject1**](InlineObject1.md) |  | [required] |

### Return type

[**crate::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_deliveryoptions_email_put

> crate::models::InlineResponse2004 fyi_deliveryoptions_email_put(enabled)
Enable/Disable email option

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enabled** | **String** | true/false | [required] |

### Return type

[**crate::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_deliveryoptions_get

> crate::models::InlineResponse2005 fyi_deliveryoptions_get()
Get delivery options

options for sending fyis to email and other devices 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_disclaimer_typecode_get

> crate::models::InlineResponse2003 fyi_disclaimer_typecode_get(typecode)
Get disclaimer for a certain kind of fyi

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**typecode** | **String** | fyi code, for example --M8, EA | [required] |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_disclaimer_typecode_put

> crate::models::InlineResponse2004 fyi_disclaimer_typecode_put(typecode)
Mark disclaimer read

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**typecode** | **String** | fyi code, for example --M8, EA | [required] |

### Return type

[**crate::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_notifications_get

> Vec<serde_json::Value> fyi_notifications_get(max, exclude, include)
Get a list of notifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max** | **String** | max number of fyis in response | [required] |
**exclude** | Option<**String**> | if set, don't set include |  |
**include** | Option<**String**> | if set, don't set exclude |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_notifications_more_get

> Vec<serde_json::Value> fyi_notifications_more_get(id)
Get more notifications based on a certain one

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | id of last notification in the list | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_notifications_notification_id_put

> serde_json::Value fyi_notifications_notification_id_put(notification_id)
Get a list of notifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | mark a notification read | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_settings_get

> Vec<crate::models::InlineResponse2002> fyi_settings_get()
Get a list of subscriptions

Return the current choices of subscriptions, we can toggle the option 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse2002>**](inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_settings_typecode_post

> serde_json::Value fyi_settings_typecode_post(typecode, body)
Enable/Disable certain subscription

Configure which typecode you would like to enable/disable. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**typecode** | **String** | fyi code | [required] |
**body** | [**InlineObject**](InlineObject.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fyi_unreadnumber_get

> crate::models::InlineResponse2001 fyi_unreadnumber_get()
Get unread number of fyis. The HTTP method POST is also supported.

Returns the total number of unread fyis 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

