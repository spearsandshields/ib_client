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
pub struct AlertRequestConditions {
    /// Types: 1-Price, 3-Time, 4-Margin, 5-Trade, 6-Volume, 7: MTA market 8: MTA Position, 9: MTA Acc. Daily PN& 
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<i32>,
    /// format, conid@exchange
    #[serde(rename = "conidex", skip_serializing_if = "Option::is_none")]
    pub conidex: Option<String>,
    /// optional, operator for the current condition, can be >= or <=
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// optional, only some type of conditions have triggerMethod
    #[serde(rename = "triggerMethod", skip_serializing_if = "Option::is_none")]
    pub trigger_method: Option<String>,
    /// can not be empty, can pass default value \"*\"
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// \"a\" means \"AND\", \"o\" means \"OR\", \"n\" means \"END\", the last one condition in the condition array should \"n\" 
    #[serde(rename = "logicBind", skip_serializing_if = "Option::is_none")]
    pub logic_bind: Option<String>,
    /// only needed for some MTA alert condition
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl AlertRequestConditions {
    pub fn new() -> AlertRequestConditions {
        AlertRequestConditions {
            _type: None,
            conidex: None,
            operator: None,
            trigger_method: None,
            value: None,
            logic_bind: None,
            time_zone: None,
        }
    }
}


