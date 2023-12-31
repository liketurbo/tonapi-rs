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
pub struct JettonBalance {
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<crate::rest_api::codegen::models::TokenRates>>,
    #[serde(rename = "wallet_address")]
    pub wallet_address: Box<crate::rest_api::codegen::models::AccountAddress>,
    #[serde(rename = "jetton")]
    pub jetton: Box<crate::rest_api::codegen::models::JettonPreview>,
}

impl JettonBalance {
    pub fn new(
        balance: String,
        wallet_address: crate::rest_api::codegen::models::AccountAddress,
        jetton: crate::rest_api::codegen::models::JettonPreview,
    ) -> JettonBalance {
        JettonBalance {
            balance,
            price: None,
            wallet_address: Box::new(wallet_address),
            jetton: Box::new(jetton),
        }
    }
}
