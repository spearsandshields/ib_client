/*
 * Client Portal Web API
 *
 * Client Portal Web API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Transactions : account transactions



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transactions {
    /// will always be getTransactions
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// same as request
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Indicates whether current day and realtime data is included in the result
    #[serde(rename = "includesRealTime", skip_serializing_if = "Option::is_none")]
    pub includes_real_time: Option<bool>,
    /// Period start date. Epoch time, GMT
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<f32>,
    /// Period end date. Epoch time, GMT
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<f32>,
    /// Sorted by date descending
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<crate::models::TransactionsTransactions>>,
}

impl Transactions {
    /// account transactions
    pub fn new() -> Transactions {
        Transactions {
            id: None,
            currency: None,
            includes_real_time: None,
            from: None,
            to: None,
            transactions: None,
        }
    }
}


