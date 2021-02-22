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
pub struct InlineResponse20017 {
    #[serde(rename = "server_id", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i32>,
    #[serde(rename = "_updated", skip_serializing_if = "Option::is_none")]
    pub _updated: Option<i32>,
    /// Last Price
    #[serde(rename = "31", skip_serializing_if = "Option::is_none")]
    pub var_31: Option<String>,
    /// Symbol
    #[serde(rename = "55", skip_serializing_if = "Option::is_none")]
    pub var_55: Option<String>,
    /// Text
    #[serde(rename = "58", skip_serializing_if = "Option::is_none")]
    pub var_58: Option<String>,
    /// High
    #[serde(rename = "70", skip_serializing_if = "Option::is_none")]
    pub var_70: Option<String>,
    /// Low
    #[serde(rename = "71", skip_serializing_if = "Option::is_none")]
    pub var_71: Option<String>,
    /// Position
    #[serde(rename = "72", skip_serializing_if = "Option::is_none")]
    pub var_72: Option<String>,
    /// Market Value
    #[serde(rename = "73", skip_serializing_if = "Option::is_none")]
    pub var_73: Option<String>,
    /// Average Price
    #[serde(rename = "74", skip_serializing_if = "Option::is_none")]
    pub var_74: Option<String>,
    /// Unrealized PnL
    #[serde(rename = "75", skip_serializing_if = "Option::is_none")]
    pub var_75: Option<String>,
    /// Formatted position
    #[serde(rename = "76", skip_serializing_if = "Option::is_none")]
    pub var_76: Option<String>,
    /// Formatted Unrealized PnL
    #[serde(rename = "77", skip_serializing_if = "Option::is_none")]
    pub var_77: Option<String>,
    /// Daily PnL
    #[serde(rename = "78", skip_serializing_if = "Option::is_none")]
    pub var_78: Option<String>,
    /// Change Price
    #[serde(rename = "82", skip_serializing_if = "Option::is_none")]
    pub var_82: Option<String>,
    /// Change Percent
    #[serde(rename = "83", skip_serializing_if = "Option::is_none")]
    pub var_83: Option<String>,
    /// Bid Price
    #[serde(rename = "84", skip_serializing_if = "Option::is_none")]
    pub var_84: Option<String>,
    /// Ask Size
    #[serde(rename = "85", skip_serializing_if = "Option::is_none")]
    pub var_85: Option<String>,
    /// Ask Price
    #[serde(rename = "86", skip_serializing_if = "Option::is_none")]
    pub var_86: Option<String>,
    /// Volume
    #[serde(rename = "87", skip_serializing_if = "Option::is_none")]
    pub var_87: Option<String>,
    /// Bid Size
    #[serde(rename = "88", skip_serializing_if = "Option::is_none")]
    pub var_88: Option<String>,
    /// Exchange
    #[serde(rename = "6004", skip_serializing_if = "Option::is_none")]
    pub var_6004: Option<String>,
    /// Conid
    #[serde(rename = "6008", skip_serializing_if = "Option::is_none")]
    pub var_6008: Option<String>,
    /// Security Type
    #[serde(rename = "6070", skip_serializing_if = "Option::is_none")]
    pub var_6070: Option<String>,
    /// Months
    #[serde(rename = "6072", skip_serializing_if = "Option::is_none")]
    pub var_6072: Option<String>,
    /// Regular Expiry
    #[serde(rename = "6073", skip_serializing_if = "Option::is_none")]
    pub var_6073: Option<String>,
    /// Marker for market data delivery method (similar to request id)
    #[serde(rename = "6119", skip_serializing_if = "Option::is_none")]
    pub var_6119: Option<String>,
    /// Underlying Conid. Use /trsrv/secdef to get more information about the security
    #[serde(rename = "6457", skip_serializing_if = "Option::is_none")]
    pub var_6457: Option<String>,
    /// Market Data Availability. The field may contain two chars. The first char is the primary code: R = Realtime, D = Delayed, Z = Frozen, Y = Frozen Delayed. The second char is the secondary code: P = Snapshot Available, p = Consolidated. 
    #[serde(rename = "6509", skip_serializing_if = "Option::is_none")]
    pub var_6509: Option<String>,
    /// Company name
    #[serde(rename = "7051", skip_serializing_if = "Option::is_none")]
    pub var_7051: Option<String>,
    /// Last Size
    #[serde(rename = "7059", skip_serializing_if = "Option::is_none")]
    pub var_7059: Option<String>,
    /// Conid + Exchange
    #[serde(rename = "7094", skip_serializing_if = "Option::is_none")]
    pub var_7094: Option<String>,
    /// Contract Description
    #[serde(rename = "7219", skip_serializing_if = "Option::is_none")]
    pub var_7219: Option<String>,
    /// Contract Description
    #[serde(rename = "7220", skip_serializing_if = "Option::is_none")]
    pub var_7220: Option<String>,
    /// Listing Exchange
    #[serde(rename = "7221", skip_serializing_if = "Option::is_none")]
    pub var_7221: Option<String>,
    /// Industry
    #[serde(rename = "7280", skip_serializing_if = "Option::is_none")]
    pub var_7280: Option<String>,
    /// Category
    #[serde(rename = "7281", skip_serializing_if = "Option::is_none")]
    pub var_7281: Option<String>,
    /// Average Daily Volume
    #[serde(rename = "7282", skip_serializing_if = "Option::is_none")]
    pub var_7282: Option<String>,
    /// Implied volatility of the option
    #[serde(rename = "7633", skip_serializing_if = "Option::is_none")]
    pub var_7633: Option<String>,
    /// Historic Volume (30d)
    #[serde(rename = "7284", skip_serializing_if = "Option::is_none")]
    pub var_7284: Option<String>,
    /// Put/Call Ratio
    #[serde(rename = "7285", skip_serializing_if = "Option::is_none")]
    pub var_7285: Option<String>,
    /// Dividend Amount
    #[serde(rename = "7286", skip_serializing_if = "Option::is_none")]
    pub var_7286: Option<String>,
    /// Dividend Yield %
    #[serde(rename = "7287", skip_serializing_if = "Option::is_none")]
    pub var_7287: Option<String>,
    /// Ex-date of the dividend
    #[serde(rename = "7288", skip_serializing_if = "Option::is_none")]
    pub var_7288: Option<String>,
    /// Market Cap
    #[serde(rename = "7289", skip_serializing_if = "Option::is_none")]
    pub var_7289: Option<String>,
    /// P/E
    #[serde(rename = "7290", skip_serializing_if = "Option::is_none")]
    pub var_7290: Option<String>,
    /// EPS
    #[serde(rename = "7291", skip_serializing_if = "Option::is_none")]
    pub var_7291: Option<String>,
    /// Cost Basis
    #[serde(rename = "7292", skip_serializing_if = "Option::is_none")]
    pub var_7292: Option<String>,
    /// 52 Week High
    #[serde(rename = "7293", skip_serializing_if = "Option::is_none")]
    pub var_7293: Option<String>,
    /// 52 Week Low
    #[serde(rename = "7294", skip_serializing_if = "Option::is_none")]
    pub var_7294: Option<String>,
    /// Open Price
    #[serde(rename = "7295", skip_serializing_if = "Option::is_none")]
    pub var_7295: Option<String>,
    /// Close Price
    #[serde(rename = "7296", skip_serializing_if = "Option::is_none")]
    pub var_7296: Option<String>,
    /// Delta
    #[serde(rename = "7308", skip_serializing_if = "Option::is_none")]
    pub var_7308: Option<String>,
    /// Gamma
    #[serde(rename = "7309", skip_serializing_if = "Option::is_none")]
    pub var_7309: Option<String>,
    /// Theta
    #[serde(rename = "7310", skip_serializing_if = "Option::is_none")]
    pub var_7310: Option<String>,
    /// Vega
    #[serde(rename = "7311", skip_serializing_if = "Option::is_none")]
    pub var_7311: Option<String>,
}

