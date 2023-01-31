use crate::domain::{
    dto::account::{AccountDetailsDTO, AccountListItemDTO},
    model::{
        account::{Account, NewAccount},
        user::User,
    },
    repository::account_repository::AccountRepository,
};

use anyhow::Result;

#[derive(Clone)]
pub struct AccountMiddleware {
    repository: AccountRepository,
}

impl AccountMiddleware {
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

    pub fn list_account_for_user(
        &self,
        user: &User,
        page: u16,
        per_page: u16,
    ) -> Result<Vec<Account>> {
        let accounts = self
            .repository
            .find_all_for_user(user, page.into(), per_page.into())?;

        Ok(accounts)
    }

    pub fn find_account_for_user(&self, id: &str, user: &User) -> Result<Option<Account>> {
        let real_id = id.parse::<i32>()?;

        let account = self.repository.find_for_user(real_id, user)?;

        Ok(account)
    }

    pub fn to_list_dto(&self, accounts: Vec<Account>) -> Vec<AccountListItemDTO> {
        let mut list_dto = Vec::<AccountListItemDTO>::new();

        for account in accounts.iter() {
            list_dto.push(AccountListItemDTO::from(account));
        }

        list_dto
    }

    pub fn to_details_dto(&self, account: &Account) -> AccountDetailsDTO {
        AccountDetailsDTO::from(account)
    }
}
