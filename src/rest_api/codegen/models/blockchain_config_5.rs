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
pub struct BlockchainConfig5 {
    #[serde(rename = "blackhole_addr", skip_serializing_if = "Option::is_none")]
    pub blackhole_addr: Option<String>,
    #[serde(rename = "fee_burn_nom")]
    pub fee_burn_nom: i64,
    #[serde(rename = "fee_burn_denom")]
    pub fee_burn_denom: i64,
}

impl BlockchainConfig5 {
    pub fn new(fee_burn_nom: i64, fee_burn_denom: i64) -> BlockchainConfig5 {
        BlockchainConfig5 {
            blackhole_addr: None,
            fee_burn_nom,
            fee_burn_denom,
        }
    }
}
