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
    pub from: Box<crate::models::BlockRaw>,
    #[serde(rename = "to")]
    pub to: Box<crate::models::BlockRaw>,
    #[serde(rename = "steps")]
    pub steps: Vec<crate::models::GetRawBlockProof200ResponseStepsInner>,
}

impl GetRawBlockProof200Response {
    pub fn new(complete: bool, from: crate::models::BlockRaw, to: crate::models::BlockRaw, steps: Vec<crate::models::GetRawBlockProof200ResponseStepsInner>) -> GetRawBlockProof200Response {
        GetRawBlockProof200Response {
            complete,
            from: Box::new(from),
            to: Box::new(to),
            steps,
        }
    }
}

