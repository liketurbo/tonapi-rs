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
pub struct Accounts {
    #[serde(rename = "accounts")]
    pub accounts: Vec<crate::rest_api::codegen::models::Account>,
}

impl Accounts {
    pub fn new(accounts: Vec<crate::rest_api::codegen::models::Account>) -> Accounts {
        Accounts { accounts }
    }
}
