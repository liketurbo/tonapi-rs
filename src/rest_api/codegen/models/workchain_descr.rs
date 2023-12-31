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
pub struct WorkchainDescr {
    #[serde(rename = "workchain")]
    pub workchain: i32,
    #[serde(rename = "enabled_since")]
    pub enabled_since: i64,
    #[serde(rename = "actual_min_split")]
    pub actual_min_split: i32,
    #[serde(rename = "min_split")]
    pub min_split: i32,
    #[serde(rename = "max_split")]
    pub max_split: i32,
    #[serde(rename = "basic")]
    pub basic: i32,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "accept_msgs")]
    pub accept_msgs: bool,
    #[serde(rename = "flags")]
    pub flags: i32,
    #[serde(rename = "zerostate_root_hash")]
    pub zerostate_root_hash: String,
    #[serde(rename = "zerostate_file_hash")]
    pub zerostate_file_hash: String,
    #[serde(rename = "version")]
    pub version: i64,
}

impl WorkchainDescr {
    pub fn new(
        workchain: i32,
        enabled_since: i64,
        actual_min_split: i32,
        min_split: i32,
        max_split: i32,
        basic: i32,
        active: bool,
        accept_msgs: bool,
        flags: i32,
        zerostate_root_hash: String,
        zerostate_file_hash: String,
        version: i64,
    ) -> WorkchainDescr {
        WorkchainDescr {
            workchain,
            enabled_since,
            actual_min_split,
            min_split,
            max_split,
            basic,
            active,
            accept_msgs,
            flags,
            zerostate_root_hash,
            zerostate_file_hash,
            version,
        }
    }
}
