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
pub struct HistoryDataData {
    /// open price
    #[serde(rename = "o", skip_serializing_if = "Option::is_none")]
    pub o: Option<f32>,
    /// close price
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub c: Option<f32>,
    /// high price
    #[serde(rename = "h", skip_serializing_if = "Option::is_none")]
    pub h: Option<f32>,
    /// low price
    #[serde(rename = "l", skip_serializing_if = "Option::is_none")]
    pub l: Option<f32>,
    /// volume
    #[serde(rename = "v", skip_serializing_if = "Option::is_none")]
    pub v: Option<f32>,
    /// unix time stamp
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub t: Option<f32>,
}

impl HistoryDataData {
    pub fn new() -> HistoryDataData {
        HistoryDataData {
            o: None,
            c: None,
            h: None,
            l: None,
            v: None,
            t: None,
        }
    }
}


