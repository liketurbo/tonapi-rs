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
pub struct JettonBurnAction {
    #[serde(rename = "sender")]
    pub sender: Box<crate::models::AccountAddress>,
    #[serde(rename = "senders_wallet")]
    pub senders_wallet: String,
    /// amount in quanta of tokens
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "jetton")]
    pub jetton: Box<crate::models::JettonPreview>,
}

impl JettonBurnAction {
    pub fn new(sender: crate::models::AccountAddress, senders_wallet: String, amount: String, jetton: crate::models::JettonPreview) -> JettonBurnAction {
        JettonBurnAction {
            sender: Box::new(sender),
            senders_wallet,
            amount,
            jetton: Box::new(jetton),
        }
    }
}


