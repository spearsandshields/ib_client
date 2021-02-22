# InlineResponse20022Rules

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | Option<**String**> |  | [optional]
**order_types** | Option<[**Vec<crate::models::InlineResponse20022OrderTypes>**](inline_response_200_22_orderTypes.md)> |  | [optional]
**ibalgo_types** | Option<[**Vec<crate::models::InlineResponse20022IbalgoTypes>**](inline_response_200_22_ibalgoTypes.md)> |  | [optional]
**fraq_types** | Option<[**Vec<crate::models::InlineResponse20022FraqTypes>**](inline_response_200_22_fraqTypes.md)> |  | [optional]
**cqt_types** | Option<[**Vec<crate::models::InlineResponse20022CqtTypes>**](inline_response_200_22_cqtTypes.md)> |  | [optional]
**order_defaults** | Option<[**Vec<crate::models::InlineResponse20022OrderDefaults>**](inline_response_200_22_orderDefaults.md)> | If object returned will provide the defaults based on user settings | [optional]
**order_types_outside** | Option<[**Vec<crate::models::InlineResponse20022OrderTypesOutside>**](inline_response_200_22_orderTypesOutside.md)> |  | [optional]
**default_size** | Option<**i32**> | Default quantity | [optional]
**cash_size** | Option<**i32**> | cash value | [optional]
**size_increment** | Option<**i32**> | increment quantity value | [optional]
**tif_types** | Option<[**Vec<crate::models::InlineResponse20022TifTypes>**](inline_response_200_22_tifTypes.md)> |  | [optional]
**default_tif** | Option<**String**> | Default time in force value | [optional]
**limit_price** | Option<**f32**> | Limit price | [optional]
**stopprice** | Option<**f32**> | Stop price | [optional]
**order_origination** | Option<**f32**> | Order origin designation for US securities options and Options Clearing Corporation | [optional]
**preview** | Option<**bool**> | order preview required | [optional]
**display_size** | Option<**f32**> |  | [optional]
**fraq_int** | Option<**f32**> | decimal places for fractional order size | [optional]
**cash_ccy** | Option<**String**> | Cash currency for the contract | [optional]
**cash_qty_incr** | Option<**f32**> | Increment value for cash quantity | [optional]
**price_magnifier** | Option<**f32**> | Price Magnifier | [optional]
**negative_capable** | Option<**bool**> | trading negative price support | [optional]
**increment** | Option<**f32**> | Price increment value | [optional]
**increment_digits** | Option<**i32**> | Number of digits for price increment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


