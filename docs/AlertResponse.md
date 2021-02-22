# AlertResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | Option<**String**> | account id | [optional]
**order_id** | Option<**i32**> |  | [optional]
**alert_name** | Option<**String**> | name of alert | [optional]
**alert_message** | Option<**String**> | The message you want to receive via email or text message | [optional]
**alert_active** | Option<**i32**> | whether alert is active or not, so value can only be 0 or 1 | [optional]
**alert_repeatable** | Option<**i32**> | whether alert is repeatable or not, so value can only be 0 or 1 | [optional]
**alert_email** | Option<**String**> | email address to receive alert | [optional]
**alert_send_message** | Option<**i32**> | whether allowing to send email or not, so value can only be 0 or 1,  | [optional]
**tif** | Option<**String**> | time in force, can only be GTC or GTD | [optional]
**expire_time** | Option<**String**> | format, YYYYMMDD-HH:mm:ss  | [optional]
**order_status** | Option<**String**> | status of alert | [optional]
**outside_rth** | Option<**i32**> | value can only be 0 or 1, set to 1 if the alert can be triggered outside regular trading hours.  | [optional]
**itws_orders_only** | Option<**i32**> | value can only be 0 or 1, set to 1 to enable the alert only in IBKR mobile  | [optional]
**alert_show_popup** | Option<**i32**> | value can only be 0 or 1, set to 1 to allow to show alert in pop-ups | [optional]
**alert_triggered** | Option<**bool**> | whether the alert has been triggered | [optional]
**order_not_editable** | Option<**bool**> | whether the alert can be edited | [optional]
**tool_id** | Option<**i32**> | for MTA alert only, each user has a unique toolId and it will stay the same, do not send for normal alert  | [optional]
**alert_play_audio** | Option<**String**> | audio message to play when alert is triggered | [optional]
**alert_mta_currency** | Option<**String**> | MTA alert only | [optional]
**alert_mta_defaults** | Option<**String**> | MTA alert only | [optional]
**time_zone** | Option<**String**> | MTA alert only | [optional]
**alert_default_type** | Option<**String**> | MTA alert only | [optional]
**condition_size** | Option<**i32**> | size of conditions array | [optional]
**condition_outside_rth** | Option<**i32**> | whether allowing the condition can be triggered outside of regular trading hours, 1 means allow | [optional]
**conditions** | Option<[**Vec<crate::models::AlertResponseConditions>**](alert_response_conditions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


