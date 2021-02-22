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
pub struct InlineResponse20022 {
    /// Classification of Financial Instrument codes
    #[serde(rename = "cfi_code", skip_serializing_if = "Option::is_none")]
    pub cfi_code: Option<String>,
    /// Underlying Symbol for contract
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "cusip", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    /// Expiration Date in the format YYYYMMDD
    #[serde(rename = "expiry_full", skip_serializing_if = "Option::is_none")]
    pub expiry_full: Option<f32>,
    /// IBKRs contract identifier
    #[serde(rename = "con_id", skip_serializing_if = "Option::is_none")]
    pub con_id: Option<f32>,
    /// Date on which the underlying transaction settles if the option is exercised
    #[serde(rename = "maturity_date", skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<f32>,
    /// Specific group of companies or businesses.
    #[serde(rename = "industry", skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    /// Asset Class of the contract
    #[serde(rename = "instrument_type", skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    /// Designation of the contract
    #[serde(rename = "trading_class", skip_serializing_if = "Option::is_none")]
    pub trading_class: Option<String>,
    /// Comma separated list of exchanges or trading venues
    #[serde(rename = "valid_exchanges", skip_serializing_if = "Option::is_none")]
    pub valid_exchanges: Option<String>,
    /// Allowed to sell shares that you own
    #[serde(rename = "allow_sell_long", skip_serializing_if = "Option::is_none")]
    pub allow_sell_long: Option<bool>,
    /// Supports zero commission trades
    #[serde(rename = "is_zero_commission_security", skip_serializing_if = "Option::is_none")]
    pub is_zero_commission_security: Option<bool>,
    /// Contracts symbol from primary exchange. For options it is the OCC symbol.
    #[serde(rename = "local_symbol", skip_serializing_if = "Option::is_none")]
    pub local_symbol: Option<String>,
    #[serde(rename = "classifier", skip_serializing_if = "Option::is_none")]
    pub classifier: Option<String>,
    /// Currency contract trades in
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Formatted contract parameters
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// IBKRs contract identifier for the underlying instrument
    #[serde(rename = "underlying_con_id", skip_serializing_if = "Option::is_none")]
    pub underlying_con_id: Option<f32>,
    /// Provides trading outside of Regular Trading Hours
    #[serde(rename = "r_t_h", skip_serializing_if = "Option::is_none")]
    pub r_t_h: Option<bool>,
    /// numerical value of each point of price movement
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<String>,
    /// fixed price at which the owner of the option buys or sells the underlying
    #[serde(rename = "strike", skip_serializing_if = "Option::is_none")]
    pub strike: Option<String>,
    /// Put or Call of the option
    #[serde(rename = "right", skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
    /// Legal entity for underlying contract
    #[serde(rename = "underlying_issuer", skip_serializing_if = "Option::is_none")]
    pub underlying_issuer: Option<String>,
    /// Month the contract must be satisfied by making or accepting delivery
    #[serde(rename = "contract_month", skip_serializing_if = "Option::is_none")]
    pub contract_month: Option<String>,
    /// Contracts company name
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Support IBKRs SMART routing
    #[serde(rename = "smart_available", skip_serializing_if = "Option::is_none")]
    pub smart_available: Option<bool>,
    /// Primary Exchange, Routing or Trading Venue
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<crate::models::InlineResponse20022Rules>>,
}

impl InlineResponse20022 {
    pub fn new() -> InlineResponse20022 {
        InlineResponse20022 {
            cfi_code: None,
            symbol: None,
            cusip: None,
            expiry_full: None,
            con_id: None,
            maturity_date: None,
            industry: None,
            instrument_type: None,
            trading_class: None,
            valid_exchanges: None,
            allow_sell_long: None,
            is_zero_commission_security: None,
            local_symbol: None,
            classifier: None,
            currency: None,
            text: None,
            underlying_con_id: None,
            r_t_h: None,
            multiplier: None,
            strike: None,
            right: None,
            underlying_issuer: None,
            contract_month: None,
            company_name: None,
            smart_available: None,
            exchange: None,
            rules: None,
        }
    }
}


