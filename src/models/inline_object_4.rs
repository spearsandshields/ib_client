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
pub struct InlineObject4 {
    /// Notes for bracket orders: 1. Children orders will not have its own \"cOID\", so please donot pass \"cOID\" parameter in child order.Instead, they will have a \"parentId\" which must be equal to \"cOID\" of parent. 2. When you cancel a parent order, it will cancel all bracket orders, when you cancel one child order, it will also cancel its sibling order. 
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<crate::models::OrderRequest>>,
}

impl InlineObject4 {
    pub fn new() -> InlineObject4 {
        InlineObject4 {
            orders: None,
        }
    }
}

