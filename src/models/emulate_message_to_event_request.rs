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
pub struct EmulateMessageToEventRequest {
    #[serde(rename = "boc")]
    pub boc: String,
}

impl EmulateMessageToEventRequest {
    pub fn new(boc: String) -> EmulateMessageToEventRequest {
        EmulateMessageToEventRequest {
            boc,
        }
    }
}

