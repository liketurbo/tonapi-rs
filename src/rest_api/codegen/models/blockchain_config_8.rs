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

/// BlockchainConfig8 : The network version and additional capabilities supported by the validators.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainConfig8 {
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(rename = "capabilities")]
    pub capabilities: i64,
}

impl BlockchainConfig8 {
    /// The network version and additional capabilities supported by the validators.
    pub fn new(version: i64, capabilities: i64) -> BlockchainConfig8 {
        BlockchainConfig8 {
            version,
            capabilities,
        }
    }
}
