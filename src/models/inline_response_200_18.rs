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
pub struct InlineResponse20018 {
    /// success means market data was canceled.
    #[serde(rename = "confirmed", skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<String>,
}

impl InlineResponse20018 {
    pub fn new() -> InlineResponse20018 {
        InlineResponse20018 {
            confirmed: None,
        }
    }
}


