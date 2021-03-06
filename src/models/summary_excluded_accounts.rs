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
pub struct SummaryExcludedAccounts {
    #[serde(rename = "lastUpdateAttempt", skip_serializing_if = "Option::is_none")]
    pub last_update_attempt: Option<String>,
    #[serde(rename = "fiName", skip_serializing_if = "Option::is_none")]
    pub fi_name: Option<String>,
    #[serde(rename = "acctTitle", skip_serializing_if = "Option::is_none")]
    pub acct_title: Option<String>,
    #[serde(rename = "acctNumAtFI", skip_serializing_if = "Option::is_none")]
    pub acct_num_at_fi: Option<String>,
    #[serde(rename = "acctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<String>,
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
    #[serde(rename = "harvestCode", skip_serializing_if = "Option::is_none")]
    pub harvest_code: Option<i32>,
    #[serde(rename = "lastUpdateStatusCode", skip_serializing_if = "Option::is_none")]
    pub last_update_status_code: Option<String>,
    #[serde(rename = "rc", skip_serializing_if = "Option::is_none")]
    pub rc: Option<i32>,
}

impl SummaryExcludedAccounts {
    pub fn new() -> SummaryExcludedAccounts {
        SummaryExcludedAccounts {
            last_update_attempt: None,
            fi_name: None,
            acct_title: None,
            acct_num_at_fi: None,
            acct_id: None,
            last_update: None,
            harvest_code: None,
            last_update_status_code: None,
            rc: None,
        }
    }
}


