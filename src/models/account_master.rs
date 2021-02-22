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
pub struct AccountMaster {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "officialTitle", skip_serializing_if = "Option::is_none")]
    pub official_title: Option<String>,
}

impl AccountMaster {
    pub fn new() -> AccountMaster {
        AccountMaster {
            title: None,
            official_title: None,
        }
    }
}


