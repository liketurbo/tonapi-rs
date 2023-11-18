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
pub struct NftItems {
    #[serde(rename = "nft_items")]
    pub nft_items: Vec<crate::models::NftItem>,
}

impl NftItems {
    pub fn new(nft_items: Vec<crate::models::NftItem>) -> NftItems {
        NftItems {
            nft_items,
        }
    }
}


