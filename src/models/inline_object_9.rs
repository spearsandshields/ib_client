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
pub struct InlineObject9 {
    #[serde(rename = "acctIds", skip_serializing_if = "Option::is_none")]
    pub acct_ids: Option<Vec<String>>,
    /// Frequency of cumulative performance data points: 'D'aily, 'M'onthly,'Q'uarterly. 
    #[serde(rename = "freq", skip_serializing_if = "Option::is_none")]
    pub freq: Option<Freq>,
}

impl InlineObject9 {
    pub fn new() -> InlineObject9 {
        InlineObject9 {
            acct_ids: None,
            freq: None,
        }
    }
}

/// Frequency of cumulative performance data points: 'D'aily, 'M'onthly,'Q'uarterly. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Freq {
    #[serde(rename = "D")]
    D,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "Q")]
    Q,
}

