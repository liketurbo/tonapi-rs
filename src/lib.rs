mod stream_api;
mod rest_api;

pub use stream_api::*;
pub use rest_api::*;

pub enum Network {
    Mainnet,
    Testnet,
}
