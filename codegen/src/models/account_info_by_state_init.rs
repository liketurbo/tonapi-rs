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
pub struct AccountInfoByStateInit {
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "address")]
    pub address: String,
}

impl AccountInfoByStateInit {
    pub fn new(public_key: String, address: String) -> AccountInfoByStateInit {
        AccountInfoByStateInit {
            public_key,
            address,
        }
    }
}

