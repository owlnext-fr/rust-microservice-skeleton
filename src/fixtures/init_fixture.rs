use anyhow::Context;
use chrono::Utc;
use ulid::Ulid;

use crate::{
    core::{configuration::ConfigState, fairings::fixture::Fixture, password::generate_salt},
    domain::{
        model::{account::NewAccount, application::NewApplication, user::NewUser},
        repository::{
            account_repository::AccountRepository, application_repository::ApplicationRepository,
            user_repository::UserRepository,
        },
    },
    middlewares::{
        account_middleware::AccountMiddleware, application_middleware::ApplicationMiddleware,
        user_middleware::UserMiddleware,
    },
};

pub struct InitFixture {
    account_middleware: AccountMiddleware<AccountRepository>,
    application_middleware: ApplicationMiddleware<ApplicationRepository>,
    user_middleware: UserMiddleware<UserRepository>,
    configuration: ConfigState,
}

impl InitFixture {
    pub fn new(
        account_middleware: AccountMiddleware<AccountRepository>,
        application_middleware: ApplicationMiddleware<ApplicationRepository>,
        user_middleware: UserMiddleware<UserRepository>,
        configuration: ConfigState,
    ) -> Self {
        Self {
            account_middleware,
            application_middleware,
            user_middleware,
            configuration,
        }
    }
}

impl Fixture for InitFixture {
    fn load(&self) -> anyhow::Result<()> {
        debug!("init fixture");

        let launch = self
            .configuration
            .get_bool_or_default("enable_init_fixtures", false);

        debug!("{launch:?}");

        if launch {
            debug!("variable ok");

            let account_name = self
                .configuration
                .get_string("init_account_name")
                .with_context(|| {
                    "Cannot find init_account_name in env, required for init fixtures"
                })?;

            let application_name = self
                .configuration
                .get_string("init_application_name")
                .with_context(|| {
                    "Cannot find init_application_name in env, required for init fixtures"
                })?;

            let application_email = self
                .configuration
                .get_string("init_application_email")
                .with_context(|| {
                    "Cannot find init_application_email in env, required for init fixtures"
                })?;

            let user_login = self
                .configuration
                .get_string("init_user_login")
                .with_context(|| {
                    "Cannot find init_user_login in env, required for init fixtures"
                })?;

            let user_password = self
                .configuration
                .get_string("init_user_password")
                .with_context(|| {
                    "Cannot find init_user_password in env, required for init fixtures"
                })?;

            if self
                .account_middleware
                .find_one_by_name(&account_name)
                .is_err()
            {
                let account = self
                    .account_middleware
                    .create(NewAccount::new(account_name.as_str()))?;

                let application = self.application_middleware.create(NewApplication::new(
                    Ulid::new().to_string().as_str(),
                    application_name.as_str(),
                    application_email.as_str(),
                    account.id,
                ))?;

                let _user = self.user_middleware.create(NewUser {
                    email: Some(&application_email),
                    first_name: Some(&user_login),
                    last_name: Some(&user_login),
                    login: &user_login,
                    roles: vec!["ROLE_USER", "ROLE_USER_ADMIN"],
                    password: &user_password,
                    salt: Some(generate_salt().as_str()),
                    application_id: application.id,
                    created_date: Utc::now(),
                    created_by: None,
                    deleted_date: None,
                    deleted_by: None,
                    is_deleted: false,
                })?;
            }
        }

        Ok(())
    }
}
