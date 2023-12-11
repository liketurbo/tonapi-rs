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
pub struct JettonQuantity {
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "wallet_address")]
    pub wallet_address: Box<crate::rest_api::codegen::models::AccountAddress>,
    #[serde(rename = "jetton")]
    pub jetton: Box<crate::rest_api::codegen::models::JettonPreview>,
}

impl JettonQuantity {
    pub fn new(
        quantity: String,
        wallet_address: crate::rest_api::codegen::models::AccountAddress,
        jetton: crate::rest_api::codegen::models::JettonPreview,
    ) -> JettonQuantity {
        JettonQuantity {
            quantity,
            wallet_address: Box::new(wallet_address),
            jetton: Box::new(jetton),
        }
    }
}