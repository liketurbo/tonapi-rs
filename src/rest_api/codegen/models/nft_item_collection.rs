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
pub struct NftItemCollection {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
}

impl NftItemCollection {
    pub fn new(address: String, name: String, description: String) -> NftItemCollection {
        NftItemCollection {
            address,
            name,
            description,
        }
    }
}