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
pub struct InlineObject8 {
    #[serde(rename = "acctIds", skip_serializing_if = "Option::is_none")]
    pub acct_ids: Option<Vec<String>>,
}

impl InlineObject8 {
    pub fn new() -> InlineObject8 {
        InlineObject8 {
            acct_ids: None,
        }
    }
}


