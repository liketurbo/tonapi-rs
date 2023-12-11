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
pub struct GetRawShardBlockProof200Response {
    #[serde(rename = "masterchain_id")]
    pub masterchain_id: Box<crate::rest_api::codegen::models::BlockRaw>,
    #[serde(rename = "links")]
    pub links: Vec<crate::rest_api::codegen::models::GetRawShardBlockProof200ResponseLinksInner>,
}

impl GetRawShardBlockProof200Response {
    pub fn new(
        masterchain_id: crate::rest_api::codegen::models::BlockRaw,
        links: Vec<crate::rest_api::codegen::models::GetRawShardBlockProof200ResponseLinksInner>,
    ) -> GetRawShardBlockProof200Response {
        GetRawShardBlockProof200Response {
            masterchain_id: Box::new(masterchain_id),
            links,
        }
    }
}