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
pub struct InlineResponse20011 {
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<crate::models::InlineResponse20011Orders>>,
    /// If live order update is a snapshot
    #[serde(rename = "snapshot", skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

impl InlineResponse20011 {
    pub fn new() -> InlineResponse20011 {
        InlineResponse20011 {
            orders: None,
            snapshot: None,
        }
    }
}


