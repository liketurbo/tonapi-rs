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
pub struct JettonBridgePrices {
    #[serde(rename = "bridge_burn_fee")]
    pub bridge_burn_fee: i64,
    #[serde(rename = "bridge_mint_fee")]
    pub bridge_mint_fee: i64,
    #[serde(rename = "wallet_min_tons_for_storage")]
    pub wallet_min_tons_for_storage: i64,
    #[serde(rename = "wallet_gas_consumption")]
    pub wallet_gas_consumption: i64,
    #[serde(rename = "minter_min_tons_for_storage")]
    pub minter_min_tons_for_storage: i64,
    #[serde(rename = "discover_gas_consumption")]
    pub discover_gas_consumption: i64,
}

impl JettonBridgePrices {
    pub fn new(bridge_burn_fee: i64, bridge_mint_fee: i64, wallet_min_tons_for_storage: i64, wallet_gas_consumption: i64, minter_min_tons_for_storage: i64, discover_gas_consumption: i64) -> JettonBridgePrices {
        JettonBridgePrices {
            bridge_burn_fee,
            bridge_mint_fee,
            wallet_min_tons_for_storage,
            wallet_gas_consumption,
            minter_min_tons_for_storage,
            discover_gas_consumption,
        }
    }
}


