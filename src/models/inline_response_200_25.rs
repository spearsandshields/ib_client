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
pub struct InlineResponse20025 {
    #[serde(rename = "acctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<serde_json::Value>,
}

impl InlineResponse20025 {
    pub fn new() -> InlineResponse20025 {
        InlineResponse20025 {
            acct_id: None,
        }
    }
}


