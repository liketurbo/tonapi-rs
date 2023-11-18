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
pub struct BlockchainConfig18StoragePricesInner {
    #[serde(rename = "utime_since")]
    pub utime_since: i64,
    #[serde(rename = "bit_price_ps")]
    pub bit_price_ps: i64,
    #[serde(rename = "cell_price_ps")]
    pub cell_price_ps: i64,
    #[serde(rename = "mc_bit_price_ps")]
    pub mc_bit_price_ps: i64,
    #[serde(rename = "mc_cell_price_ps")]
    pub mc_cell_price_ps: i64,
}

impl BlockchainConfig18StoragePricesInner {
    pub fn new(utime_since: i64, bit_price_ps: i64, cell_price_ps: i64, mc_bit_price_ps: i64, mc_cell_price_ps: i64) -> BlockchainConfig18StoragePricesInner {
        BlockchainConfig18StoragePricesInner {
            utime_since,
            bit_price_ps,
            cell_price_ps,
            mc_bit_price_ps,
            mc_cell_price_ps,
        }
    }
}

