#[macro_use]
extern crate serde_derive;

pub mod stream_api;

mod codegen {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/src/lib.rs"));
}

macro_rules! reexport_fn {
    ($module:ident::$func:ident) => {
        pub async fn $func(
            &self,
            account_id: &str,
        ) -> Result<models::Account, Error<$module::GetAccountError>> {
            $module::$func(&self.config, account_id).await
        }
    };
}

use anyhow::Ok;
use codegen::{
    apis::{accounts_api, configuration::Configuration, Error},
    models,
};

pub struct RestApiV2 {
    pub config: codegen::apis::configuration::Configuration,
}

impl Default for RestApiV2 {
    fn default() -> Self {
        let mut config = Configuration::default();
        let user_agent = Some(format!(
            "{}@{}",
            env!("CARGO_PKG_REPOSITORY")
                .split("/")
                .last()
                .unwrap_or(env!("CARGO_PKG_NAME")),
            env!("CARGO_PKG_VERSION")
        ));
        config.user_agent = user_agent;
        Self { config }
    }
}
