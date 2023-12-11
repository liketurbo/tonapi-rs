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

/// BlockchainConfig20 : The cost of computations in the masterchain. The complexity of any computation is estimated in gas units.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainConfig20 {
    #[serde(rename = "gas_limits_prices")]
    pub gas_limits_prices: Box<crate::rest_api::codegen::models::GasLimitPrices>,
}

impl BlockchainConfig20 {
    /// The cost of computations in the masterchain. The complexity of any computation is estimated in gas units.
    pub fn new(
        gas_limits_prices: crate::rest_api::codegen::models::GasLimitPrices,
    ) -> BlockchainConfig20 {
        BlockchainConfig20 {
            gas_limits_prices: Box::new(gas_limits_prices),
        }
    }
}