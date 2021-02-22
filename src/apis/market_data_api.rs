/*
 * Client Portal Web API
 *
 * Client Portal Web API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `iserver_marketdata_conid_unsubscribe_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverMarketdataConidUnsubscribeGetError {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `iserver_marketdata_history_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverMarketdataHistoryGetError {
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::SystemError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `iserver_marketdata_snapshot_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverMarketdataSnapshotGetError {
    Status400(crate::models::InlineResponse4001),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `iserver_marketdata_unsubscribeall_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverMarketdataUnsubscribeallGetError {
    UnknownValue(serde_json::Value),
}


/// Cancel market data for given conid. To cancel all market data request(s), see /iserver/marketdata/unsubscribeall. 
pub async fn iserver_marketdata_conid_unsubscribe_get(configuration: &configuration::Configuration, conid: &str) -> Result<crate::models::InlineResponse20018, Error<IserverMarketdataConidUnsubscribeGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/iserver/marketdata/{conid}/unsubscribe", configuration.base_path, conid=crate::apis::urlencode(conid));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverMarketdataConidUnsubscribeGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get historical market Data for given conid, length of data is controlled by 'period' and 'bar'. Formatted as: min=minute, h=hour, d=day, w=week, m=month, y=year e.g. period =1y with bar =1w returns 52 data points (Max of 1000 data points supported). **Note**: There's a limit of 5 concurrent requests. Excessive requests will return a 'Too many requests' status 429 response. 
pub async fn iserver_marketdata_history_get(configuration: &configuration::Configuration, conid: &str, period: &str, exchange: Option<&str>, bar: Option<&str>, outside_rth: Option<bool>) -> Result<crate::models::HistoryData, Error<IserverMarketdataHistoryGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/iserver/marketdata/history", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("conid", &conid.to_string())]);
    if let Some(ref local_var_str) = exchange {
        local_var_req_builder = local_var_req_builder.query(&[("exchange", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("period", &period.to_string())]);
    if let Some(ref local_var_str) = bar {
        local_var_req_builder = local_var_req_builder.query(&[("bar", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = outside_rth {
        local_var_req_builder = local_var_req_builder.query(&[("outsideRth", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverMarketdataHistoryGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get Market Data for the given conid(s). The endpoint will return by default bid, ask, last, change, change pct, close, listing exchange. See response fields for a list of available fields that can be request via fields argument. The endpoint /iserver/accounts must be called prior to /iserver/marketdata/snapshot. For derivative contracts the endpoint /iserver/secdef/search must be called first. First /snapshot endpoint call for given conid will initiate the market data request.  To receive all available fields the /snapshot endpoint will need to be called several times. To receive streaming market data the endpoint /ws can be used. Refer to [Streaming WebSocket Data](https://interactivebrokers.github.io/cpwebapi/RealtimeSubscription.html) for details. 
pub async fn iserver_marketdata_snapshot_get(configuration: &configuration::Configuration, conids: &str, since: Option<i32>, fields: Option<&str>) -> Result<Vec<crate::models::InlineResponse20017>, Error<IserverMarketdataSnapshotGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/iserver/marketdata/snapshot", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("conids", &conids.to_string())]);
    if let Some(ref local_var_str) = since {
        local_var_req_builder = local_var_req_builder.query(&[("since", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverMarketdataSnapshotGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Cancel all market data request(s). To cancel market data for given conid, see /iserver/marketdata/{conid}/unsubscribe.  
pub async fn iserver_marketdata_unsubscribeall_get(configuration: &configuration::Configuration, ) -> Result<crate::models::InlineResponse20019, Error<IserverMarketdataUnsubscribeallGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/iserver/marketdata/unsubscribeall", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IserverMarketdataUnsubscribeallGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

