# HistoryData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Underlying Symbol of the corresponding contract | [optional]
**text** | Option<**String**> | companyName of the corresponding contract | [optional]
**price_factor** | Option<**i32**> | priceFactor is price increment obtained from display rule | [optional]
**start_time** | Option<**String**> | start date time in the format YYYYMMDD-HH:mm:ss | [optional]
**high** | Option<**String**> | High value during this time series with format %h/%v/%t. %h is the high price (scaled by priceFactor), %v is volume (volume factor will always be 100 (reported volume = actual volume/100)) and %t is minutes from start time of the chart  | [optional]
**low** | Option<**String**> | Low value during this time series with format %l/%v/%t. %l is the low price (scaled by priceFactor), %v is volume (volume factor will always be 100 (reported volume = actual volume/100)) and %t is minutes from start time of the chart  | [optional]
**time_period** | Option<**String**> | The duration for the historical data request | [optional]
**bar_length** | Option<**i32**> | The number of seconds in a bar | [optional]
**md_availability** | Option<**String**> | Market Data Availability. The field may contain two chars. The first char is the primary code: S = Streaming, R = Realtime, D = Delayed, Z = Frozen, Y = Frozen Delayed. The second char is the secondary code: P = Snapshot Available, p = Consolidated.  | [optional]
**mkt_data_delay** | Option<**i32**> | The time it takes, in milliseconds, to process the historical data request | [optional]
**outside_rth** | Option<**bool**> | The historical data returned includes outside of regular trading hours  | [optional]
**trading_day_duration** | Option<**i32**> | The number of seconds in the trading day | [optional]
**volume_factor** | Option<**i32**> |  | [optional]
**price_display_rule** | Option<**i32**> |  | [optional]
**price_display_value** | Option<**String**> |  | [optional]
**negative_capable** | Option<**bool**> |  | [optional]
**message_version** | Option<**i32**> |  | [optional]
**data** | Option<[**Vec<crate::models::HistoryDataData>**](history_data_data.md)> |  | [optional]
**points** | Option<**i32**> | total number of points | [optional]
**travel_time** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


