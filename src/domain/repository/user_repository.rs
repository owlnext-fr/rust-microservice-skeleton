use diesel::prelude::*;

use crate::{
    core::database::{DbPoolState, DB},
    domain::{
        model::user::{NewUser, User},
        schema::{
            users::{application_id, created_date, id, is_deleted, login},
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

    pub fn find_one_by_id(&self, user_id: i32) -> Result<Option<User>> {
        let user = users::table
            .filter(id.eq(user_id))
            .get_result::<User>(&mut self.get_db())
            .optional()?;

        Ok(user)
    }

    pub fn find_one_by_login(&self, user_login: &str) -> Result<Option<User>> {
        let user = users::table
            .filter(login.eq(user_login))
            .filter(is_deleted.eq(false))
            .get_result::<User>(&mut self.get_db())
            .optional()?;

        Ok(user)
    }

    pub fn find_all_for_application_id(
        &self,
        user_application_id: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<User>> {
        let offset = (page - 1) * per_page;

        let users = users::table
            .filter(application_id.eq(user_application_id))
            .filter(is_deleted.eq(false))
            .limit(per_page.into())
            .offset(offset.into())
            .order(created_date.asc())
            .get_results::<User>(&mut self.get_db())?;

        Ok(users)
    }

    pub fn find_one_for_user_and_application(
        &self,
        user_id: i32,
        user_application_id: i32,
    ) -> Result<Option<User>> {
        let user = users::table
            .filter(id.eq(user_id))
            .filter(application_id.eq(user_application_id))
            .filter(is_deleted.eq(false))
            .get_result::<User>(&mut self.get_db())
            .optional()?;

        Ok(user)
    }

    pub fn insert(&self, new_user: NewUser) -> Result<User> {
        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&mut self.get_db())?;

        Ok(user)
    }

    pub fn update(&self, updated_user: &User) -> Result<User> {
        let updated = diesel::update(updated_user)
            .set(updated_user)
            .get_result(&mut self.get_db())?;

        Ok(updated)
    }
}
