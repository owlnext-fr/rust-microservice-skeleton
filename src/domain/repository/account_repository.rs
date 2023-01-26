use diesel::prelude::*;

use crate::{
    core::database::{DbPoolState, DB},
    domain::{
        model::account::{Account, NewAccount},
        schema::account::{self, name},
    },
};

use anyhow::Result;

#[derive(Clone)]
pub struct AccountRepository {
    db_conn: DbPoolState,
}

impl AccountRepository {
    pub fn new(db_pool: DbPoolState) -> Self {
        Self { db_conn: db_pool }
    }

    fn get_db(&self) -> DB {
        self.db_conn.db_pool.get().unwrap()
    }

    pub fn insert(&self, new_account: NewAccount) -> Result<Account> {
        let account = diesel::insert_into(account::table)
            .values(&new_account)
            .get_result(&mut self.get_db())?;

        Ok(account)
    }

    pub fn find_one_by_name(&self, account_name: &str) -> Result<Account> {
        let account = account::table
            .filter(name.eq(account_name))
            .get_result::<Account>(&mut self.get_db())?;

        Ok(account)
    }
}
