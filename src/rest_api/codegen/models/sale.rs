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
pub struct Sale {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "market")]
    pub market: Box<crate::rest_api::codegen::models::AccountAddress>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::rest_api::codegen::models::AccountAddress>>,
    #[serde(rename = "price")]
    pub price: Box<crate::rest_api::codegen::models::Price>,
}

impl Sale {
    pub fn new(
        address: String,
        market: crate::rest_api::codegen::models::AccountAddress,
        price: crate::rest_api::codegen::models::Price,
    ) -> Sale {
        Sale {
            address,
            market: Box::new(market),
            owner: None,
            price: Box::new(price),
        }
    }
}