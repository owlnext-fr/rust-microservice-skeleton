use diesel::prelude::*;

use crate::{
    core::database::{DbPoolState, DB},
    domain::{
        model::user::User,
        schema::{
            users::{id, is_deleted, login},
            *,
        },
    },
};

use anyhow::Result;

#[derive(Clone)]
pub struct UserRepository {
    db_conn: DbPoolState,
}

impl UserRepository {
    pub fn new(db_pool: DbPoolState) -> Self {
        Self { db_conn: db_pool }
    }

    fn get_db(&self) -> DB {
        self.db_conn.db_pool.get().unwrap()
    }

    pub fn load_user_by_login(&self, user_login: &str) -> Result<User> {
        let user = users::table
            .filter(login.eq(user_login))
            .filter(is_deleted.eq(false))
            .get_result::<User>(&mut self.get_db())?;

        Ok(user)
    }

    pub fn find_by_id(&self, user_id: i32) -> Result<User> {
        let user = users::table
            .filter(id.eq(user_id))
            .get_result::<User>(&mut self.get_db())?;

        Ok(user)
    }
}
