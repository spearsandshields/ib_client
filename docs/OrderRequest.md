# OrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acct_id** | Option<**String**> | acctId is optional. It should be one of the accounts returned by /iserver/accounts. If not passed, the first one in the list is selected.  | [optional]
**conid** | Option<**i32**> | conid is the identifier of the security you want to trade, you can find the conid with /iserver/secdef/search.  | [optional]
**sec_type** | Option<**String**> | conid:type for example 265598:STK | [optional]
**c_oid** | Option<**String**> | Customer Order ID. An arbitraty string that can be used to identify the order, e.g \"my-fb-order\". The value must be unique for a 24h span. Please do not set this value for child orders when placing a bracket order.  | [optional]
**parent_id** | Option<**String**> | When placing bracket orders, the child parentId must be equal to the cOId (customer order id) of the parent.  | [optional]
**order_type** | Option<**String**> | orderType can be one of MKT (Market), LMT (Limit), STP (Stop) or STP_LIMIT (stop limit)  | [optional]
**listing_exchange** | Option<**String**> | listingExchange is optional. By default we use \"SMART\" routing. Possible values are available via this end point: /v1/portal/iserver/contract/{{conid}}/info, see valid_exchange: e.g: SMART,AMEX,NYSE, CBOE,ISE,CHX,ARCA,ISLAND,DRCTEDGE,BEX,BATS,EDGEA,CSFBALGO,JE FFALGO,BYX,IEX,FOXRIVER,TPLUS1,NYSENAT,PSX  | [optional]
**outside_rth** | Option<**bool**> | set to true if the order can be executed outside regular trading hours.  | [optional]
**price** | Option<**f32**> | optional if order is MKT, for LMT, this is the limit price. For STP this is the stop price.  | [optional]
**side** | Option<**String**> | SELL or BUY | [optional]
**ticker** | Option<**String**> |  | [optional]
**tif** | Option<**String**> | GTC (Good Till Cancel) or DAY. DAY orders are automatically cancelled at the end of the Day or Trading hours.  | [optional]
**referrer** | Option<**String**> | for example QuickTrade | [optional]
**quantity** | Option<**f32**> | usually integer, for some special cases can be float numbers | [optional]
**fx_qty** | Option<**f32**> | double number, this is the cash quantity field which can only be used for FX conversion order.  | [optional]
**use_adaptive** | Option<**bool**> | If true, the system will use the Adaptive Algo to submit the order https://www.interactivebrokers.com/en/index.php?f=19091  | [optional]
**is_currency_conversion** | Option<**bool**> | set to true if the order is a FX conversion order  | [optional]
**allocation_method** | Option<**String**> | Set the allocation method when placing an order using an FA account for a group Possible allocation methods are \"NetLiquidity\", \"AvailableEquity\", \"EqualQuantity\" and \"PctChange\".  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


