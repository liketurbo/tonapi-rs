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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetRawMasterchainInfo200Response {
    #[serde(rename = "last")]
    pub last: Box<crate::rest_api::codegen::models::BlockRaw>,
    #[serde(rename = "state_root_hash")]
    pub state_root_hash: String,
    #[serde(rename = "init")]
    pub init: Box<crate::rest_api::codegen::models::InitStateRaw>,
}

impl GetRawMasterchainInfo200Response {
    pub fn new(
        last: crate::rest_api::codegen::models::BlockRaw,
        state_root_hash: String,
        init: crate::rest_api::codegen::models::InitStateRaw,
    ) -> GetRawMasterchainInfo200Response {
        GetRawMasterchainInfo200Response {
            last: Box::new(last),
            state_root_hash,
            init: Box::new(init),
        }
    }
}