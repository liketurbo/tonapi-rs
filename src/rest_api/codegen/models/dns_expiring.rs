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
pub struct DnsExpiring {
    #[serde(rename = "items")]
    pub items: Vec<crate::rest_api::codegen::models::DnsExpiringItemsInner>,
}

impl DnsExpiring {
    pub fn new(items: Vec<crate::rest_api::codegen::models::DnsExpiringItemsInner>) -> DnsExpiring {
        DnsExpiring { items }
    }
}
