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
pub struct InlineResponse2004 {
    #[serde(rename = "T", skip_serializing_if = "Option::is_none")]
    pub T: Option<i32>,
    #[serde(rename = "V", skip_serializing_if = "Option::is_none")]
    pub V: Option<i32>,
}

impl InlineResponse2004 {
    pub fn new() -> InlineResponse2004 {
        InlineResponse2004 {
            T: None,
            V: None,
        }
    }
}

