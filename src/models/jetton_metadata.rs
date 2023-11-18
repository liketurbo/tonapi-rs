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
pub struct JettonMetadata {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "decimals")]
    pub decimals: String,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "social", skip_serializing_if = "Option::is_none")]
    pub social: Option<Vec<String>>,
    #[serde(rename = "websites", skip_serializing_if = "Option::is_none")]
    pub websites: Option<Vec<String>>,
    #[serde(rename = "catalogs", skip_serializing_if = "Option::is_none")]
    pub catalogs: Option<Vec<String>>,
}

impl JettonMetadata {
    pub fn new(address: String, name: String, symbol: String, decimals: String) -> JettonMetadata {
        JettonMetadata {
            address,
            name,
            symbol,
            decimals,
            image: None,
            description: None,
            social: None,
            websites: None,
            catalogs: None,
        }
    }
}

