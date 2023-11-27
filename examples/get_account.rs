use tonapi::RestApiV2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rest_api = RestApiV2::default();

    let account = rest_api
        .get_account("EQBszTJahYw3lpP64ryqscKQaDGk4QpsO7RO6LYVvKHSINS0")
        .await?;
    println!("Account: {}\n", account.balance);
    Ok(())
}
