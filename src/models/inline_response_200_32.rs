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
pub struct InlineResponse20032 {
    /// 1 for Live, 2 for Paper
    #[serde(rename = "LOGIN_TYPE", skip_serializing_if = "Option::is_none")]
    pub LOGIN_TYPE: Option<f32>,
    /// Username
    #[serde(rename = "USER_NAME", skip_serializing_if = "Option::is_none")]
    pub USER_NAME: Option<String>,
    /// User ID
    #[serde(rename = "USER_ID", skip_serializing_if = "Option::is_none")]
    pub USER_ID: Option<f32>,
    /// Time in milliseconds until session expires. Caller needs to call the again to re-validate session
    #[serde(rename = "expire", skip_serializing_if = "Option::is_none")]
    pub expire: Option<f32>,
    /// true if session was validated; false if not.
    #[serde(rename = "RESULT", skip_serializing_if = "Option::is_none")]
    pub RESULT: Option<bool>,
    /// Time of session validation
    #[serde(rename = "AUTH_TIME", skip_serializing_if = "Option::is_none")]
    pub AUTH_TIME: Option<f32>,
}

impl InlineResponse20032 {
    pub fn new() -> InlineResponse20032 {
        InlineResponse20032 {
            LOGIN_TYPE: None,
            USER_NAME: None,
            USER_ID: None,
            expire: None,
            RESULT: None,
            AUTH_TIME: None,
        }
    }
}

