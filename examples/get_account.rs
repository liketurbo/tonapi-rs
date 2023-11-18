use tonapi_sdk_rs::apis::{accounts_api::get_account, configuration::Configuration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();
    let account = get_account(&config, "EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0").await?;
    println!("Account: {}\n", account.balance);
    Ok(())
}
