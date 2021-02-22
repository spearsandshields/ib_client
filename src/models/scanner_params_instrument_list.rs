/*
 * Client Portal Web API
 *
 * Client Portal Web API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScannerParamsInstrumentList : Contains list of instruments for which scanner can be ran



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScannerParamsInstrumentList {
    /// Contains information like name, supported filters, etc. for an instrument
    #[serde(rename = "Instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Vec<crate::models::ScannerParamsInstrumentListInstrument>>,
}

impl ScannerParamsInstrumentList {
    /// Contains list of instruments for which scanner can be ran
    pub fn new() -> ScannerParamsInstrumentList {
        ScannerParamsInstrumentList {
            instrument: None,
        }
    }
}

