# InlineResponse20022

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cfi_code** | Option<**String**> | Classification of Financial Instrument codes | [optional]
**symbol** | Option<**String**> | Underlying Symbol for contract | [optional]
**cusip** | Option<**String**> |  | [optional]
**expiry_full** | Option<**f32**> | Expiration Date in the format YYYYMMDD | [optional]
**con_id** | Option<**f32**> | IBKRs contract identifier | [optional]
**maturity_date** | Option<**f32**> | Date on which the underlying transaction settles if the option is exercised | [optional]
**industry** | Option<**String**> | Specific group of companies or businesses. | [optional]
**instrument_type** | Option<**String**> | Asset Class of the contract | [optional]
**trading_class** | Option<**String**> | Designation of the contract | [optional]
**valid_exchanges** | Option<**String**> | Comma separated list of exchanges or trading venues | [optional]
**allow_sell_long** | Option<**bool**> | Allowed to sell shares that you own | [optional]
**is_zero_commission_security** | Option<**bool**> | Supports zero commission trades | [optional]
**local_symbol** | Option<**String**> | Contracts symbol from primary exchange. For options it is the OCC symbol. | [optional]
**classifier** | Option<**String**> |  | [optional]
**currency** | Option<**String**> | Currency contract trades in | [optional]
**text** | Option<**String**> | Formatted contract parameters | [optional]
**underlying_con_id** | Option<**f32**> | IBKRs contract identifier for the underlying instrument | [optional]
**r_t_h** | Option<**bool**> | Provides trading outside of Regular Trading Hours | [optional]
**multiplier** | Option<**String**> | numerical value of each point of price movement | [optional]
**strike** | Option<**String**> | fixed price at which the owner of the option buys or sells the underlying | [optional]
**right** | Option<**String**> | Put or Call of the option | [optional]
**underlying_issuer** | Option<**String**> | Legal entity for underlying contract | [optional]
**contract_month** | Option<**String**> | Month the contract must be satisfied by making or accepting delivery | [optional]
**company_name** | Option<**String**> | Contracts company name | [optional]
**smart_available** | Option<**bool**> | Support IBKRs SMART routing | [optional]
**exchange** | Option<**String**> | Primary Exchange, Routing or Trading Venue | [optional]
**rules** | Option<[**Vec<crate::models::InlineResponse20022Rules>**](inline_response_200_22_rules.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


