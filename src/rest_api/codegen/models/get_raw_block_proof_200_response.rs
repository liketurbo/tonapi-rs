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
pub struct GetRawBlockProof200Response {
    #[serde(rename = "complete")]
    pub complete: bool,
    #[serde(rename = "from")]
    pub from: Box<crate::rest_api::codegen::models::BlockRaw>,
    #[serde(rename = "to")]
    pub to: Box<crate::rest_api::codegen::models::BlockRaw>,
    #[serde(rename = "steps")]
    pub steps: Vec<crate::rest_api::codegen::models::GetRawBlockProof200ResponseStepsInner>,
}

impl GetRawBlockProof200Response {
    pub fn new(
        complete: bool,
        from: crate::rest_api::codegen::models::BlockRaw,
        to: crate::rest_api::codegen::models::BlockRaw,
        steps: Vec<crate::rest_api::codegen::models::GetRawBlockProof200ResponseStepsInner>,
    ) -> GetRawBlockProof200Response {
        GetRawBlockProof200Response {
            complete,
            from: Box::new(from),
            to: Box::new(to),
            steps,
        }
    }
}
