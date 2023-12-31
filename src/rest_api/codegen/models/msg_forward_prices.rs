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
pub struct MsgForwardPrices {
    #[serde(rename = "lump_price")]
    pub lump_price: i64,
    #[serde(rename = "bit_price")]
    pub bit_price: i64,
    #[serde(rename = "cell_price")]
    pub cell_price: i64,
    #[serde(rename = "ihr_price_factor")]
    pub ihr_price_factor: i64,
    #[serde(rename = "first_frac")]
    pub first_frac: i64,
    #[serde(rename = "next_frac")]
    pub next_frac: i64,
}

impl MsgForwardPrices {
    pub fn new(
        lump_price: i64,
        bit_price: i64,
        cell_price: i64,
        ihr_price_factor: i64,
        first_frac: i64,
        next_frac: i64,
    ) -> MsgForwardPrices {
        MsgForwardPrices {
            lump_price,
            bit_price,
            cell_price,
            ihr_price_factor,
            first_frac,
            next_frac,
        }
    }
}
