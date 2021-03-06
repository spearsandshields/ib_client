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
pub struct InlineResponse200 {
    /// true means username is still logged in, false means it is not
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<bool>,
}

impl InlineResponse200 {
    pub fn new() -> InlineResponse200 {
        InlineResponse200 {
            confirmed: None,
        }
    }
}