impl InlineResponse20017 {
    pub fn new() -> InlineResponse20017 {
        InlineResponse20017 {
            server_id: None,
            conid: None,
            _updated: None,
            var_31: None,
            var_55: None,
            var_58: None,
            var_70: None,
            var_71: None,
            var_72: None,
            var_73: None,
            var_74: None,
            var_75: None,
            var_76: None,
            var_77: None,
            var_78: None,
            var_82: None,
            var_83: None,
            var_84: None,
            var_85: None,
            var_86: None,
            var_87: None,
            var_88: None,
            var_6004: None,
            var_6008: None,
            var_6070: None,
            var_6072: None,
            var_6073: None,
            var_6119: None,
            var_6457: None,
            var_6509: None,
            var_7051: None,
            var_7059: None,
            var_7094: None,
            var_7219: None,
            var_7220: None,
            var_7221: None,
            var_7280: None,
            var_7281: None,
            var_7282: None,
            var_7633: None,
            var_7284: None,
            var_7285: None,
            var_7286: None,
            var_7287: None,
            var_7288: None,
            var_7289: None,
            var_7290: None,
            var_7291: None,
            var_7292: None,
            var_7293: None,
            var_7294: None,
            var_7295: None,
            var_7296: None,
            var_7308: None,
            var_7309: None,
            var_7310: None,
            var_7311: None,
        }
    }
}


