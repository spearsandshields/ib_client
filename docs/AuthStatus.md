# AuthStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authenticated** | Option<**bool**> | Brokerage session is authenticated | [optional]
**connected** | Option<**bool**> | Connected to backend | [optional]
**competing** | Option<**bool**> | Brokerage session is competing, e.g. user is logged in to IBKR Mobile, WebTrader, TWS or other trading platforms. | [optional]
**fail** | Option<**String**> | Authentication failed, why. | [optional]
**message** | Option<**String**> | System messages that may affect trading | [optional]
**prompts** | Option<**Vec<String>**> | Prompt messages that may affect trading or the account | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


