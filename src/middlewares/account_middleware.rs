use crate::domain::{
    model::{
        account::{Account, NewAccount},
        user::User,
    },
    repository::account_repository::AccountRepository,
};

use anyhow::Result;

#[derive(Default, Clone)]
pub struct AccountMiddleware<AccountRepository> {
    repository: AccountRepository,
}

impl AccountMiddleware<AccountRepository> {
    pub fn new(repository: AccountRepository) -> Self {
        Self { repository }
    }

    pub fn create(&self, new_account: NewAccount) -> Result<Account> {
        let account = self.repository.insert(new_account)?;

        Ok(account)
    }

    pub fn find_one_by_name(&self, name: &str) -> Result<Account> {
        let account = self.repository.find_one_by_name(name)?;

        Ok(account)
    }

    pub fn list_account_for_user(&self, user: &User) -> Result<Vec<Account>> {
        let accounts = self.repository.find_for_user(user)?;

        Ok(accounts)
    }
}
