# Rust API client for openapi

Client Portal Web API 
You would need to set up certificates to use this client in production environment.

For local development, you can configure the gateway to turn off ssl validation

under clientportal.gw/root/conf.yaml, set
```
listenSsl: false
```
## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Documentation for API Endpoints

All URIs are relative to *https://localhost:5000/v1/api*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountApi* | [**iserver_account_pnl_partitioned_get**](docs/AccountApi.md#iserver_account_pnl_partitioned_get) | **get** /iserver/account/pnl/partitioned | PnL for the selected account
*AccountApi* | [**iserver_account_post**](docs/AccountApi.md#iserver_account_post) | **post** /iserver/account | Switch Account
*AccountApi* | [**iserver_accounts_get**](docs/AccountApi.md#iserver_accounts_get) | **get** /iserver/accounts | Brokerage Accounts
*AccountApi* | [**portfolio_account_id_ledger_get**](docs/AccountApi.md#portfolio_account_id_ledger_get) | **get** /portfolio/{accountId}/ledger | Account Ledger
*AccountApi* | [**portfolio_account_id_meta_get**](docs/AccountApi.md#portfolio_account_id_meta_get) | **get** /portfolio/{accountId}/meta | Account Information
*AccountApi* | [**portfolio_account_id_summary_get**](docs/AccountApi.md#portfolio_account_id_summary_get) | **get** /portfolio/{accountId}/summary | Account Summary
*AccountApi* | [**portfolio_accounts_get**](docs/AccountApi.md#portfolio_accounts_get) | **get** /portfolio/accounts | Portfolio Accounts
*AccountApi* | [**portfolio_subaccounts_get**](docs/AccountApi.md#portfolio_subaccounts_get) | **get** /portfolio/subaccounts | List of Sub-Accounts
*AlertApi* | [**iserver_account_account_id_alert_activate_post**](docs/AlertApi.md#iserver_account_account_id_alert_activate_post) | **post** /iserver/account/{accountId}/alert/activate | Activate or deactivate an alert
*AlertApi* | [**iserver_account_account_id_alert_alert_id_delete**](docs/AlertApi.md#iserver_account_account_id_alert_alert_id_delete) | **delete** /iserver/account/{accountId}/alert/{alertId} | Delete an alert
*AlertApi* | [**iserver_account_account_id_alert_post**](docs/AlertApi.md#iserver_account_account_id_alert_post) | **post** /iserver/account/{accountId}/alert | Create or modify alert
*AlertApi* | [**iserver_account_account_id_alerts_get**](docs/AlertApi.md#iserver_account_account_id_alerts_get) | **get** /iserver/account/{accountId}/alerts | Get a list of available alerts
*AlertApi* | [**iserver_account_alert_id_get**](docs/AlertApi.md#iserver_account_alert_id_get) | **get** /iserver/account/alert/:id | Get details of an alert
*AlertApi* | [**iserver_account_mta_get**](docs/AlertApi.md#iserver_account_mta_get) | **get** /iserver/account/mta | Get MTA alert
*ContractApi* | [**iserver_contract_conid_info_and_rules_get**](docs/ContractApi.md#iserver_contract_conid_info_and_rules_get) | **get** /iserver/contract/{conid}/info-and-rules | Info and Rules
*ContractApi* | [**iserver_contract_conid_info_get**](docs/ContractApi.md#iserver_contract_conid_info_get) | **get** /iserver/contract/{conid}/info | Contract Details
*ContractApi* | [**iserver_secdef_info_get**](docs/ContractApi.md#iserver_secdef_info_get) | **get** /iserver/secdef/info | Secdef Info
*ContractApi* | [**iserver_secdef_search_post**](docs/ContractApi.md#iserver_secdef_search_post) | **post** /iserver/secdef/search | Search by Symbol or Name
*ContractApi* | [**iserver_secdef_strikes_get**](docs/ContractApi.md#iserver_secdef_strikes_get) | **get** /iserver/secdef/strikes | Search Strikes
*ContractApi* | [**trsrv_futures_get**](docs/ContractApi.md#trsrv_futures_get) | **get** /trsrv/futures | Security Futures by Symbol
*ContractApi* | [**trsrv_secdef_post**](docs/ContractApi.md#trsrv_secdef_post) | **post** /trsrv/secdef | Secdef by Conid
*ContractApi* | [**trsrv_secdef_schedule_get**](docs/ContractApi.md#trsrv_secdef_schedule_get) | **get** /trsrv/secdef/schedule | Get trading schedule for symbol
*ContractApi* | [**trsrv_stocks_get**](docs/ContractApi.md#trsrv_stocks_get) | **get** /trsrv/stocks | Security Stocks by Symbol
*FYIApi* | [**fyi_deliveryoptions_device_id_delete**](docs/FYIApi.md#fyi_deliveryoptions_device_id_delete) | **delete** /fyi/deliveryoptions/{deviceId} | Delete a device
*FYIApi* | [**fyi_deliveryoptions_device_post**](docs/FYIApi.md#fyi_deliveryoptions_device_post) | **post** /fyi/deliveryoptions/device | Enable/Disable device option
*FYIApi* | [**fyi_deliveryoptions_email_put**](docs/FYIApi.md#fyi_deliveryoptions_email_put) | **put** /fyi/deliveryoptions/email | Enable/Disable email option
*FYIApi* | [**fyi_deliveryoptions_get**](docs/FYIApi.md#fyi_deliveryoptions_get) | **get** /fyi/deliveryoptions | Get delivery options
*FYIApi* | [**fyi_disclaimer_typecode_get**](docs/FYIApi.md#fyi_disclaimer_typecode_get) | **get** /fyi/disclaimer/{typecode} | Get disclaimer for a certain kind of fyi
*FYIApi* | [**fyi_disclaimer_typecode_put**](docs/FYIApi.md#fyi_disclaimer_typecode_put) | **put** /fyi/disclaimer/{typecode} | Mark disclaimer read
*FYIApi* | [**fyi_notifications_get**](docs/FYIApi.md#fyi_notifications_get) | **get** /fyi/notifications | Get a list of notifications
*FYIApi* | [**fyi_notifications_more_get**](docs/FYIApi.md#fyi_notifications_more_get) | **get** /fyi/notifications/more | Get more notifications based on a certain one
*FYIApi* | [**fyi_notifications_notification_id_put**](docs/FYIApi.md#fyi_notifications_notification_id_put) | **put** /fyi/notifications/{notificationId} | Get a list of notifications
*FYIApi* | [**fyi_settings_get**](docs/FYIApi.md#fyi_settings_get) | **get** /fyi/settings | Get a list of subscriptions
*FYIApi* | [**fyi_settings_typecode_post**](docs/FYIApi.md#fyi_settings_typecode_post) | **post** /fyi/settings/{typecode} | Enable/Disable certain subscription
*FYIApi* | [**fyi_unreadnumber_get**](docs/FYIApi.md#fyi_unreadnumber_get) | **get** /fyi/unreadnumber | Get unread number of fyis. The HTTP method POST is also supported.
*IBCustApi* | [**ibcust_entity_info_get**](docs/IBCustApi.md#ibcust_entity_info_get) | **get** /ibcust/entity/info | IBCust Entity Info
*MarketDataApi* | [**iserver_marketdata_conid_unsubscribe_get**](docs/MarketDataApi.md#iserver_marketdata_conid_unsubscribe_get) | **get** /iserver/marketdata/{conid}/unsubscribe | Market Data Cancel (Single)
*MarketDataApi* | [**iserver_marketdata_history_get**](docs/MarketDataApi.md#iserver_marketdata_history_get) | **get** /iserver/marketdata/history | Market Data History
*MarketDataApi* | [**iserver_marketdata_snapshot_get**](docs/MarketDataApi.md#iserver_marketdata_snapshot_get) | **get** /iserver/marketdata/snapshot | Market Data
*MarketDataApi* | [**iserver_marketdata_unsubscribeall_get**](docs/MarketDataApi.md#iserver_marketdata_unsubscribeall_get) | **get** /iserver/marketdata/unsubscribeall | Market Data Cancel (All)
*OrderApi* | [**iserver_account_account_id_order_order_id_delete**](docs/OrderApi.md#iserver_account_account_id_order_order_id_delete) | **delete** /iserver/account/{accountId}/order/{orderId} | Cancel Order
*OrderApi* | [**iserver_account_account_id_order_order_id_post**](docs/OrderApi.md#iserver_account_account_id_order_order_id_post) | **post** /iserver/account/{accountId}/order/{orderId} | Modify Order
*OrderApi* | [**iserver_account_account_id_order_post**](docs/OrderApi.md#iserver_account_account_id_order_post) | **post** /iserver/account/{accountId}/order | Place Order
*OrderApi* | [**iserver_account_account_id_order_whatif_post**](docs/OrderApi.md#iserver_account_account_id_order_whatif_post) | **post** /iserver/account/{accountId}/order/whatif | Preview Order
*OrderApi* | [**iserver_account_account_id_orders_post**](docs/OrderApi.md#iserver_account_account_id_orders_post) | **post** /iserver/account/{accountId}/orders | Place Orders (Support bracket orders)
*OrderApi* | [**iserver_account_orders_fa_group_post**](docs/OrderApi.md#iserver_account_orders_fa_group_post) | **post** /iserver/account/orders/{faGroup} | Place Orders for Financial Advisor Groups
*OrderApi* | [**iserver_account_orders_get**](docs/OrderApi.md#iserver_account_orders_get) | **get** /iserver/account/orders | Live Orders
*OrderApi* | [**iserver_reply_replyid_post**](docs/OrderApi.md#iserver_reply_replyid_post) | **post** /iserver/reply/{replyid} | Place Order Reply
*PnLApi* | [**iserver_account_pnl_partitioned_get**](docs/PnLApi.md#iserver_account_pnl_partitioned_get) | **get** /iserver/account/pnl/partitioned | PnL for the selected account
*PortfolioApi* | [**portfolio_account_id_allocation_get**](docs/PortfolioApi.md#portfolio_account_id_allocation_get) | **get** /portfolio/{accountId}/allocation | Account Allocation
*PortfolioApi* | [**portfolio_account_id_ledger_get**](docs/PortfolioApi.md#portfolio_account_id_ledger_get) | **get** /portfolio/{accountId}/ledger | Account Ledger
*PortfolioApi* | [**portfolio_account_id_meta_get**](docs/PortfolioApi.md#portfolio_account_id_meta_get) | **get** /portfolio/{accountId}/meta | Account Information
*PortfolioApi* | [**portfolio_account_id_position_conid_get**](docs/PortfolioApi.md#portfolio_account_id_position_conid_get) | **get** /portfolio/{accountId}/position/{conid} | Position by Conid
*PortfolioApi* | [**portfolio_account_id_positions_invalidate_post**](docs/PortfolioApi.md#portfolio_account_id_positions_invalidate_post) | **post** /portfolio/{accountId}/positions/invalidate | Invalidates the backend cache of the Portfolio
*PortfolioApi* | [**portfolio_account_id_positions_page_id_get**](docs/PortfolioApi.md#portfolio_account_id_positions_page_id_get) | **get** /portfolio/{accountId}/positions/{pageId} | Portfolio Positions
*PortfolioApi* | [**portfolio_account_id_summary_get**](docs/PortfolioApi.md#portfolio_account_id_summary_get) | **get** /portfolio/{accountId}/summary | Account Summary
*PortfolioApi* | [**portfolio_accounts_get**](docs/PortfolioApi.md#portfolio_accounts_get) | **get** /portfolio/accounts | Portfolio Accounts
*PortfolioApi* | [**portfolio_allocation_post**](docs/PortfolioApi.md#portfolio_allocation_post) | **post** /portfolio/allocation | Account Alloction (All Accounts)
*PortfolioApi* | [**portfolio_positions_conid_get**](docs/PortfolioApi.md#portfolio_positions_conid_get) | **get** /portfolio/positions/{conid} | Positions by Conid
*PortfolioApi* | [**portfolio_subaccounts_get**](docs/PortfolioApi.md#portfolio_subaccounts_get) | **get** /portfolio/subaccounts | List of Sub-Accounts
*PortfolioAnalystApi* | [**pa_performance_post**](docs/PortfolioAnalystApi.md#pa_performance_post) | **post** /pa/performance | Account Performance
*PortfolioAnalystApi* | [**pa_summary_post**](docs/PortfolioAnalystApi.md#pa_summary_post) | **post** /pa/summary | Account Balance's Summary
*PortfolioAnalystApi* | [**pa_transactions_post**](docs/PortfolioAnalystApi.md#pa_transactions_post) | **post** /pa/transactions | Position's Transaction History
*ScannerApi* | [**iserver_scanner_params_get**](docs/ScannerApi.md#iserver_scanner_params_get) | **get** /iserver/scanner/params | Scanner Parameters
*ScannerApi* | [**iserver_scanner_run_post**](docs/ScannerApi.md#iserver_scanner_run_post) | **post** /iserver/scanner/run | run scanner to get a list of contracts
*SessionApi* | [**iserver_auth_status_post**](docs/SessionApi.md#iserver_auth_status_post) | **post** /iserver/auth/status | Authentication Status
*SessionApi* | [**iserver_reauthenticate_post**](docs/SessionApi.md#iserver_reauthenticate_post) | **post** /iserver/reauthenticate | Tries to re-authenticate to Brokerage
*SessionApi* | [**logout_post**](docs/SessionApi.md#logout_post) | **post** /logout | Ends the current session
*SessionApi* | [**sso_validate_get**](docs/SessionApi.md#sso_validate_get) | **get** /sso/validate | Validate SSO
*SessionApi* | [**tickle_post**](docs/SessionApi.md#tickle_post) | **post** /tickle | Ping the server to keep the session open
*StreamingApi* | [**ws_post**](docs/StreamingApi.md#ws_post) | **post** /ws | Websocket Endpoint
*TradesApi* | [**iserver_account_trades_get**](docs/TradesApi.md#iserver_account_trades_get) | **get** /iserver/account/trades | List of Trades for the selected account


## Documentation For Models

 - [Account](docs/Account.md)
 - [AccountMaster](docs/AccountMaster.md)
 - [AlertRequest](docs/AlertRequest.md)
 - [AlertRequestConditions](docs/AlertRequestConditions.md)
 - [AlertResponse](docs/AlertResponse.md)
 - [AlertResponseConditions](docs/AlertResponseConditions.md)
 - [AuthStatus](docs/AuthStatus.md)
 - [CalendarRequest](docs/CalendarRequest.md)
 - [CalendarRequestDate](docs/CalendarRequestDate.md)
 - [CalendarRequestFilters](docs/CalendarRequestFilters.md)
 - [Contract](docs/Contract.md)
 - [ContractRules](docs/ContractRules.md)
 - [HistoryData](docs/HistoryData.md)
 - [HistoryDataData](docs/HistoryDataData.md)
 - [HistoryResult](docs/HistoryResult.md)
 - [HistoryResultBars](docs/HistoryResultBars.md)
 - [IbcustEntityInfoAddress](docs/IbcustEntityInfoAddress.md)
 - [IbcustEntityInfoEntities](docs/IbcustEntityInfoEntities.md)
 - [IbcustEntityInfoName](docs/IbcustEntityInfoName.md)
 - [InlineObject](docs/InlineObject.md)
 - [InlineObject1](docs/InlineObject1.md)
 - [InlineObject10](docs/InlineObject10.md)
 - [InlineObject11](docs/InlineObject11.md)
 - [InlineObject2](docs/InlineObject2.md)
 - [InlineObject3](docs/InlineObject3.md)
 - [InlineObject4](docs/InlineObject4.md)
 - [InlineObject5](docs/InlineObject5.md)
 - [InlineObject6](docs/InlineObject6.md)
 - [InlineObject7](docs/InlineObject7.md)
 - [InlineObject8](docs/InlineObject8.md)
 - [InlineObject9](docs/InlineObject9.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [InlineResponse2001](docs/InlineResponse2001.md)
 - [InlineResponse20010](docs/InlineResponse20010.md)
 - [InlineResponse20011](docs/InlineResponse20011.md)
 - [InlineResponse20011Orders](docs/InlineResponse20011Orders.md)
 - [InlineResponse20012](docs/InlineResponse20012.md)
 - [InlineResponse20013](docs/InlineResponse20013.md)
 - [InlineResponse20014](docs/InlineResponse20014.md)
 - [InlineResponse20014Amount](docs/InlineResponse20014Amount.md)
 - [InlineResponse20014Equity](docs/InlineResponse20014Equity.md)
 - [InlineResponse20015](docs/InlineResponse20015.md)
 - [InlineResponse20016](docs/InlineResponse20016.md)
 - [InlineResponse20017](docs/InlineResponse20017.md)
 - [InlineResponse20018](docs/InlineResponse20018.md)
 - [InlineResponse20019](docs/InlineResponse20019.md)
 - [InlineResponse2002](docs/InlineResponse2002.md)
 - [InlineResponse20020](docs/InlineResponse20020.md)
 - [InlineResponse20021](docs/InlineResponse20021.md)
 - [InlineResponse20022](docs/InlineResponse20022.md)
 - [InlineResponse20022CqtTypes](docs/InlineResponse20022CqtTypes.md)
 - [InlineResponse20022FraqTypes](docs/InlineResponse20022FraqTypes.md)
 - [InlineResponse20022IbalgoTypes](docs/InlineResponse20022IbalgoTypes.md)
 - [InlineResponse20022OrderDefaults](docs/InlineResponse20022OrderDefaults.md)
 - [InlineResponse20022OrderTypes](docs/InlineResponse20022OrderTypes.md)
 - [InlineResponse20022OrderTypesOutside](docs/InlineResponse20022OrderTypesOutside.md)
 - [InlineResponse20022Rules](docs/InlineResponse20022Rules.md)
 - [InlineResponse20022String](docs/InlineResponse20022String.md)
 - [InlineResponse20022TifTypes](docs/InlineResponse20022TifTypes.md)
 - [InlineResponse20023](docs/InlineResponse20023.md)
 - [InlineResponse20023FilterList](docs/InlineResponse20023FilterList.md)
 - [InlineResponse20023InstrumentList](docs/InlineResponse20023InstrumentList.md)
 - [InlineResponse20023LocationTree](docs/InlineResponse20023LocationTree.md)
 - [InlineResponse20023Locations](docs/InlineResponse20023Locations.md)
 - [InlineResponse20023ScanTypeList](docs/InlineResponse20023ScanTypeList.md)
 - [InlineResponse20024](docs/InlineResponse20024.md)
 - [InlineResponse20025](docs/InlineResponse20025.md)
 - [InlineResponse20026](docs/InlineResponse20026.md)
 - [InlineResponse20026Schedules](docs/InlineResponse20026Schedules.md)
 - [InlineResponse20026Sessions](docs/InlineResponse20026Sessions.md)
 - [InlineResponse20026TradingTimes](docs/InlineResponse20026TradingTimes.md)
 - [InlineResponse20027](docs/InlineResponse20027.md)
 - [InlineResponse20028](docs/InlineResponse20028.md)
 - [InlineResponse20029](docs/InlineResponse20029.md)
 - [InlineResponse2003](docs/InlineResponse2003.md)
 - [InlineResponse20030](docs/InlineResponse20030.md)
 - [InlineResponse20031](docs/InlineResponse20031.md)
 - [InlineResponse20032](docs/InlineResponse20032.md)
 - [InlineResponse20033](docs/InlineResponse20033.md)
 - [InlineResponse2004](docs/InlineResponse2004.md)
 - [InlineResponse2005](docs/InlineResponse2005.md)
 - [InlineResponse2005E](docs/InlineResponse2005E.md)
 - [InlineResponse2006](docs/InlineResponse2006.md)
 - [InlineResponse2007](docs/InlineResponse2007.md)
 - [InlineResponse2008](docs/InlineResponse2008.md)
 - [InlineResponse2009](docs/InlineResponse2009.md)
 - [InlineResponse400](docs/InlineResponse400.md)
 - [InlineResponse4001](docs/InlineResponse4001.md)
 - [InlineResponse429](docs/InlineResponse429.md)
 - [IserverSecdefSearchSections](docs/IserverSecdefSearchSections.md)
 - [Ledger](docs/Ledger.md)
 - [MarketData](docs/MarketData.md)
 - [ModifyOrder](docs/ModifyOrder.md)
 - [Order](docs/Order.md)
 - [OrderRequest](docs/OrderRequest.md)
 - [Performance](docs/Performance.md)
 - [PerformanceCps](docs/PerformanceCps.md)
 - [PerformanceCpsData](docs/PerformanceCpsData.md)
 - [PerformanceNav](docs/PerformanceNav.md)
 - [PerformanceTpps](docs/PerformanceTpps.md)
 - [ScannerParams](docs/ScannerParams.md)
 - [ScannerParamsInstrumentList](docs/ScannerParamsInstrumentList.md)
 - [ScannerParamsInstrumentListInstrument](docs/ScannerParamsInstrumentListInstrument.md)
 - [ScannerParamsLocationTree](docs/ScannerParamsLocationTree.md)
 - [ScannerParamsLocationTreeLocation](docs/ScannerParamsLocationTreeLocation.md)
 - [ScannerParamsScanTypeList](docs/ScannerParamsScanTypeList.md)
 - [ScannerParamsScanTypeListScanType](docs/ScannerParamsScanTypeListScanType.md)
 - [ScannerResult](docs/ScannerResult.md)
 - [ScannerResultContracts](docs/ScannerResultContracts.md)
 - [ScannerResultContractsContract](docs/ScannerResultContractsContract.md)
 - [SecdefInfo](docs/SecdefInfo.md)
 - [SetAccount](docs/SetAccount.md)
 - [StatsData](docs/StatsData.md)
 - [Summary](docs/Summary.md)
 - [SummaryAccountSummaries](docs/SummaryAccountSummaries.md)
 - [SummaryBalanceByDate](docs/SummaryBalanceByDate.md)
 - [SummaryBalanceByDateSeries](docs/SummaryBalanceByDateSeries.md)
 - [SummaryExcludedAccounts](docs/SummaryExcludedAccounts.md)
 - [SummaryTotal](docs/SummaryTotal.md)
 - [SystemError](docs/SystemError.md)
 - [Trade](docs/Trade.md)
 - [Transactions](docs/Transactions.md)
 - [TransactionsTransactions](docs/TransactionsTransactions.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



