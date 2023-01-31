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

    pub fn find_all_for_user(
        &self,
        user: &User,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Application>> {
        let offset = (page - 1) * per_page;

        let applications = sql_query(
            "
            SELECT *
            FROM application app
            INNER JOIN users u on u.application_id = app.id AND u.is_deleted = false and u.id = $1
            WHERE app.is_deleted = false
            AND app.id = $2
            LIMIT $3
            OFFSET $4
        ",
        )
        .bind::<Integer, _>(user.id)
        .bind::<Integer, _>(user.application_id)
        .bind::<Integer, _>(per_page)
        .bind::<Integer, _>(offset)
        .load(&mut self.get_db())?;

        Ok(applications)
    }

    pub fn find_for_user(&self, id: i32, user: &User) -> Result<Option<Application>> {
        let application = sql_query(
            "
            SELECT *
            FROM application app
            INNER JOIN users u ON u.application_id = app.id AND u.is_deleted = false AND u.id = $1
            WHERE app.is_deleted = false
            AND app.id = $2
        ",
        )
        .bind::<Integer, _>(user.id)
        .bind::<Integer, _>(id)
        .get_result(&mut self.get_db())
        .optional()?;

        Ok(application)
    }
}
