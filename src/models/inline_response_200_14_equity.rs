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
pub struct InlineResponse20014Equity {
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
    #[serde(rename = "change", skip_serializing_if = "Option::is_none")]
    pub change: Option<String>,
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl InlineResponse20014Equity {
    pub fn new() -> InlineResponse20014Equity {
        InlineResponse20014Equity {
            current: None,
            change: None,
            after: None,
        }
    }
}

