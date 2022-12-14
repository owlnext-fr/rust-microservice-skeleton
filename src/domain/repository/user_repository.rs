use diesel::{prelude::*, result::Error};

use super::traits::database_enabled_repository_trait::DatabaseEnabledRepositoryTrait;
use crate::{
    core::database::DbPoolState,
    domain::{
        model::user::User,
        schema::{
            user::{is_deleted, login},
            *,
        },
    },
};

#[derive(Default, Clone)]
pub struct UserRepository {
    db_conn: Option<DbPoolState>,
}

impl DatabaseEnabledRepositoryTrait for UserRepository {
    fn set_db(&mut self, db_conn: DbPoolState) -> &mut Self {
        self.db_conn = Some(db_conn);
        self
    }

    fn get_db(
        &self,
    ) -> r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>> {
        self.db_conn.clone().unwrap().db_pool.get().unwrap()
    }
}

impl UserRepository {
    pub fn load_user_by_login(&self, user_login: &str) -> Result<User, Error> {
        let mut conn = self.get_db();

        user::table
            .filter(login.eq(user_login))
            .filter(is_deleted.eq(false))
            .get_result::<User>(&mut conn)
    }
}
