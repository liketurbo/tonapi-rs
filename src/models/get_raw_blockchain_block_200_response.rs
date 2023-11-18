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
pub struct GetRawBlockchainBlock200Response {
    #[serde(rename = "id")]
    pub id: Box<crate::models::BlockRaw>,
    #[serde(rename = "data")]
    pub data: String,
}

impl GetRawBlockchainBlock200Response {
    pub fn new(id: crate::models::BlockRaw, data: String) -> GetRawBlockchainBlock200Response {
        GetRawBlockchainBlock200Response {
            id: Box::new(id),
            data,
        }
    }
}


