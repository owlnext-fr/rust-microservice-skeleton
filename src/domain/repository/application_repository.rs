use diesel::{prelude::*, sql_query, sql_types::Integer};

use crate::{
    core::database::{DbPoolState, DB},
    domain::{
        model::{
            application::{Application, NewApplication},
            user::User,
        },
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

    pub fn find_one_for_user(&self, user: &User) -> Result<Option<Application>> {
        let application = sql_query(
            "
            SELECT *
            FROM application app
            WHERE app.is_deleted = false
            AND app.id = $1
        ",
        )
        .bind::<Integer, _>(user.id)
        .get_result(&mut self.get_db())
        .optional()?;

        Ok(application)
    }
}
