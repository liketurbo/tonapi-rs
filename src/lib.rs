use generated_tonapi::{
    apis::{
        accounts_api::{get_account, GetAccountError},
        configuration::Configuration,
        Error,
    },
    models::Account,
};

pub struct RestApiV2 {
    config: Configuration,
}

impl Default for RestApiV2 {
    fn default() -> Self {
        let mut config = Configuration::default();
        let user_agent = Some(format!(
            "{}@{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ));
        config.user_agent = user_agent;
        Self { config }
    }
}

impl RestApiV2 {
    pub async fn get_account(&self, account_id: &str) -> Result<Account, Error<GetAccountError>> {
        get_account(&self.config, account_id).await
    }
}
