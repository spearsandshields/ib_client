# Transactions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | will always be getTransactions | [optional]
**currency** | Option<**String**> | same as request | [optional]
**includes_real_time** | Option<**bool**> | Indicates whether current day and realtime data is included in the result | [optional]
**from** | Option<**f32**> | Period start date. Epoch time, GMT | [optional]
**to** | Option<**f32**> | Period end date. Epoch time, GMT | [optional]
**transactions** | Option<[**Vec<crate::models::TransactionsTransactions>**](transactions_transactions.md)> | Sorted by date descending | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


