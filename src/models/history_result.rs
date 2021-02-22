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
pub struct HistoryResult {
    #[serde(rename = "bars", skip_serializing_if = "Option::is_none")]
    pub bars: Option<crate::models::HistoryResultBars>,
}

impl HistoryResult {
    pub fn new() -> HistoryResult {
        HistoryResult {
            bars: None,
        }
    }
}


