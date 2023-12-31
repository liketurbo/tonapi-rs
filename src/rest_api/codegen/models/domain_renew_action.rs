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
pub struct DomainRenewAction {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "contract_address")]
    pub contract_address: String,
    #[serde(rename = "renewer")]
    pub renewer: Box<crate::rest_api::codegen::models::AccountAddress>,
}

impl DomainRenewAction {
    pub fn new(
        domain: String,
        contract_address: String,
        renewer: crate::rest_api::codegen::models::AccountAddress,
    ) -> DomainRenewAction {
        DomainRenewAction {
            domain,
            contract_address,
            renewer: Box::new(renewer),
        }
    }
}
