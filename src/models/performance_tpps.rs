/*
 * Client Portal Web API
 *
 * Client Portal Web API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PerformanceTpps : Time period performance data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceTpps {
    /// array of dates, the length should be same as the length of returns inside data.
    #[serde(rename = "dates", skip_serializing_if = "Option::is_none")]
    pub dates: Option<Vec<String>>,
    /// M means Month
    #[serde(rename = "freq", skip_serializing_if = "Option::is_none")]
    pub freq: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::PerformanceCpsData>>,
}

impl PerformanceTpps {
    /// Time period performance data
    pub fn new() -> PerformanceTpps {
        PerformanceTpps {
            dates: None,
            freq: None,
            data: None,
        }
    }
}


