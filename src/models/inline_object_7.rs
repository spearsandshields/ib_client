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
pub struct InlineObject7 {
    #[serde(rename = "conids", skip_serializing_if = "Option::is_none")]
    pub conids: Option<Vec<i32>>,
}

impl InlineObject7 {
    pub fn new() -> InlineObject7 {
        InlineObject7 {
            conids: None,
        }
    }
}


