/*
 * Client Portal Web API
 *
 * Client Portal Web API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Summary : account information



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::models::SummaryTotal>,
    /// date format-- yyyy-MM-dd
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "excludedAccounts", skip_serializing_if = "Option::is_none")]
    pub excluded_accounts: Option<Vec<crate::models::SummaryExcludedAccounts>>,
    #[serde(rename = "lastSuccessfulUpdate", skip_serializing_if = "Option::is_none")]
    pub last_successful_update: Option<String>,
    #[serde(rename = "accountSummaries", skip_serializing_if = "Option::is_none")]
    pub account_summaries: Option<Vec<crate::models::SummaryAccountSummaries>>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// indicator of user having configured any external accounts
    #[serde(rename = "hasExternalAccounts", skip_serializing_if = "Option::is_none")]
    pub has_external_accounts: Option<bool>,
    #[serde(rename = "rc", skip_serializing_if = "Option::is_none")]
    pub rc: Option<i32>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "pm", skip_serializing_if = "Option::is_none")]
    pub pm: Option<String>,
    #[serde(rename = "view", skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
    #[serde(rename = "balanceByDate", skip_serializing_if = "Option::is_none")]
    pub balance_by_date: Option<crate::models::SummaryBalanceByDate>,
}

impl Summary {
    /// account information
    pub fn new() -> Summary {
        Summary {
            total: None,
            start_date: None,
            excluded_accounts: None,
            last_successful_update: None,
            account_summaries: None,
            end_date: None,
            has_external_accounts: None,
            rc: None,
            currency: None,
            user_id: None,
            pm: None,
            view: None,
            balance_by_date: None,
        }
    }
}


