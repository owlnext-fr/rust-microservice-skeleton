use diesel::prelude::*;

use crate::{
    core::database::{DbPoolState, DB},
    domain::{
        model::application::{Application, NewApplication},
        schema::*,
    },
};

use anyhow::Result;

#[derive(Clone)]
pub struct ApplicationRepository {
    db_conn: DbPoolState,
}

impl ApplicationRepository {
    pub fn new(db_pool: DbPoolState) -> Self {
        Self { db_conn: db_pool }
    }

    fn get_db(&self) -> DB {
        self.db_conn.db_pool.get().unwrap()
    }

    pub fn insert(&self, new_application: NewApplication) -> Result<Application> {
        let account = diesel::insert_into(application::table)
            .values(&new_application)
            .get_result(&mut self.get_db())?;

        Ok(account)
    }
}
