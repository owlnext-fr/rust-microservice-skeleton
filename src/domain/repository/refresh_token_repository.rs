use crate::{
    core::database::{DbPoolState, DB},
    domain::{
        model::refresh_token::{NewRefreshToken, RefreshToken},
        schema::{refresh_token::token, *},
    },
};
use diesel::prelude::*;

use anyhow::Result;

#[derive(Clone)]
pub struct RefreshTokenRepository {
    db_conn: DbPoolState,
}

impl RefreshTokenRepository {
    pub fn new(db_pool: DbPoolState) -> Self {
        Self { db_conn: db_pool }
    }

    fn get_db(&self) -> DB {
        self.db_conn.db_pool.get().unwrap()
    }

    pub fn insert(&self, new_refresh_token: NewRefreshToken) -> Result<RefreshToken> {
        let refresh_token = diesel::insert_into(refresh_token::table)
            .values(&new_refresh_token)
            .get_result(&mut self.get_db())?;

        Ok(refresh_token)
    }

    pub fn find_by_token(&self, refresh_token: &str) -> Result<RefreshToken> {
        let refresh_token = refresh_token::table
            .filter(token.eq(refresh_token))
            .get_result::<RefreshToken>(&mut self.get_db())?;

        Ok(refresh_token)
    }
}
