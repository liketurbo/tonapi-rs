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

/// BlockchainConfig22 : The limits on the block in the masterchain, upon reaching which the block is finalized and the callback of the remaining messages (if any) is carried over to the next block.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainConfig22 {
    #[serde(rename = "block_limits")]
    pub block_limits: Box<crate::rest_api::codegen::models::BlockLimits>,
}

impl BlockchainConfig22 {
    /// The limits on the block in the masterchain, upon reaching which the block is finalized and the callback of the remaining messages (if any) is carried over to the next block.
    pub fn new(block_limits: crate::rest_api::codegen::models::BlockLimits) -> BlockchainConfig22 {
        BlockchainConfig22 {
            block_limits: Box::new(block_limits),
        }
    }
}
