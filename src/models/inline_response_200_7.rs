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
pub struct InlineResponse2007 {
    #[serde(rename = "set", skip_serializing_if = "Option::is_none")]
    pub set: Option<bool>,
    #[serde(rename = "acctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<String>,
}

impl InlineResponse2007 {
    pub fn new() -> InlineResponse2007 {
        InlineResponse2007 {
            set: None,
            acct_id: None,
        }
    }
}


