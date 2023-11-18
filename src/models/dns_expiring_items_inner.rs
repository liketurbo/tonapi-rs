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
pub struct DnsExpiringItemsInner {
    #[serde(rename = "expiring_at")]
    pub expiring_at: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "dns_item", skip_serializing_if = "Option::is_none")]
    pub dns_item: Option<Box<crate::models::NftItem>>,
}

impl DnsExpiringItemsInner {
    pub fn new(expiring_at: i64, name: String) -> DnsExpiringItemsInner {
        DnsExpiringItemsInner {
            expiring_at,
            name,
            dns_item: None,
        }
    }
}


