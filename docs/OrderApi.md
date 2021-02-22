# \OrderApi

All URIs are relative to *https://localhost:5000/v1/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_account_id_order_order_id_delete**](OrderApi.md#iserver_account_account_id_order_order_id_delete) | **delete** /iserver/account/{accountId}/order/{orderId} | Cancel Order
[**iserver_account_account_id_order_order_id_post**](OrderApi.md#iserver_account_account_id_order_order_id_post) | **post** /iserver/account/{accountId}/order/{orderId} | Modify Order
[**iserver_account_account_id_order_post**](OrderApi.md#iserver_account_account_id_order_post) | **post** /iserver/account/{accountId}/order | Place Order
[**iserver_account_account_id_order_whatif_post**](OrderApi.md#iserver_account_account_id_order_whatif_post) | **post** /iserver/account/{accountId}/order/whatif | Preview Order
[**iserver_account_account_id_orders_post**](OrderApi.md#iserver_account_account_id_orders_post) | **post** /iserver/account/{accountId}/orders | Place Orders (Support bracket orders)
[**iserver_account_orders_fa_group_post**](OrderApi.md#iserver_account_orders_fa_group_post) | **post** /iserver/account/orders/{faGroup} | Place Orders for Financial Advisor Groups
[**iserver_account_orders_get**](OrderApi.md#iserver_account_orders_get) | **get** /iserver/account/orders | Live Orders
[**iserver_reply_replyid_post**](OrderApi.md#iserver_reply_replyid_post) | **post** /iserver/reply/{replyid} | Place Order Reply



## iserver_account_account_id_order_order_id_delete

> crate::models::InlineResponse20016 iserver_account_account_id_order_order_id_delete(account_id, order_id)
Cancel Order

Cancels an open order. Must call /iserver/accounts endpoint prior to cancelling an order. Use /iservers/account/orders endpoint to review open-order(s) and get latest order status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id, or fa group if deleting a group order | [required] |
**order_id** | **String** | Customer order id, use /iservers/account/orders endpoint to query orderId. | [required] |

### Return type

[**crate::models::InlineResponse20016**](inline_response_200_16.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_order_order_id_post

> Vec<crate::models::InlineResponse20015> iserver_account_account_id_order_order_id_post(account_id, order_id, body)
Modify Order

Modifies an open order. Must call /iserver/accounts endpoint prior to modifying an order. Use /iservers/account/orders endpoint to review open-order(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id, or fa group if modifying a group order | [required] |
**order_id** | **String** | Customer order id, use /iservers/account/orders endpoint to query orderId. | [required] |
**body** | [**ModifyOrder**](ModifyOrder.md) | modify-order request | [required] |

### Return type

[**Vec<crate::models::InlineResponse20015>**](inline_response_200_15.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_order_post

> Vec<crate::models::InlineResponse20012> iserver_account_account_id_order_post(account_id, body)
Place Order

Please note here, sometimes this endpoint alone can't make sure you submit the order successfully, you could receive some questions in the response, you have to to answer them in order to submit the order successfully. You can use \"/iserver/reply/{replyid}\" endpoint to answer questions 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**body** | [**OrderRequest**](OrderRequest.md) | order request info | [required] |

### Return type

[**Vec<crate::models::InlineResponse20012>**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_order_whatif_post

> crate::models::InlineResponse20014 iserver_account_account_id_order_whatif_post(account_id, body)
Preview Order

This endpoint allows you to preview order without actually submitting the order and you can get commission information in the response. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**body** | [**OrderRequest**](OrderRequest.md) | order info | [required] |

### Return type

[**crate::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_orders_post

> Vec<crate::models::InlineResponse20012> iserver_account_account_id_orders_post(account_id, body)
Place Orders (Support bracket orders)

You can pass a list of orders here 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | account id | [required] |
**body** | [**InlineObject4**](InlineObject4.md) |  | [required] |

### Return type

[**Vec<crate::models::InlineResponse20012>**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_orders_fa_group_post

> Vec<crate::models::InlineResponse20012> iserver_account_orders_fa_group_post(fa_group, body)
Place Orders for Financial Advisor Groups

Financial Advisors can use this endpoint to place an order for a specified group. To place an order for a specified account use the endpoint /iserver/account/{accountId}/order. More information about groups can be found in the [TWS Users' Guide:](https://guides.interactivebrokers.com/twsguide/twsguide.htm#usersguidebook/financialadvisors/create_an_account_group_for_share_allocation.htm). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fa_group** | **String** | financial advisor group | [required] |
**body** | [**OrderRequest**](OrderRequest.md) | order request info | [required] |

### Return type

[**Vec<crate::models::InlineResponse20012>**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_orders_get

> crate::models::InlineResponse20011 iserver_account_orders_get(body)
Live Orders

The endpoint is meant to be used in polling mode, e.g. requesting every x seconds. The response will contain two objects, one is notification, the other is orders.  Orders is the list of live orders (cancelled, filled, submitted).  Notifications contains information about execute orders as they happen, see status field. To receive streaming live orders the endpoint /ws can be used. Refer to [Streaming WebSocket Data](https://interactivebrokers.github.io/cpwebapi/RealtimeSubscription.html) for details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**InlineObject3**](InlineObject3.md)> |  |  |

### Return type

[**crate::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_reply_replyid_post

> Vec<crate::models::InlineResponse20013> iserver_reply_replyid_post(replyid, body)
Place Order Reply

Reply to questions when placing orders and submit orders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**replyid** | **String** | Please use the \"id\" from the response of \"Place Order\" endpoint | [required] |
**body** | [**InlineObject5**](InlineObject5.md) |  | [required] |

### Return type

[**Vec<crate::models::InlineResponse20013>**](inline_response_200_13.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

