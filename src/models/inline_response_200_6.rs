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
pub struct InlineResponse2006 {
    /// Unique account id
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    /// Account Id and its alias
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<serde_json::Value>,
    #[serde(rename = "selectedAccount", skip_serializing_if = "Option::is_none")]
    pub selected_account: Option<String>,
}

impl InlineResponse2006 {
    pub fn new() -> InlineResponse2006 {
        InlineResponse2006 {
            accounts: None,
            aliases: None,
            selected_account: None,
        }
    }
}


