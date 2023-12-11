use serde::{Deserialize, Serialize};

/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::rest_api::codegen::apis::ResponseContent;

/// struct for passing parameters to the method [`get_account_seqno`]
#[derive(Clone, Debug)]
pub struct GetAccountSeqnoParams {
    /// account ID
    pub account_id: String,
}

/// struct for passing parameters to the method [`get_wallet_backup`]
#[derive(Clone, Debug)]
pub struct GetWalletBackupParams {
    pub x_ton_connect_auth: String,
}

/// struct for passing parameters to the method [`get_wallets_by_public_key`]
#[derive(Clone, Debug)]
pub struct GetWalletsByPublicKeyParams {
    pub public_key: String,
}

/// struct for passing parameters to the method [`set_wallet_backup`]
#[derive(Clone, Debug)]
pub struct SetWalletBackupParams {
    pub x_ton_connect_auth: String,
    /// Information for saving backup
    pub body: std::path::PathBuf,
}

/// struct for passing parameters to the method [`ton_connect_proof`]
#[derive(Clone, Debug)]
pub struct TonConnectProofParams {
    /// Data that is expected from TON Connect
    pub ton_connect_proof_request: crate::rest_api::codegen::models::TonConnectProofRequest,
}

/// struct for typed errors of method [`get_account_seqno`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountSeqnoError {
    DefaultResponse(crate::rest_api::codegen::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_wallet_backup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWalletBackupError {
    DefaultResponse(crate::rest_api::codegen::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_wallets_by_public_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWalletsByPublicKeyError {
    DefaultResponse(crate::rest_api::codegen::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_wallet_backup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetWalletBackupError {
    DefaultResponse(crate::rest_api::codegen::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ton_connect_proof`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TonConnectProofError {
    DefaultResponse(crate::rest_api::codegen::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// Get account seqno
pub async fn get_account_seqno(
    configuration: &configuration::Configuration,
    params: GetAccountSeqnoParams,
) -> Result<crate::rest_api::codegen::models::Seqno, Error<GetAccountSeqnoError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/wallet/{account_id}/seqno",
        local_var_configuration.base_path,
        account_id = crate::rest_api::codegen::apis::urlencode(account_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAccountSeqnoError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get backup info
pub async fn get_wallet_backup(
    configuration: &configuration::Configuration,
    params: GetWalletBackupParams,
) -> Result<crate::rest_api::codegen::models::GetWalletBackup200Response, Error<GetWalletBackupError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let x_ton_connect_auth = params.x_ton_connect_auth;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/wallet/backup", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("X-TonConnect-Auth", x_ton_connect_auth.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWalletBackupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get wallets by public key
pub async fn get_wallets_by_public_key(
    configuration: &configuration::Configuration,
    params: GetWalletsByPublicKeyParams,
) -> Result<crate::rest_api::codegen::models::Accounts, Error<GetWalletsByPublicKeyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let public_key = params.public_key;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/pubkeys/{public_key}/wallets",
        local_var_configuration.base_path,
        public_key = crate::rest_api::codegen::apis::urlencode(public_key)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetWalletsByPublicKeyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Set backup info
pub async fn set_wallet_backup(
    configuration: &configuration::Configuration,
    params: SetWalletBackupParams,
) -> Result<(), Error<SetWalletBackupError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let x_ton_connect_auth = params.x_ton_connect_auth;
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/wallet/backup", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("X-TonConnect-Auth", x_ton_connect_auth.to_string());
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<SetWalletBackupError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Account verification and token issuance
pub async fn ton_connect_proof(
    configuration: &configuration::Configuration,
    params: TonConnectProofParams,
) -> Result<crate::rest_api::codegen::models::TonConnectProof200Response, Error<TonConnectProofError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let ton_connect_proof_request = params.ton_connect_proof_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/wallet/auth/proof", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&ton_connect_proof_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TonConnectProofError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}