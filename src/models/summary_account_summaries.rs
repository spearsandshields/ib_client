/*
 * Client Portal Web API
 *
 * Client Portal Web API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SummaryAccountSummaries {
    #[serde(rename = "chg", skip_serializing_if = "Option::is_none")]
    pub chg: Option<String>,
    #[serde(rename = "hasAccounts", skip_serializing_if = "Option::is_none")]
    pub has_accounts: Option<String>,
    #[serde(rename = "accountTypeName", skip_serializing_if = "Option::is_none")]
    pub account_type_name: Option<String>,
    #[serde(rename = "rtn", skip_serializing_if = "Option::is_none")]
    pub rtn: Option<String>,
    #[serde(rename = "endVal", skip_serializing_if = "Option::is_none")]
    pub end_val: Option<String>,
    #[serde(rename = "accountTypeCode", skip_serializing_if = "Option::is_none")]
    pub account_type_code: Option<String>,
    #[serde(rename = "startVal", skip_serializing_if = "Option::is_none")]
    pub start_val: Option<String>,
}

impl SummaryAccountSummaries {
    pub fn new() -> SummaryAccountSummaries {
        SummaryAccountSummaries {
            chg: None,
            has_accounts: None,
            account_type_name: None,
            rtn: None,
            end_val: None,
            account_type_code: None,
            start_val: None,
        }
    }
}


