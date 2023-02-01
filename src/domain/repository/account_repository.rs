use diesel::{prelude::*, sql_query, sql_types::Integer};

use crate::{
    core::database::{DbPoolState, DB},
    domain::{
        model::{
            account::{Account, NewAccount},
            user::User,
        },
        schema::account::{self, id, is_deleted, name},
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

    pub fn find_one_by_id(&self, account_id: i32) -> Result<Option<Account>> {
        let account = account::table
            .filter(id.eq(account_id))
            .filter(is_deleted.eq(false))
            .get_result::<Account>(&mut self.get_db())
            .optional()?;

        Ok(account)
    }

    pub fn find_one_by_name(&self, account_name: &str) -> Result<Option<Account>> {
        let account = account::table
            .filter(name.eq(account_name))
            .filter(is_deleted.eq(false))
            .get_result::<Account>(&mut self.get_db())
            .optional()?;

        Ok(account)
    }

    pub fn find_all_for_user(&self, user: &User, page: i32, per_page: i32) -> Result<Vec<Account>> {
        let offset = (page - 1) * per_page;

        let accounts = sql_query(
            "
            SELECT *
            FROM account ac
            INNER JOIN application app ON app.account_id = ac.id AND app.id = $1 AND app.is_deleted = false
            WHERE ac.is_deleted = false
            LIMIT $2
            OFFSET $3
        ",
        )
        .bind::<Integer, _>(user.id)
        .bind::<Integer, _>(per_page)
        .bind::<Integer, _>(offset)
        .load(&mut self.get_db())?;

        Ok(accounts)
    }

    pub fn find_for_user(&self, account_id: i32, user: &User) -> Result<Option<Account>> {
        let account = sql_query(
            "
            SELECT *
            FROM account ac
            INNER JOIN application app ON app.account_id = ac.id AND app.id = $1 AND app.is_deleted = false
            WHERE ac.is_deleted = false
            AND ac.id = $2
        ",
        )
        .bind::<Integer, _>(user.id)
        .bind::<Integer, _>(account_id)
        .get_result(&mut self.get_db()).optional()?;

        Ok(account)
    }
}
