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
pub struct Action {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "TonTransfer", skip_serializing_if = "Option::is_none")]
    pub ton_transfer: Option<Box<crate::rest_api::codegen::models::TonTransferAction>>,
    #[serde(rename = "ContractDeploy", skip_serializing_if = "Option::is_none")]
    pub contract_deploy: Option<Box<crate::rest_api::codegen::models::ContractDeployAction>>,
    #[serde(rename = "JettonTransfer", skip_serializing_if = "Option::is_none")]
    pub jetton_transfer: Option<Box<crate::rest_api::codegen::models::JettonTransferAction>>,
    #[serde(rename = "JettonBurn", skip_serializing_if = "Option::is_none")]
    pub jetton_burn: Option<Box<crate::rest_api::codegen::models::JettonBurnAction>>,
    #[serde(rename = "JettonMint", skip_serializing_if = "Option::is_none")]
    pub jetton_mint: Option<Box<crate::rest_api::codegen::models::JettonMintAction>>,
    #[serde(rename = "NftItemTransfer", skip_serializing_if = "Option::is_none")]
    pub nft_item_transfer: Option<Box<crate::rest_api::codegen::models::NftItemTransferAction>>,
    #[serde(rename = "Subscribe", skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<Box<crate::rest_api::codegen::models::SubscriptionAction>>,
    #[serde(rename = "UnSubscribe", skip_serializing_if = "Option::is_none")]
    pub un_subscribe: Option<Box<crate::rest_api::codegen::models::UnSubscriptionAction>>,
    #[serde(rename = "AuctionBid", skip_serializing_if = "Option::is_none")]
    pub auction_bid: Option<Box<crate::rest_api::codegen::models::AuctionBidAction>>,
    #[serde(rename = "NftPurchase", skip_serializing_if = "Option::is_none")]
    pub nft_purchase: Option<Box<crate::rest_api::codegen::models::NftPurchaseAction>>,
    #[serde(rename = "DepositStake", skip_serializing_if = "Option::is_none")]
    pub deposit_stake: Option<Box<crate::rest_api::codegen::models::DepositStakeAction>>,
    #[serde(rename = "WithdrawStake", skip_serializing_if = "Option::is_none")]
    pub withdraw_stake: Option<Box<crate::rest_api::codegen::models::WithdrawStakeAction>>,
    #[serde(
        rename = "WithdrawStakeRequest",
        skip_serializing_if = "Option::is_none"
    )]
    pub withdraw_stake_request:
        Option<Box<crate::rest_api::codegen::models::WithdrawStakeRequestAction>>,
    #[serde(
        rename = "ElectionsDepositStake",
        skip_serializing_if = "Option::is_none"
    )]
    pub elections_deposit_stake:
        Option<Box<crate::rest_api::codegen::models::ElectionsDepositStakeAction>>,
    #[serde(
        rename = "ElectionsRecoverStake",
        skip_serializing_if = "Option::is_none"
    )]
    pub elections_recover_stake:
        Option<Box<crate::rest_api::codegen::models::ElectionsRecoverStakeAction>>,
    #[serde(rename = "JettonSwap", skip_serializing_if = "Option::is_none")]
    pub jetton_swap: Option<Box<crate::rest_api::codegen::models::JettonSwapAction>>,
    #[serde(rename = "SmartContractExec", skip_serializing_if = "Option::is_none")]
    pub smart_contract_exec: Option<Box<crate::rest_api::codegen::models::SmartContractAction>>,
    #[serde(rename = "DomainRenew", skip_serializing_if = "Option::is_none")]
    pub domain_renew: Option<Box<crate::rest_api::codegen::models::DomainRenewAction>>,
    #[serde(rename = "simple_preview")]
    pub simple_preview: Box<crate::rest_api::codegen::models::ActionSimplePreview>,
}

impl Action {
    pub fn new(
        r#type: Type,
        status: Status,
        simple_preview: crate::rest_api::codegen::models::ActionSimplePreview,
    ) -> Action {
        Action {
            r#type,
            status,
            ton_transfer: None,
            contract_deploy: None,
            jetton_transfer: None,
            jetton_burn: None,
            jetton_mint: None,
            nft_item_transfer: None,
            subscribe: None,
            un_subscribe: None,
            auction_bid: None,
            nft_purchase: None,
            deposit_stake: None,
            withdraw_stake: None,
            withdraw_stake_request: None,
            elections_deposit_stake: None,
            elections_recover_stake: None,
            jetton_swap: None,
            smart_contract_exec: None,
            domain_renew: None,
            simple_preview: Box::new(simple_preview),
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "TonTransfer")]
    TonTransfer,
    #[serde(rename = "JettonTransfer")]
    JettonTransfer,
    #[serde(rename = "JettonBurn")]
    JettonBurn,
    #[serde(rename = "JettonMint")]
    JettonMint,
    #[serde(rename = "NftItemTransfer")]
    NftItemTransfer,
    #[serde(rename = "ContractDeploy")]
    ContractDeploy,
    #[serde(rename = "Subscribe")]
    Subscribe,
    #[serde(rename = "UnSubscribe")]
    UnSubscribe,
    #[serde(rename = "AuctionBid")]
    AuctionBid,
    #[serde(rename = "NftPurchase")]
    NftPurchase,
    #[serde(rename = "DepositStake")]
    DepositStake,
    #[serde(rename = "WithdrawStake")]
    WithdrawStake,
    #[serde(rename = "WithdrawStakeRequest")]
    WithdrawStakeRequest,
    #[serde(rename = "JettonSwap")]
    JettonSwap,
    #[serde(rename = "SmartContractExec")]
    SmartContractExec,
    #[serde(rename = "ElectionsRecoverStake")]
    ElectionsRecoverStake,
    #[serde(rename = "ElectionsDepositStake")]
    ElectionsDepositStake,
    #[serde(rename = "DomainRenew")]
    DomainRenew,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl Default for Type {
    fn default() -> Type {
        Self::TonTransfer
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Ok
    }
}
