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
pub struct InlineResponse20022FraqTypes {
    /// order types that support fractional trades
    #[serde(rename = "0", skip_serializing_if = "Option::is_none")]
    pub var_0: Option<String>,
}

impl InlineResponse20022FraqTypes {
    pub fn new() -> InlineResponse20022FraqTypes {
        InlineResponse20022FraqTypes {
            var_0: None,
        }
    }
}


