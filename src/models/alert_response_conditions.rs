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
pub struct AlertResponseConditions {
    /// Types: 1-Price, 3-Time, 4-Margin, 5-Trade, 6-Volume, 7: MTA market 8: MTA Position, 9: MTA Acc. Daily PN& 
    #[serde(rename = "condition_type", skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<i32>,
    /// format, conid@exchange
    #[serde(rename = "conidex", skip_serializing_if = "Option::is_none")]
    pub conidex: Option<String>,
    #[serde(rename = "contract_description_1", skip_serializing_if = "Option::is_none")]
    pub contract_description_1: Option<String>,
    /// optional, operator for the current condition, can be >= or <=
    #[serde(rename = "condition_operator", skip_serializing_if = "Option::is_none")]
    pub condition_operator: Option<String>,
    /// optional, only some type of conditions have triggerMethod
    #[serde(rename = "condition_trigger_method", skip_serializing_if = "Option::is_none")]
    pub condition_trigger_method: Option<String>,
    /// can not be empty, can pass default value \"*\"
    #[serde(rename = "condition_value", skip_serializing_if = "Option::is_none")]
    pub condition_value: Option<String>,
    /// \"a\" means \"AND\", \"o\" means \"OR\", \"n\" means \"END\", the last one condition in the condition array should \"n\" 
    #[serde(rename = "condition_logic_bind", skip_serializing_if = "Option::is_none")]
    pub condition_logic_bind: Option<String>,
    /// only needed for some MTA alert condition
    #[serde(rename = "condition_time_zone", skip_serializing_if = "Option::is_none")]
    pub condition_time_zone: Option<String>,
}

impl AlertResponseConditions {
    pub fn new() -> AlertResponseConditions {
        AlertResponseConditions {
            condition_type: None,
            conidex: None,
            contract_description_1: None,
            condition_operator: None,
            condition_trigger_method: None,
            condition_value: None,
            condition_logic_bind: None,
            condition_time_zone: None,
        }
    }
}

