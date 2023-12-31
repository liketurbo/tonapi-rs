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
pub struct GetAccountInfoByStateInitRequest {
    #[serde(rename = "state_init")]
    pub state_init: String,
}

impl GetAccountInfoByStateInitRequest {
    pub fn new(state_init: String) -> GetAccountInfoByStateInitRequest {
        GetAccountInfoByStateInitRequest { state_init }
    }
}
