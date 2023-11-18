use tonapi_sdk_rs::apis::{accounts_api::get_account_nft_items, configuration::Configuration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();
    let nfts_items = get_account_nft_items(
        &config,
        "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0",
        None,
        Some(10),
        None,
        None,
    )
    .await?;
    println!("Len: {:?}", nfts_items.nft_items.len());
    Ok(())
}
