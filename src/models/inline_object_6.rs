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
pub struct InlineObject6 {
    /// symbol or name to be searched
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// should be true if the search is to be performed by name. false by default.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<bool>,
    /// If search is done by name, only the assets provided in this field will be returned. Currently, only STK is supported.
    #[serde(rename = "secType", skip_serializing_if = "Option::is_none")]
    pub sec_type: Option<String>,
}

impl InlineObject6 {
    pub fn new(symbol: String) -> InlineObject6 {
        InlineObject6 {
            symbol,
            name: None,
            sec_type: None,
        }
    }
}


