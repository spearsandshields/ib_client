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
pub struct InlineResponse20031 {
    #[serde(rename = "ACCTID", skip_serializing_if = "Option::is_none")]
    pub ACCTID: Option<Vec<serde_json::Value>>,
}

impl InlineResponse20031 {
    pub fn new() -> InlineResponse20031 {
        InlineResponse20031 {
            ACCTID: None,
        }
    }
}


