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
pub struct InlineResponse20015 {
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "local_order_id", skip_serializing_if = "Option::is_none")]
    pub local_order_id: Option<String>,
    #[serde(rename = "order_status", skip_serializing_if = "Option::is_none")]
    pub order_status: Option<String>,
}

impl InlineResponse20015 {
    pub fn new() -> InlineResponse20015 {
        InlineResponse20015 {
            order_id: None,
            local_order_id: None,
            order_status: None,
        }
    }
}


