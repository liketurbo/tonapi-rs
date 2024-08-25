use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success {
        #[serde(flatten)]
        result: T,
    },
    Error {
        error: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub address: String,
    pub balance: i64,
    pub currencies_balance: Option<HashMap<String, f64>>,
    pub last_activity: i64,
    pub status: AccountStatus,
    pub interfaces: Option<Vec<String>>,
    pub name: Option<String>,
    pub is_scam: Option<bool>,
    pub icon: Option<String>,
    pub memo_required: Option<bool>,
    pub get_methods: Vec<String>,
    pub is_suspended: Option<bool>,
    pub is_wallet: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountStatus {
    Nonexist,
    Uninit,
    Active,
    Frozen,
}

#[derive(Debug, Deserialize)]
pub struct DomainNames {
    pub domains: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct JettonBalances {
    pub balances: Vec<JettonBalance>,
}

#[derive(Debug, Deserialize)]
pub struct JettonBalance {
    pub balance: String,
    pub price: Option<TokenRates>,
    pub wallet_address: AccountAddress,
    pub jetton: JettonPreview,
    pub extensions: Option<Vec<String>>,
    pub lock: Option<Lock>,
}

#[derive(Debug, Deserialize)]
pub struct TokenRates {
    pub prices: Option<HashMap<String, f64>>,
    pub diff_24h: Option<HashMap<String, String>>,
    pub diff_7d: Option<HashMap<String, String>>,
    pub diff_30d: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct AccountAddress {
    pub address: String,
    pub name: Option<String>,
    pub is_scam: bool,
    pub icon: Option<String>,
    pub is_wallet: bool,
}

#[derive(Debug, Deserialize)]
pub struct JettonPreview {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub image: String,
    pub verification: JettonVerificationType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JettonVerificationType {
    Whitelist,
    Blacklist,
    None,
}

#[derive(Debug, Deserialize)]
pub struct Lock {
    pub amount: String,
    pub till: i64, // Unix timestamp
}

#[derive(Debug, Deserialize)]
pub struct AccountEvents {
    pub events: Vec<AccountEvent>,
    pub next_from: i64,
}

#[derive(Debug, Deserialize)]
pub struct AccountEvent {
    pub event_id: String,
    pub timestamp: i64,
    pub actions: Vec<Action>,
    pub account: AccountAddress,
    pub is_scam: bool,
    pub lt: i64,
    pub in_progress: bool,
    pub extra: i64,
}

#[derive(Debug, Deserialize)]
pub struct Action {
    pub r#type: ActionType,
    pub status: ActionStatus,
    pub simple_preview: ActionSimplePreview,
    pub base_transactions: Vec<String>,
    pub ton_transfer: Option<TonTransferAction>,
    pub contract_deploy: Option<ContractDeployAction>,
    pub jetton_transfer: Option<JettonTransferAction>,
    pub jetton_burn: Option<JettonBurnAction>,
    pub jetton_mint: Option<JettonMintAction>,
    pub nft_item_transfer: Option<NftItemTransferAction>,
    pub subscribe: Option<SubscriptionAction>,
    pub unsubscribe: Option<UnSubscriptionAction>,
    pub auction_bid: Option<AuctionBidAction>,
    pub nft_purchase: Option<NftPurchaseAction>,
    pub deposit_stake: Option<DepositStakeAction>,
    pub withdraw_stake: Option<WithdrawStakeAction>,
    pub withdraw_stake_request: Option<WithdrawStakeRequestAction>,
    pub elections_deposit_stake: Option<ElectionsDepositStakeAction>,
    pub elections_recover_stake: Option<ElectionsRecoverStakeAction>,
    pub jetton_swap: Option<JettonSwapAction>,
    pub smart_contract_exec: Option<SmartContractAction>,
    pub domain_renew: Option<DomainRenewAction>,
    pub inscription_transfer: Option<InscriptionTransferAction>,
    pub inscription_mint: Option<InscriptionMintAction>,
}

#[derive(Debug, Deserialize)]
pub enum ActionType {
    TonTransfer,
    JettonTransfer,
    JettonBurn,
    JettonMint,
    NftItemTransfer,
    ContractDeploy,
    Subscribe,
    UnSubscribe,
    AuctionBid,
    NftPurchase,
    DepositStake,
    WithdrawStake,
    WithdrawStakeRequest,
    JettonSwap,
    SmartContractExec,
    ElectionsRecoverStake,
    ElectionsDepositStake,
    DomainRenew,
    InscriptionTransfer,
    InscriptionMint,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionStatus {
    Ok,
    Failed,
}

#[derive(Debug, Deserialize)]
pub struct ActionSimplePreview {
    pub name: String,
    pub description: String,
    pub action_image: Option<String>,
    pub value: Option<String>,
    pub value_image: Option<String>,
    pub accounts: Vec<AccountAddress>,
}

#[derive(Debug, Deserialize)]
pub struct TonTransferAction {
    pub sender: AccountAddress,
    pub recipient: AccountAddress,
    pub amount: i64,
    pub comment: Option<String>,
    pub encrypted_comment: Option<EncryptedComment>,
    pub refund: Option<Refund>,
}

#[derive(Debug, Deserialize)]
pub struct EncryptedComment {
    pub encryption_type: String,
    pub cipher_text: String,
}

#[derive(Debug, Deserialize)]
pub struct Refund {
    pub r#type: RefundType,
    pub origin: String,
}

#[derive(Debug, Deserialize)]
pub enum RefundType {
    #[serde(rename = "DNS.ton")]
    DnsTon,
    #[serde(rename = "DNS.tg")]
    DnsTg,
    #[serde(rename = "GetGems")]
    GetGems,
}

#[derive(Debug, Deserialize)]
pub struct ContractDeployAction {
    pub address: String,
    pub interfaces: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct JettonTransferAction {
    pub sender: AccountAddress,
    pub recipient: AccountAddress,
    pub senders_wallet: String,
    pub recipients_wallet: String,
    pub amount: String,
    pub comment: Option<String>,
    pub encrypted_comment: Option<EncryptedComment>,
    pub refund: Option<Refund>,
    pub jetton: JettonPreview,
}

#[derive(Debug, Deserialize)]
pub struct JettonBurnAction {
    pub sender: AccountAddress,
    pub senders_wallet: String,
    pub amount: String,
    pub jetton: JettonPreview,
}

#[derive(Debug, Deserialize)]
pub struct JettonMintAction {
    pub recipient: AccountAddress,
    pub recipients_wallet: String,
    pub amount: String,
    pub jetton: JettonPreview,
}

#[derive(Debug, Deserialize)]
pub struct NftItemTransferAction {
    pub sender: Option<AccountAddress>,
    pub recipient: Option<AccountAddress>,
    pub nft: String,
    pub comment: Option<String>,
    pub encrypted_comment: Option<EncryptedComment>,
    pub payload: Option<String>,
    pub refund: Option<Refund>,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionAction {
    pub subscriber: AccountAddress,
    pub subscription: String,
    pub beneficiary: AccountAddress,
    pub amount: i64,
    pub initial: bool,
}

#[derive(Debug, Deserialize)]
pub struct UnSubscriptionAction {
    pub subscriber: AccountAddress,
    pub subscription: String,
    pub beneficiary: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct AuctionBidAction {
    pub auction_type: AuctionType,
    pub amount: Price,
    pub nft: NftItem,
    pub bidder: AccountAddress,
    pub auction: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub enum AuctionType {
    #[serde(rename = "DNS.ton")]
    DnsTon,
    #[serde(rename = "DNS.tg")]
    DnsTg,
    #[serde(rename = "NUMBER.tg")]
    NumberTg,
    #[serde(rename = "getgems")]
    Getgems,
}

#[derive(Debug, Deserialize)]
pub struct Price {
    pub value: String,
    pub token_name: String,
}

#[derive(Debug, Deserialize)]
pub struct NftItem {
    pub address: String,
    pub index: i64,
    pub owner: Option<AccountAddress>,
    pub collection: Option<Collection>,
    pub verified: bool,
    pub metadata: HashMap<String, serde_json::Value>,
    pub sale: Option<Sale>,
    pub previews: Option<Vec<ImagePreview>>,
    pub dns: Option<String>,
    pub approved_by: Option<NftApprovedBy>,
    pub include_cnft: Option<bool>,
    pub trust: Option<TrustType>,
}

#[derive(Debug, Deserialize)]
pub struct Collection {
    pub address: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Sale {
    pub address: String,
    pub market: AccountAddress,
    pub owner: Option<AccountAddress>,
    pub price: Price,
}

#[derive(Debug, Deserialize)]
pub struct ImagePreview {
    pub resolution: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub enum NftApprovalSource {
    #[serde(rename = "getgems")]
    Getgems,
    #[serde(rename = "tonkeeper")]
    Tonkeeper,
    #[serde(rename = "ton.diamonds")]
    TonDiamonds,
}

#[derive(Debug, Deserialize)]
pub struct NftApprovedBy(pub Vec<NftApprovalSource>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrustType {
    Whitelist,
    Graylist,
    Blacklist,
    None,
}

#[derive(Debug, Deserialize)]
pub struct NftPurchaseAction {
    pub auction_type: AuctionType,
    pub amount: Price,
    pub nft: NftItem,
    pub seller: AccountAddress,
    pub buyer: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct DepositStakeAction {
    pub amount: i64,
    pub staker: AccountAddress,
    pub pool: AccountAddress,
    pub implementation: PoolImplementationType,
}

#[derive(Debug, Deserialize)]
pub enum PoolImplementationType {
    #[serde(rename = "whales")]
    Whales,
    #[serde(rename = "tf")]
    Tf,
    #[serde(rename = "liquidTF")]
    LiquidTF,
}

#[derive(Debug, Deserialize)]
pub struct WithdrawStakeAction {
    pub amount: i64,
    pub staker: AccountAddress,
    pub pool: AccountAddress,
    pub implementation: PoolImplementationType,
}

#[derive(Debug, Deserialize)]
pub struct WithdrawStakeRequestAction {
    pub amount: Option<i64>,
    pub staker: AccountAddress,
    pub pool: AccountAddress,
    pub implementation: PoolImplementationType,
}

#[derive(Debug, Deserialize)]
pub struct ElectionsDepositStakeAction {
    pub amount: i64,
    pub staker: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct ElectionsRecoverStakeAction {
    pub amount: i64,
    pub staker: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct JettonSwapAction {
    pub dex: DexType,
    pub amount_in: String,
    pub amount_out: String,
    pub ton_in: Option<i64>,
    pub ton_out: Option<i64>,
    pub user_wallet: AccountAddress,
    pub router: AccountAddress,
    pub jetton_master_in: Option<JettonPreview>,
    pub jetton_master_out: Option<JettonPreview>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DexType {
    Stonfi,
    Dedust,
    Megatonfi,
}

#[derive(Debug, Deserialize)]
pub struct SmartContractAction {
    pub executor: AccountAddress,
    pub contract: AccountAddress,
    pub ton_attached: i64, // Amount in nanotons
    pub operation: String,
    pub payload: Option<String>,
    pub refund: Option<Refund>,
}

#[derive(Debug, Deserialize)]
pub struct DomainRenewAction {
    pub domain: String,
    pub contract_address: String, // Address in string format
    pub renewer: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct InscriptionTransferAction {
    pub sender: AccountAddress,
    pub recipient: AccountAddress,
    pub amount: String, // Amount is in string format as per the example
    pub comment: Option<String>,
    #[serde(rename = "type")]
    pub transfer_type: String, // The type field is renamed to transfer_type to avoid keyword conflict
    pub ticker: String,
    pub decimals: i32,
}

#[derive(Debug, Deserialize)]
pub struct InscriptionMintAction {
    #[serde(rename = "type")]
    pub mint_type: String, // The type field is renamed to mint_type to avoid keyword conflict
    pub ticker: String,
    pub recipient: AccountAddress,
    pub amount: String, // Amount is in string format as per the example
    pub decimals: i32,
}

#[derive(Debug, Deserialize)]
pub struct NftItems {
    pub nft_items: Vec<NftItem>,
}

#[derive(Debug, Deserialize)]
pub struct TraceIds {
    pub traces: Vec<TraceId>,
}

#[derive(Debug, Deserialize)]
pub struct TraceId {
    pub id: String,
    pub utime: i64,
}

#[derive(Debug, Deserialize)]
pub struct Subscriptions {
    pub subscriptions: Vec<Subscription>,
}

#[derive(Debug, Deserialize)]
pub struct Subscription {
    pub address: String,
    pub wallet_address: String,
    pub beneficiary_address: String,
    pub amount: i64,
    pub period: i64,
    pub start_time: i64,
    pub timeout: i64,
    pub last_payment_time: i64,
    pub last_request_time: i64,
    pub subscription_id: i64,
    pub failed_attempts: i32,
}

#[derive(Debug, Deserialize)]
pub struct FoundAccounts {
    pub addresses: Vec<FoundAccountAddress>,
}

#[derive(Debug, Deserialize)]
pub struct FoundAccountAddress {
    pub address: String,
    pub name: String,
    pub preview: String,
}

#[derive(Debug, Deserialize)]
pub struct DnsExpiring {
    pub items: Vec<DnsExpiringItem>,
}

#[derive(Debug, Deserialize)]
pub struct DnsExpiringItem {
    pub expiring_at: i64,
    pub name: String,
    pub dns_item: NftItem,
}

#[derive(Debug, Deserialize)]
pub struct PublicKey {
    pub public_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Multisigs {
    pub multisigs: Vec<Multisig>,
}

#[derive(Debug, Deserialize)]
pub struct Multisig {
    pub address: String,
    pub seqno: i64,
    pub threshold: i32,
    pub signers: Vec<String>,
    pub proposers: Vec<String>,
    pub orders: Vec<MultisigOrder>,
}

#[derive(Debug, Deserialize)]
pub struct MultisigOrder {
    pub address: String,
    pub order_seqno: i64,
    pub threshold: i32,
    pub sent_for_execution: bool,
    pub signers: Vec<String>,
    pub approvals_num: i32,
    pub expiration_date: i64,
    pub risk: Risk,
}

#[derive(Debug, Deserialize)]
pub struct Risk {
    pub transfer_all_remaining_balance: bool,
    pub ton: i64,
    pub jettons: Vec<JettonQuantity>,
    pub nfts: Vec<NftItem>,
}

#[derive(Debug, Deserialize)]
pub struct JettonQuantity {
    pub quantity: String,
    pub wallet_address: AccountAddress,
    pub jetton: JettonPreview,
}

#[derive(Debug, Deserialize)]
pub struct BalanceChange {
    pub balance_change: i64,
}

#[derive(Debug, Deserialize)]
pub struct NftCollections {
    pub nft_collections: Vec<NftCollection>,
}

#[derive(Debug, Deserialize)]
pub struct NftCollection {
    pub address: String,
    pub next_item_index: i64,
    pub owner: Option<AccountAddress>,
    pub raw_collection_content: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub previews: Option<Vec<ImagePreview>>,
    pub approved_by: NftApprovedBy,
}

#[derive(Debug, Deserialize)]
pub struct Jettons {
    pub jettons: Vec<JettonInfo>,
}

#[derive(Debug, Deserialize)]
pub struct JettonInfo {
    pub mintable: bool,
    pub total_supply: String,
    pub admin: Option<AccountAddress>,
    pub metadata: JettonMetadata,
    pub verification: JettonVerificationType,
    pub holders_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct JettonMetadata {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: String,
    pub image: Option<String>,
    pub description: Option<String>,
    pub social: Option<Vec<String>>,
    pub websites: Option<Vec<String>>,
    pub catalogs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct JettonHolders {
    pub addresses: Vec<JettonHolderAddress>,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct JettonHolderAddress {
    pub address: String,
    pub owner: AccountAddress,
    pub balance: String,
}

#[derive(Debug, Deserialize)]
pub struct JettonTransferPayload {
    pub custom_payload: Option<String>,
    pub state_init: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub event_id: String,
    pub timestamp: i64,
    pub actions: Vec<Action>,
    pub value_flow: Vec<ValueFlow>,
    pub is_scam: bool,
    pub lt: i64,
    pub in_progress: bool,
}

#[derive(Debug, Deserialize)]
pub struct ValueFlow {
    pub account: AccountAddress,
    pub ton: i64,
    pub fees: i64,
    pub jettons: Option<Vec<JettonFlow>>,
}

#[derive(Debug, Deserialize)]
pub struct JettonFlow {
    pub account: AccountAddress,
    pub jetton: JettonPreview,
    pub quantity: i64,
}

#[derive(Debug, Deserialize)]
pub struct DomainInfo {
    pub name: String,
    pub expiring_at: Option<i64>,
    pub item: Option<NftItem>,
}

#[derive(Debug, Deserialize)]
pub struct DnsRecord {
    pub wallet: Option<WalletDns>,
    pub next_resolver: Option<String>,
    pub sites: Vec<String>,
    pub storage: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WalletDns {
    pub address: String,
    pub account: AccountAddress,
    pub is_wallet: bool,
    pub has_method_pubkey: bool,
    pub has_method_seqno: bool,
    pub names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DomainBids {
    pub data: Vec<DomainBid>,
}

#[derive(Debug, Deserialize)]
pub struct DomainBid {
    pub success: bool,
    pub value: i64,
    #[serde(rename = "txTime")]
    pub tx_time: i64,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub bidder: AccountAddress,
}

#[derive(Debug, Deserialize)]
pub struct Auctions {
    pub data: Vec<Auction>,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct Auction {
    pub domain: String,
    pub owner: String,
    pub price: i64,
    pub bids: i64,
    pub date: i64,
}

#[derive(Debug, Deserialize)]
pub struct Dump {
    pub dump: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthToken {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountSeqno {
    pub seqno: i64,
}

#[derive(Debug, Deserialize)]
pub struct MessageConsequences {
    pub trace: Trace,
    pub risk: Risk,
    pub event: AccountEvent,
}

#[derive(Debug, Deserialize)]
pub struct Trace {
    pub transaction: Transaction,
    pub interfaces: Vec<String>,
    pub children: Option<Vec<Trace>>,
    pub emulated: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub lt: i64,
    pub account: AccountAddress,
    pub success: bool,
    pub utime: i64,
    pub orig_status: AccountStatus,
    pub end_status: AccountStatus,
    pub total_fees: i64,
    pub end_balance: i64,
    pub transaction_type: TransactionType,
    pub state_update_old: String,
    pub state_update_new: String,
    pub in_msg: Option<Message>,
    pub out_msgs: Vec<Message>,
    pub block: String,
    pub prev_trans_hash: Option<String>,
    pub prev_trans_lt: Option<i64>,
    pub compute_phase: Option<ComputePhase>,
    pub storage_phase: Option<StoragePhase>,
    pub credit_phase: Option<CreditPhase>,
    pub action_phase: Option<ActionPhase>,
    pub bounce_phase: Option<BouncePhaseType>,
    pub aborted: bool,
    pub destroyed: bool,
    pub raw: String,
}

#[derive(Debug, Deserialize)]
pub enum TransactionType {
    TransOrd,
    TransTickTock,
    TransSplitPrepare,
    TransSplitInstall,
    TransMergePrepare,
    TransMergeInstall,
    TransStorage,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub msg_type: MsgType,
    pub created_lt: i64,
    pub ihr_disabled: bool,
    pub bounce: bool,
    pub bounced: bool,
    pub value: i64,
    pub fwd_fee: i64,
    pub ihr_fee: i64,
    pub destination: Option<AccountAddress>,
    pub source: Option<AccountAddress>,
    pub import_fee: i64,
    pub created_at: i64,
    pub op_code: Option<String>,
    pub init: Option<StateInit>,
    pub hash: String,
    pub raw_body: Option<String>,
    pub decoded_op_name: Option<String>,
    pub decoded_body: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MsgType {
    IntMsg,
    ExtInMsg,
    ExtOutMsg,
}

#[derive(Debug, Deserialize)]
pub struct StateInit {
    pub boc: String,
    pub interfaces: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ComputePhase {
    pub skipped: bool,
    pub skip_reason: Option<ComputeSkipReason>,
    pub success: Option<bool>,
    pub gas_fees: Option<i64>,
    pub gas_used: Option<i64>,
    pub vm_steps: Option<i32>,
    pub exit_code: Option<i32>,
    pub exit_code_description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputeSkipReason {
    CskipNoState,
    CskipBadState,
    CskipNoGas,
}

#[derive(Debug, Deserialize)]
pub struct StoragePhase {
    pub fees_collected: i64,
    pub fees_due: Option<i64>,
    pub status_change: AccStatusChange,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccStatusChange {
    AcstUnchanged,
    AcstFrozen,
    AcstDeleted,
}

#[derive(Debug, Deserialize)]
pub struct CreditPhase {
    pub fees_collected: i64,
    pub credit: i64,
}

#[derive(Debug, Deserialize)]
pub struct ActionPhase {
    pub success: bool,
    pub result_code: i32,
    pub total_actions: i32,
    pub skipped_actions: i32,
    pub fwd_fees: i64,
    pub total_fees: i64,
    pub result_code_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum BouncePhaseType {
    #[serde(rename = "TrPhaseBounceNegfunds")]
    TrPhaseBounceNegFunds,
    #[serde(rename = "TrPhaseBounceNofunds")]
    TrPhaseBounceNoFunds,
    TrPhaseBounceOk,
}

#[derive(Debug, Deserialize)]
pub struct Rates {
    pub rates: TokenRates,
}

#[derive(Debug, Deserialize)]
pub struct RateChart {
    pub points: Vec<Point>,
}

#[derive(Debug, Deserialize)]
pub struct Point(
    pub u64, // timestamp
    pub f64, // value
);

#[derive(Debug, Deserialize)]
pub struct MarketRates {
    pub markets: Vec<MarketTonRates>,
}

#[derive(Debug, Deserialize)]
pub struct MarketTonRates {
    pub market: String,
    pub usd_price: f64,
    pub last_date_update: i64,
}

#[derive(Debug, Deserialize)]
pub struct AccountStaking {
    pub pools: Vec<AccountStakingInfo>,
}

#[derive(Debug, Deserialize)]
pub struct AccountStakingInfo {
    pub pool: String,
    pub amount: i64,
    pub pending_deposit: i64,
    pub pending_withdraw: i64,
    pub ready_withdraw: i64,
}

#[derive(Debug, Deserialize)]
pub struct StakingPoolInfo {
    pub implementation: PoolImplementation,
    pub pool: PoolInfo,
}

#[derive(Debug, Deserialize)]
pub struct PoolImplementation {
    pub name: String,
    pub description: String,
    pub url: String,
    pub socials: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PoolInfo {
    pub address: String,
    pub name: String,
    pub total_amount: i64,
    pub implementation: PoolImplementationType,
    pub apy: f64,
    pub min_stake: i64,
    pub cycle_start: i64,
    pub cycle_end: i64,
    pub verified: bool,
    pub current_nominators: i32,
    pub max_nominators: i32,
    pub liquid_jetton_master: Option<String>,
    pub nominators_stake: i64,
    pub validator_stake: i64,
    pub cycle_length: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct StakingPoolHistory {
    pub apy: ApyHistory,
}

#[derive(Debug, Deserialize)]
pub struct ApyHistory {
    pub apy: f64,
    pub time: i64,
}

#[derive(Debug, Deserialize)]
pub struct StakingPools {
    pub implementations: HashMap<String, PoolImplementation>,
    pub pools: Vec<PoolInfo>,
}

#[derive(Debug, Deserialize)]
pub struct StorageProviders {
    pub providers: Vec<StorageProvider>,
}

#[derive(Debug, Deserialize)]
pub struct StorageProvider {
    pub address: String,
    pub accept_new_contracts: bool,
    pub rate_per_mb_day: i64,
    pub max_span: i64,
    pub minimal_file_size: i64,
    pub maximal_file_size: i64,
}

#[derive(Debug, Deserialize)]
pub struct TonConnectPayload {
    pub payload: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountInfo {
    pub public_key: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct GaslessConfig {
    pub relay_address: String,
    pub gas_jettons: Vec<GasJetton>,
}

#[derive(Debug, Deserialize)]
pub struct GasJetton {
    pub master_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SignRawParams {
    pub relay_address: String,
    pub commission: String,
    pub from: String,
    pub valid_until: i64,
    pub messages: Vec<SignRawMessage>,
}

#[derive(Debug, Deserialize)]
pub struct SignRawMessage {
    pub address: String,
    pub amount: String,
    pub payload: Option<String>,
    pub state_init: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReducedBlocks {
    pub blocks: Vec<ReducedBlock>,
}

#[derive(Debug, Deserialize)]
pub struct ReducedBlock {
    pub workchain_id: i32,
    pub shard: String,
    pub seqno: i32,
    pub master_ref: Option<String>,
    pub tx_quantity: i32,
    pub utime: i64,
    pub shards_blocks: Vec<String>,
    pub parent: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BlockchainBlock {
    pub workchain_id: i32,
    pub shard: String,
    pub seqno: i32,
    pub root_hash: String,
    pub file_hash: String,
    pub global_id: i32,
    pub value_flow: BlockValueFlow,
    pub version: i32,
    pub after_merge: bool,
    pub before_split: bool,
    pub after_split: bool,
    pub want_split: bool,
    pub want_merge: bool,
    pub key_block: bool,
    pub gen_utime: i64,
    pub start_lt: i64,
    pub end_lt: i64,
    pub vert_seqno: i32,
    pub gen_catchain_seqno: i32,
    pub min_ref_mc_seqno: i32,
    pub prev_key_block_seqno: i32,
    pub gen_software_version: Option<i32>,
    pub gen_software_capabilities: Option<i64>,
    pub master_ref: Option<String>,
    pub prev_refs: Option<Vec<String>>,
    pub in_msg_descr_length: i64,
    pub out_msg_descr_length: i64,
    pub rand_seed: String,
    pub created_by: String,
    pub tx_quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct BlockValueFlow {
    pub from_prev_blk: BlockCurrencyCollection,
    pub to_next_blk: BlockCurrencyCollection,
    pub imported: BlockCurrencyCollection,
    pub exported: BlockCurrencyCollection,
    pub fees_collected: BlockCurrencyCollection,
    pub burned: Option<BlockCurrencyCollection>,
    pub fees_imported: BlockCurrencyCollection,
    pub recovered: BlockCurrencyCollection,
    pub created: BlockCurrencyCollection,
    pub minted: BlockCurrencyCollection,
}

#[derive(Debug, Deserialize)]
pub struct BlockCurrencyCollection {
    pub grams: i64,
    pub other: Vec<OtherCurrency>,
}

#[derive(Debug, Deserialize)]
pub struct OtherCurrency {
    pub id: i64,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockchainBlockShards {
    pub shards: Vec<Shard>,
}

#[derive(Debug, Deserialize)]
pub struct Shard {
    pub last_known_block_id: String,
    pub last_known_block: BlockchainBlock,
}

#[derive(Debug, Deserialize)]
pub struct BlockchainBlocks {
    pub blocks: Vec<BlockchainBlock>,
}

#[derive(Debug, Deserialize)]
pub struct Transactions {
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Deserialize)]
pub struct BlockchainConfig {
    pub raw: String,
    #[serde(rename = "0")]
    pub config_address: String,
    #[serde(rename = "1")]
    pub elector_address: String,
    #[serde(rename = "2")]
    pub minter_address: String,
    #[serde(rename = "3")]
    pub fee_collector_address: Option<String>,
    #[serde(rename = "4")]
    pub dns_root_address: String,
    #[serde(rename = "5")]
    pub fee_burn_config: Option<FeeBurnConfig>,
    #[serde(rename = "6")]
    pub minting_fees: Option<MintingFees>,
    #[serde(rename = "7")]
    pub currency_volumes: Option<CurrencyVolumes>,
    #[serde(rename = "8")]
    pub network_version: Option<NetworkVersion>,
    #[serde(rename = "9")]
    pub mandatory_params: Option<MandatoryParams>,
    #[serde(rename = "10")]
    pub critical_params: Option<CriticalParams>,
    #[serde(rename = "11")]
    pub proposal_setup: Option<ConfigProposal>,
    #[serde(rename = "12")]
    pub workchains: Option<WorkchainsWrapper>,
    #[serde(rename = "13")]
    pub complaint_fees: Option<ComplaintFees>,
    #[serde(rename = "14")]
    pub block_creation_rewards: Option<BlockCreationRewards>,
    #[serde(rename = "15")]
    pub validator_election: Option<ValidatorElection>,
    #[serde(rename = "16")]
    pub validator_limits: Option<ValidatorLimits>,
    #[serde(rename = "17")]
    pub stake_params: Option<StakeParams>,
    #[serde(rename = "18")]
    pub storage_prices: Option<StoragePrices>,
    #[serde(rename = "20")]
    pub gas_limits_masterchain: Option<GasLimitPricesWrapper>,
    #[serde(rename = "21")]
    pub gas_limits_basechain: Option<GasLimitPricesWrapper>,
    #[serde(rename = "22")]
    pub block_limits_masterchain: Option<BlockLimitsWrapper>,
    #[serde(rename = "23")]
    pub block_limits_basechain: Option<BlockLimitsWrapper>,
    #[serde(rename = "24")]
    pub msg_forward_prices_masterchain: Option<MsgForwardPricesWrapper>,
    #[serde(rename = "25")]
    pub msg_forward_prices_basechain: Option<MsgForwardPricesWrapper>,
    #[serde(rename = "28")]
    pub catchain_config: Option<CatchainConfig>,
    #[serde(rename = "29")]
    pub consensus_config: Option<ConsensusConfig>,
    #[serde(rename = "31")]
    pub fundamental_smc_addr: Option<FundamentalSmcAddr>,
    #[serde(rename = "32")]
    pub validators_set_32: Option<ValidatorsSet>,
    #[serde(rename = "33")]
    pub validators_set_33: Option<ValidatorsSet>,
    #[serde(rename = "34")]
    pub validators_set_34: Option<ValidatorsSet>,
    #[serde(rename = "35")]
    pub validators_set_35: Option<ValidatorsSet>,
    #[serde(rename = "36")]
    pub validators_set_36: Option<ValidatorsSet>,
    #[serde(rename = "37")]
    pub validators_set_37: Option<ValidatorsSet>,
    #[serde(rename = "40")]
    pub misbehaviour_punishment: Option<MisbehaviourPunishmentConfig>,
    #[serde(rename = "43")]
    pub size_limits_config: Option<SizeLimitsConfig>,
    #[serde(rename = "44")]
    pub suspended_accounts: SuspendedAccounts,
    #[serde(rename = "71")]
    pub oracle_bridge_params_71: Option<OracleBridgeParamsWrapper>,
    #[serde(rename = "72")]
    pub oracle_bridge_params_72: Option<OracleBridgeParamsWrapper>,
    #[serde(rename = "73")]
    pub oracle_bridge_params_73: Option<OracleBridgeParamsWrapper>,
    #[serde(rename = "79")]
    pub jetton_bridge_params_79: Option<JettonBridgeParamsWrapper>,
    #[serde(rename = "81")]
    pub jetton_bridge_params_81: Option<JettonBridgeParamsWrapper>,
    #[serde(rename = "82")]
    pub jetton_bridge_params_82: Option<JettonBridgeParamsWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct FeeBurnConfig {
    pub blackhole_addr: Option<String>,
    pub fee_burn_nom: i64,
    pub fee_burn_denom: i64,
}

#[derive(Debug, Deserialize)]
pub struct MintingFees {
    pub mint_new_price: i64,
    pub mint_add_price: i64,
}

#[derive(Debug, Deserialize)]
pub struct CurrencyVolumes {
    pub currencies: Vec<Currency>,
}

#[derive(Debug, Deserialize)]
pub struct Currency {
    pub currency_id: i64,
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct NetworkVersion {
    pub version: i64,
    pub capabilities: i64,
}

#[derive(Debug, Deserialize)]
pub struct MandatoryParams {
    pub mandatory_params: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CriticalParams {
    pub critical_params: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigProposal {
    pub normal_params: ConfigProposalSetup,
    pub critical_params: ConfigProposalSetup,
}

#[derive(Debug, Deserialize)]
pub struct ConfigProposalSetup {
    pub min_tot_rounds: i32,
    pub max_tot_rounds: i32,
    pub min_wins: i32,
    pub max_losses: i32,
    pub min_store_sec: i64,
    pub max_store_sec: i64,
    pub bit_price: i64,
    pub cell_price: i64,
}

#[derive(Debug, Deserialize)]
pub struct WorkchainsWrapper {
    pub workchains: Vec<WorkchainDescr>,
}

#[derive(Debug, Deserialize)]
pub struct WorkchainDescr {
    pub workchain: i32,
    pub enabled_since: i64,
    pub actual_min_split: i32,
    pub min_split: i32,
    pub max_split: i32,
    pub basic: i32,
    pub active: bool,
    pub accept_msgs: bool,
    pub flags: i32,
    pub zerostate_root_hash: String,
    pub zerostate_file_hash: String,
    pub version: i64,
}

#[derive(Debug, Deserialize)]
pub struct ComplaintFees {
    pub deposit: i64,
    pub bit_price: i64,
    pub cell_price: i64,
}

#[derive(Debug, Deserialize)]
pub struct BlockCreationRewards {
    pub masterchain_block_fee: i64,
    pub basechain_block_fee: i64,
}

#[derive(Debug, Deserialize)]
pub struct ValidatorElection {
    pub validators_elected_for: i64,
    pub elections_start_before: i64,
    pub elections_end_before: i64,
    pub stake_held_for: i64,
}

#[derive(Debug, Deserialize)]
pub struct ValidatorLimits {
    pub max_validators: i32,
    pub max_main_validators: i32,
    pub min_validators: i32,
}

#[derive(Debug, Deserialize)]
pub struct StakeParams {
    pub min_stake: String,
    pub max_stake: String,
    pub min_total_stake: String,
    pub max_stake_factor: i64,
}

#[derive(Debug, Deserialize)]
pub struct StoragePrices {
    pub storage_prices: Vec<StoragePrice>,
}

#[derive(Debug, Deserialize)]
pub struct StoragePrice {
    pub utime_since: i64,
    pub bit_price_ps: i64,
    pub cell_price_ps: i64,
    pub mc_bit_price_ps: i64,
    pub mc_cell_price_ps: i64,
}

#[derive(Debug, Deserialize)]
pub struct GasLimitPricesWrapper {
    pub gas_limits_prices: GasLimitPrices,
}

#[derive(Debug, Deserialize)]
pub struct GasLimitPrices {
    pub gas_price: i64,
    pub gas_limit: i64,
    pub gas_credit: i64,
    pub block_gas_limit: i64,
    pub freeze_due_limit: i64,
    pub delete_due_limit: i64,
    pub special_gas_limit: Option<i64>,
    pub flat_gas_limit: Option<i64>,
    pub flat_gas_price: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BlockParamLimits {
    pub underload: i64,
    pub soft_limit: i64,
    pub hard_limit: i64,
}

#[derive(Debug, Deserialize)]
pub struct BlockLimitsWrapper {
    pub block_limits: BlockLimits,
}

#[derive(Debug, Deserialize)]
pub struct BlockLimits {
    pub bytes: BlockParamLimits,
    pub gas: BlockParamLimits,
    pub lt_delta: BlockParamLimits,
}

#[derive(Debug, Deserialize)]
pub struct MsgForwardPricesWrapper {
    pub msg_forward_prices: MsgForwardPrices,
}

#[derive(Debug, Deserialize)]
pub struct MsgForwardPrices {
    pub lump_price: i64,
    pub bit_price: i64,
    pub cell_price: i64,
    pub ihr_price_factor: i64,
    pub first_frac: i64,
    pub next_frac: i64,
}

#[derive(Debug, Deserialize)]
pub struct CatchainConfig {
    pub mc_catchain_lifetime: i64,
    pub shard_catchain_lifetime: i64,
    pub shard_validators_lifetime: i64,
    pub shard_validators_num: i64,
    pub flags: Option<i32>,
    pub shuffle_mc_validators: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ConsensusConfig {
    pub round_candidates: i64,
    pub next_candidate_delay_ms: i64,
    pub consensus_timeout_ms: i64,
    pub fast_attempts: i64,
    pub attempt_duration: i64,
    pub catchain_max_deps: i64,
    pub max_block_bytes: i64,
    pub max_collated_bytes: i64,
    pub proto_version: i64,
    pub catchain_max_blocks_coeff: i64,
    pub flags: Option<i32>,
    pub new_catchain_ids: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FundamentalSmcAddr {
    pub fundamental_smc_addr: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ValidatorsSet {
    pub utime_since: i64,
    pub utime_until: i64,
    pub total: i32,
    pub main: i32,
    pub total_weight: String,
    pub list: Vec<ValidatorInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ValidatorInfo {
    pub public_key: String,
    pub weight: i64,
    pub adnl_addr: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MisbehaviourPunishmentConfig {
    pub default_flat_fine: i64,
    pub default_proportional_fine: i64,
    pub severity_flat_mult: i64,
    pub severity_proportional_mult: i64,
    pub unpunishable_interval: i64,
    pub long_interval: i64,
    pub long_flat_mult: i64,
    pub long_proportional_mult: i64,
    pub medium_interval: i64,
    pub medium_flat_mult: i64,
    pub medium_proportional_mult: i64,
}

#[derive(Debug, Deserialize)]
pub struct SizeLimitsConfig {
    pub max_msg_bits: i64,
    pub max_msg_cells: i64,
    pub max_library_cells: i64,
    pub max_vm_data_depth: i32,
    pub max_ext_msg_size: i64,
    pub max_ext_msg_depth: i32,
    pub max_acc_state_cells: Option<i64>,
    pub max_acc_state_bits: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SuspendedAccounts {
    pub accounts: Vec<String>,
    pub suspended_until: i64,
}

#[derive(Debug, Deserialize)]
pub struct OracleBridgeParamsWrapper {
    pub oracle_bridge_params: OracleBridgeParams,
}

#[derive(Debug, Deserialize)]
pub struct OracleBridgeParams {
    pub bridge_addr: String,
    pub oracle_multisig_address: String,
    pub external_chain_address: String,
    pub oracles: Vec<Oracle>,
}

#[derive(Debug, Deserialize)]
pub struct Oracle {
    pub address: String,
    pub secp_pubkey: String,
}

#[derive(Debug, Deserialize)]
pub struct JettonBridgeParamsWrapper {
    pub jetton_bridge_params: JettonBridgeParams,
}

#[derive(Debug, Deserialize)]
pub struct JettonBridgeParams {
    pub bridge_address: String,
    pub oracles_address: String,
    pub state_flags: i64,
    pub burn_bridge_fee: Option<i64>,
    pub oracles: Vec<Oracle>,
    pub external_chain_address: Option<String>,
    pub prices: Option<JettonBridgePrices>,
}

#[derive(Debug, Deserialize)]
pub struct JettonBridgePrices {
    pub bridge_burn_fee: i64,
    pub bridge_mint_fee: i64,
    pub wallet_min_tons_for_storage: i64,
    pub wallet_gas_consumption: i64,
    pub minter_min_tons_for_storage: i64,
    pub discover_gas_consumption: i64,
}

#[derive(Debug, Deserialize)]
pub struct RawBlockchainConfig {
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct Validators {
    pub elect_at: i64,
    pub elect_close: i64,
    pub min_stake: i64,
    pub total_stake: i64,
    pub validators: Vec<Validator>,
}

#[derive(Debug, Deserialize)]
pub struct Validator {
    pub address: String,
    pub adnl_address: String,
    pub stake: i64,
    pub max_factor: i64,
}

#[derive(Debug, Deserialize)]
pub struct BlockchainRawAccount {
    pub address: String,
    pub balance: i64,
    pub extra_balance: Option<HashMap<String, String>>,
    pub code: Option<String>,
    pub data: Option<String>,
    pub last_transaction_lt: i64,
    pub last_transaction_hash: Option<String>,
    pub frozen_hash: Option<String>,
    pub status: AccountStatus,
    pub storage: AccountStorageInfo,
    pub libraries: Option<Vec<AccountLibrary>>,
}

#[derive(Debug, Deserialize)]
pub struct AccountStorageInfo {
    pub used_cells: i64,
    pub used_bits: i64,
    pub used_public_cells: i64,
    pub last_paid: i64,
    pub due_payment: i64,
}

#[derive(Debug, Deserialize)]
pub struct AccountLibrary {
    pub public: bool,
    pub root: String,
}

#[derive(Debug, Deserialize)]
pub struct MethodExecutionResult {
    pub success: bool,
    pub exit_code: i32,
    pub stack: Vec<TvmStackRecord>,
    pub decoded: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum TvmStackRecord {
    #[serde(rename = "cell")]
    Cell {
        #[serde(skip_serializing_if = "Option::is_none")]
        cell: Option<String>,
    },
    #[serde(rename = "slice")]
    Slice {
        #[serde(skip_serializing_if = "Option::is_none")]
        slice: Option<String>,
    },
    #[serde(rename = "num")]
    Num {
        #[serde(skip_serializing_if = "Option::is_none")]
        num: Option<String>,
    },
    #[serde(rename = "tuple")]
    Tuple {
        #[serde(skip_serializing_if = "Option::is_none")]
        tuple: Option<Vec<TvmStackRecord>>,
    },
    #[serde(rename = "nan")]
    Nan,
    #[serde(rename = "null")]
    Null,
}

#[derive(Debug, Deserialize)]
pub struct BlockchainAccountInspect {
    pub code: String,
    pub code_hash: String,
    pub methods: Vec<Method>,
    pub compiler: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Method {
    pub id: i64,
    pub method: String,
}

#[derive(Debug, Deserialize)]
pub struct RawMasterchainInfo {
    pub last: BlockRaw,
    pub state_root_hash: String,
    pub init: InitStateRaw,
}

#[derive(Debug, Deserialize)]
pub struct BlockRaw {
    pub workchain: i32,
    pub shard: String,
    pub seqno: i32,
    pub root_hash: String,
    pub file_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct InitStateRaw {
    pub workchain: i32,
    pub root_hash: String,
    pub file_hash: String,
}

#[derive(Deserialize, Debug)]
pub struct RawMasterchainInfoExt {
    pub mode: i32,
    pub version: i32,
    pub capabilities: i64,
    pub last: BlockRaw,
    pub last_utime: i32,
    pub now: i32,
    pub state_root_hash: String,
    pub init: InitStateRaw,
}

#[derive(Deserialize, Debug)]
pub struct RawTime {
    pub time: i32,
}

#[derive(Deserialize, Debug)]
pub struct RawBlockchainBlock {
    pub id: BlockRaw,
    pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct RawBlockchainBlockState {
    pub id: BlockRaw,
    pub root_hash: String,
    pub file_hash: String,
    pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct RawBlockchainBlockHeader {
    pub id: BlockRaw,
    pub mode: i32,
    pub header_proof: String,
}

#[derive(Deserialize, Debug)]
pub struct SendMessageResponse {
    pub code: i32,
}

#[derive(Deserialize, Debug)]
pub struct RawAccountState {
    pub id: BlockRaw,
    pub shardblk: BlockRaw,
    pub shard_proof: String,
    pub proof: String,
    pub state: String,
}

#[derive(Deserialize, Debug)]
pub struct RawShardInfo {
    pub id: BlockRaw,
    pub shardblk: BlockRaw,
    pub shard_proof: String,
    pub shard_descr: String,
}

#[derive(Deserialize, Debug)]
pub struct AllRawShardsInfo {
    pub id: BlockRaw,
    pub proof: String,
    pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct RawTransactions {
    pub ids: Vec<BlockRaw>,
    pub transactions: String,
}

#[derive(Deserialize, Debug)]
pub struct RawListBlockTransactions {
    pub id: BlockRaw,
    pub req_count: i32,
    pub incomplete: bool,
    pub ids: Vec<TransactionId>,
    pub proof: String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionId {
    pub mode: i32,
    pub account: Option<String>,
    pub lt: Option<i64>,
    pub hash: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RawBlockProof {
    pub complete: bool,
    pub from: BlockRaw,
    pub to: BlockRaw,
    pub steps: Vec<BlockProofStep>,
}

#[derive(Deserialize, Debug)]
pub struct BlockProofStep {
    pub lite_server_block_link_back: LiteServerBlockLinkBack,
    pub lite_server_block_link_forward: LiteServerBlockLinkForward,
}

#[derive(Deserialize, Debug)]
pub struct LiteServerBlockLinkBack {
    pub to_key_block: bool,
    pub from: BlockRaw,
    pub to: BlockRaw,
    pub dest_proof: String,
    pub proof: String,
    pub state_proof: String,
}

#[derive(Deserialize, Debug)]
pub struct LiteServerBlockLinkForward {
    pub to_key_block: bool,
    pub from: BlockRaw,
    pub to: BlockRaw,
    pub dest_proof: String,
    pub config_proof: String,
    pub signatures: Signatures,
}

#[derive(Deserialize, Debug)]
pub struct Signatures {
    pub validator_set_hash: i64,
    pub catchain_seqno: i32,
    pub signatures: Vec<Signature>,
}

#[derive(Deserialize, Debug)]
pub struct Signature {
    pub node_id_short: String,
    pub signature: String,
}

#[derive(Deserialize, Debug)]
pub struct RawConfig {
    pub mode: i32,
    pub id: BlockRaw,
    pub state_proof: String,
    pub config_proof: String,
}

#[derive(Deserialize, Debug)]
pub struct RawShardBlockProof {
    pub masterchain_id: BlockRaw,
    pub links: Vec<ShardBlockProofLink>,
}

#[derive(Deserialize, Debug)]
pub struct ShardBlockProofLink {
    pub id: BlockRaw,
    pub proof: String,
}

#[derive(Deserialize, Debug)]
pub struct OutMsgQueueSizes {
    pub ext_msg_queue_size_limit: u32,
    pub shards: Vec<ShardQueueSize>,
}

#[derive(Deserialize, Debug)]
pub struct ShardQueueSize {
    pub id: BlockRaw,
    pub size: u32,
}

#[derive(Debug, Deserialize)]
pub struct DecodedMessage {
    pub destination: AccountAddress,
    pub destination_wallet_version: String,
    pub ext_in_msg_decoded: Option<ExtInMsgDecoded>,
}

#[derive(Debug, Deserialize)]
pub struct ExtInMsgDecoded {
    pub wallet_v3: Option<WalletV3Message>,
    pub wallet_v4: Option<WalletV4Message>,
    pub wallet_highload_v2: Option<WalletHighloadV2Message>,
}

#[derive(Debug, Deserialize)]
pub struct WalletV3Message {
    pub subwallet_id: i64,
    pub valid_until: i64,
    pub seqno: i64,
    pub raw_messages: Vec<DecodedRawMessage>,
}

#[derive(Debug, Deserialize)]
pub struct WalletV4Message {
    pub subwallet_id: i64,
    pub valid_until: i64,
    pub seqno: i64,
    pub op: i32,
    pub raw_messages: Vec<DecodedRawMessage>,
}

#[derive(Debug, Deserialize)]
pub struct WalletHighloadV2Message {
    pub subwallet_id: i64,
    pub bounded_query_id: String,
    pub raw_messages: Vec<DecodedRawMessage>,
}

#[derive(Debug, Deserialize)]
pub struct DecodedRawMessage {
    pub message: DecodedMessageDetails,
    pub mode: i32,
}

#[derive(Debug, Deserialize)]
pub struct DecodedMessageDetails {
    pub boc: String,
    pub decoded_op_name: Option<String>,
    pub op_code: Option<String>,
    pub decoded_body: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct InscriptionBalances {
    pub inscriptions: Vec<InscriptionBalance>,
}

#[derive(Debug, Deserialize)]
pub struct InscriptionBalance {
    #[serde(rename = "type")]
    pub inscription_type: String,
    pub ticker: String,
    pub balance: String,
    pub decimals: i32,
}

#[derive(Deserialize, Debug)]
pub struct InscriptionOpTemplate {
    pub comment: String,
    pub destination: String,
}

#[derive(Deserialize, Debug)]
pub struct ServiceStatus {
    pub rest_online: bool,
    pub indexing_latency: i64,
    pub last_known_masterchain_seqno: i32,
}

#[derive(Deserialize, Debug)]
pub struct ParsedAddress {
    pub raw_form: String,
    pub bounceable: AddressFormat,
    pub non_bounceable: AddressFormat,
    pub given_type: String,
    pub test_only: bool,
}

#[derive(Deserialize, Debug)]
pub struct AddressFormat {
    pub b64: String,
    pub b64url: String,
}
