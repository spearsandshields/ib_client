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
pub struct InlineResponse2005 {
    /// Email option is enabled or not 0-off, 1-on.
    #[serde(rename = "M", skip_serializing_if = "Option::is_none")]
    pub M: Option<i32>,
    #[serde(rename = "E", skip_serializing_if = "Option::is_none")]
    pub E: Option<Vec<crate::models::InlineResponse2005E>>,
}

impl InlineResponse2005 {
    pub fn new() -> InlineResponse2005 {
        InlineResponse2005 {
            M: None,
            E: None,
        }
    }
}

