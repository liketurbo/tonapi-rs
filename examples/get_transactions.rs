use tonapi_sdk_rs::apis::{
    blockchain_api::get_blockchain_account_transactions, configuration::Configuration,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();
    let transactions = get_blockchain_account_transactions(
        &config,
        "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0",
        Some(25758317000002),
        None,
        Some(10),
    )
    .await?;
    println!("Len: {:?}", transactions.transactions.len());
    Ok(())
}
