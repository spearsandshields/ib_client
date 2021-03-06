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
pub struct InlineResponse20014 {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<crate::models::InlineResponse20014Amount>,
    #[serde(rename = "equity", skip_serializing_if = "Option::is_none")]
    pub equity: Option<crate::models::InlineResponse20014Equity>,
    #[serde(rename = "initial", skip_serializing_if = "Option::is_none")]
    pub initial: Option<crate::models::InlineResponse20014Equity>,
    #[serde(rename = "maintenance", skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<crate::models::InlineResponse20014Equity>,
    #[serde(rename = "warn", skip_serializing_if = "Option::is_none")]
    pub warn: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl InlineResponse20014 {
    pub fn new() -> InlineResponse20014 {
        InlineResponse20014 {
            amount: None,
            equity: None,
            initial: None,
            maintenance: None,
            warn: None,
            error: None,
        }
    }
}


