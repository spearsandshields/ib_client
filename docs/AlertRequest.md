# AlertRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | Option<**i32**> | orderId is required when modifying alert. You can get it from /iserver/account/:accountId/alerts  | [optional]
**alert_name** | Option<**String**> | name of alert | [optional]
**alert_message** | Option<**String**> | The message you want to receive via email or text message | [optional]
**alert_repeatable** | Option<**i32**> | whether alert is repeatable or not, so value can only be 0 or 1, this has to be 1 for MTA alert | [optional]
**email** | Option<**String**> | email address to receive alert | [optional]
**send_message** | Option<**i32**> | whether allowing to send email or not, so value can only be 0 or 1,  | [optional]
**tif** | Option<**String**> | time in force, can only be GTC or GTD | [optional]
**expire_time** | Option<**String**> | format, YYYYMMDD-HH:mm:ss, please NOTE this will only work when tif is GTD  | [optional]
**outside_rth** | Option<**i32**> | value can only be 0 or 1, set to 1 if the alert can be triggered outside regular trading hours.  | [optional]
**i_tws_orders_only** | Option<**i32**> | value can only be 0 or 1, set to 1 to enable the alert only in IBKR mobile  | [optional]
**show_popup** | Option<**i32**> | value can only be 0 or 1, set to 1 to allow to show alert in pop-ups | [optional]
**tool_id** | Option<**i32**> | for MTA alert only, each user has a unique toolId and it will stay the same, do not send for normal alert  | [optional]
**play_audio** | Option<**String**> | audio message to play when alert is triggered | [optional]
**conditions** | Option<[**Vec<crate::models::AlertRequestConditions>**](alert_request_conditions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


