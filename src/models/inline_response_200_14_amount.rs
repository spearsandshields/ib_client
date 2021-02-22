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
pub struct InlineResponse20014Amount {
    /// for example 23,000 USD
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// for example 1.1 ... 1.2 USD
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<String>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
}

impl InlineResponse20014Amount {
    pub fn new() -> InlineResponse20014Amount {
        InlineResponse20014Amount {
            amount: None,
            commission: None,
            total: None,
        }
    }
}


