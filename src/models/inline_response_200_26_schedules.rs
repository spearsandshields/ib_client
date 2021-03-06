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
pub struct InlineResponse20026Schedules {
    #[serde(rename = "clearingCycleEndTime", skip_serializing_if = "Option::is_none")]
    pub clearing_cycle_end_time: Option<i32>,
    /// 20000101 stands for any Sat, 20000102 stands for any Sun, ... 20000107 stands for any Fri. Any other date stands for itself.
    #[serde(rename = "tradingScheduleDate", skip_serializing_if = "Option::is_none")]
    pub trading_schedule_date: Option<i32>,
    #[serde(rename = "sessions", skip_serializing_if = "Option::is_none")]
    pub sessions: Option<crate::models::InlineResponse20026Sessions>,
    #[serde(rename = "tradingTimes", skip_serializing_if = "Option::is_none")]
    pub trading_times: Option<crate::models::InlineResponse20026TradingTimes>,
}

impl InlineResponse20026Schedules {
    pub fn new() -> InlineResponse20026Schedules {
        InlineResponse20026Schedules {
            clearing_cycle_end_time: None,
            trading_schedule_date: None,
            sessions: None,
            trading_times: None,
        }
    }
}


