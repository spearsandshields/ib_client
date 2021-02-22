/*
 * Client Portal Web API
 *
 * Client Portal Web API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InlineResponse20026TradingTimes : Returns tradingTime in exchange time zone.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20026TradingTimes {
    #[serde(rename = "openingTime", skip_serializing_if = "Option::is_none")]
    pub opening_time: Option<i32>,
    #[serde(rename = "closingTime", skip_serializing_if = "Option::is_none")]
    pub closing_time: Option<i32>,
    #[serde(rename = "cancelDayOrders", skip_serializing_if = "Option::is_none")]
    pub cancel_day_orders: Option<String>,
}

impl InlineResponse20026TradingTimes {
    /// Returns tradingTime in exchange time zone.
    pub fn new() -> InlineResponse20026TradingTimes {
        InlineResponse20026TradingTimes {
            opening_time: None,
            closing_time: None,
            cancel_day_orders: None,
        }
    }
}

