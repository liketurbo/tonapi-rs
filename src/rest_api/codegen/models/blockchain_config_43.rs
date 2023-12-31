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

/// BlockchainConfig43 : The size limits and some other characteristics of accounts and messages.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainConfig43 {
    #[serde(rename = "size_limits_config")]
    pub size_limits_config: Box<crate::rest_api::codegen::models::SizeLimitsConfig>,
}

impl BlockchainConfig43 {
    /// The size limits and some other characteristics of accounts and messages.
    pub fn new(
        size_limits_config: crate::rest_api::codegen::models::SizeLimitsConfig,
    ) -> BlockchainConfig43 {
        BlockchainConfig43 {
            size_limits_config: Box::new(size_limits_config),
        }
    }
}
