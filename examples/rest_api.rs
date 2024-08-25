use tonapi::{Network, RestApiClientV2};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RestApiClientV2::new(Network::Mainnet, None);

    let result = client.get_account("0QCbOix87iy37AwRCWaYhJHzc2gXE_WnAG5vVEAySNT7zClz").await.unwrap();
    println!("Account balance: {}", result.balance);

    Ok(())
}