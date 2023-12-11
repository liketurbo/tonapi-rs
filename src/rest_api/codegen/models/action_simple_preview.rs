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

/// ActionSimplePreview : shortly describes what this action is about.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionSimplePreview {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    /// a link to an image for this particular action.
    #[serde(rename = "action_image", skip_serializing_if = "Option::is_none")]
    pub action_image: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// a link to an image that depicts this action's asset.
    #[serde(rename = "value_image", skip_serializing_if = "Option::is_none")]
    pub value_image: Option<String>,
    #[serde(rename = "accounts")]
    pub accounts: Vec<crate::rest_api::codegen::models::AccountAddress>,
}

impl ActionSimplePreview {
    /// shortly describes what this action is about.
    pub fn new(
        name: String,
        description: String,
        accounts: Vec<crate::rest_api::codegen::models::AccountAddress>,
    ) -> ActionSimplePreview {
        ActionSimplePreview {
            name,
            description,
            action_image: None,
            value: None,
            value_image: None,
            accounts,
        }
    }
}