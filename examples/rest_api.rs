use simple_logger::SimpleLogger;
use tonapi::{
    rest_api::{
        accounts_api::{GetAccountNftItemsParams, GetAccountParams},
        blockchain_api::GetBlockchainAccountTransactionsParams,
        RestApi, RestApiConfig,
    },
    TonAddress,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().expect("logging init");
    let rest_api = RestApi::new(RestApiConfig { auth_token: None });
    let account = rest_api
        .get_account(GetAccountParams {
            account_id: TonAddress::from_base64_url(
                "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0",
            )?,
        })
        .await?;
    println!(
        "Account balance: {}; address: {}",
        account.balance, account.address
    );

    let res = rest_api
        .get_account_nft_items(GetAccountNftItemsParams {
            account_id: TonAddress::from_base64_url(
                "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0",
            )?,
            collection: None,
            indirect_ownership: None,
            limit: Some(10),
            offset: None,
        })
        .await?;
    println!("Nfts len: {}", res.nft_items.len());

    let res = rest_api
        .get_blockchain_account_transactions(GetBlockchainAccountTransactionsParams {
            account_id: TonAddress::from_base64_url(
                "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0",
            )?,
            after_lt: Some(25758317000002),
            before_lt: None,
            limit: Some(10),
        })
        .await?;
    println!("Transactions len: {}", res.transactions.len());

    Ok(())
}
