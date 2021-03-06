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
pub struct InlineResponse20028 {
    /// This is an array of object(s), there could be multiple results under same symbol 
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Vec<serde_json::Value>>,
}

impl InlineResponse20028 {
    pub fn new() -> InlineResponse20028 {
        InlineResponse20028 {
            symbol: None,
        }
    }
}


