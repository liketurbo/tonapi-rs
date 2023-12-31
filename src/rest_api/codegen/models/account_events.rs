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
pub struct AccountEvents {
    #[serde(rename = "events")]
    pub events: Vec<crate::rest_api::codegen::models::AccountEvent>,
    #[serde(rename = "next_from")]
    pub next_from: i64,
}

impl AccountEvents {
    pub fn new(
        events: Vec<crate::rest_api::codegen::models::AccountEvent>,
        next_from: i64,
    ) -> AccountEvents {
        AccountEvents { events, next_from }
    }
}
