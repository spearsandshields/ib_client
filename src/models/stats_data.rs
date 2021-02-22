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
pub struct StatsData {
    #[serde(rename = "Conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<f32>,
    #[serde(rename = "Exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(rename = "V", skip_serializing_if = "Option::is_none")]
    pub V: Option<f32>,
    #[serde(rename = "T", skip_serializing_if = "Option::is_none")]
    pub T: Option<f32>,
    #[serde(rename = "TT", skip_serializing_if = "Option::is_none")]
    pub TT: Option<f32>,
    /// Object, payload depends on event type. See confluence page for IGEvntUpd.
    #[serde(rename = "P", skip_serializing_if = "Option::is_none")]
    pub P: Option<String>,
}

impl StatsData {
    pub fn new() -> StatsData {
        StatsData {
            conid: None,
            exchange: None,
            V: None,
            T: None,
            TT: None,
            P: None,
        }
    }
}

