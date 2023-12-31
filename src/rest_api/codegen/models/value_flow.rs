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
pub struct ValueFlow {
    #[serde(rename = "account")]
    pub account: Box<crate::rest_api::codegen::models::AccountAddress>,
    #[serde(rename = "ton")]
    pub ton: i64,
    #[serde(rename = "fees")]
    pub fees: i64,
    #[serde(rename = "jettons", skip_serializing_if = "Option::is_none")]
    pub jettons: Option<Vec<crate::rest_api::codegen::models::ValueFlowJettonsInner>>,
}

impl ValueFlow {
    pub fn new(
        account: crate::rest_api::codegen::models::AccountAddress,
        ton: i64,
        fees: i64,
    ) -> ValueFlow {
        ValueFlow {
            account: Box::new(account),
            ton,
            fees,
            jettons: None,
        }
    }
}
