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
pub struct JettonsBalances {
    #[serde(rename = "balances")]
    pub balances: Vec<crate::rest_api::codegen::models::JettonBalance>,
}

impl JettonsBalances {
    pub fn new(balances: Vec<crate::rest_api::codegen::models::JettonBalance>) -> JettonsBalances {
        JettonsBalances { balances }
    }
}