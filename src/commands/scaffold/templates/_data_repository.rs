use diesel::prelude::*;

use crate::{
    core::database::{DbPoolState, DB},
    domain::{
        model::__MODULE_NAME__::{__DATA_CLASS_STRUCT_NAME__, __NEW_DATA_CLASS_STRUCT_NAME__},
        schema::__MODULE_NAME_PLURAL__::*,
    },
};

use anyhow::Result;

#[derive(Clone)]
pub struct __DATA_CLASS_STRUCT_NAME__Repository {
    db_conn: DbPoolState,
}

impl __DATA_CLASS_STRUCT_NAME__Repository {
    pub fn new(db_pool: DbPoolState) -> Self {
        Self { db_conn: db_pool }
    }

    fn get_db(&self) -> DB {
        self.db_conn.db_pool.get().unwrap()
    }

    pub fn find_one_by_id(
        &self,
        __MODULE_NAME___id: i32,
    ) -> Result<Option<__DATA_CLASS_STRUCT_NAME__>> {
        let __MODULE_NAME__ = __MODULE_NAME_PLURAL__::table
            .filter(id.eq(user_id))
            .filter(is_deleted.eq(false))
            .get_result::<__DATA_CLASS_STRUCT_NAME__>(&mut self.get_db())
            .optional()?;

        Ok(__MODULE_NAME__)
    }

    pub fn find_all_for_application_id(
        &self,
        __MODULE_NAME___application_id: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<__DATA_CLASS_STRUCT_NAME__>> {
        let offset = (page - 1) * per_page;

        let __MODULE_NAME__s = __MODULE_NAME__s::table
            .filter(application_id.eq(__MODULE_NAME___application_id))
            .filter(is_deleted.eq(false))
            .limit(per_page.into())
            .offset(offset.into())
            .order(created_date.asc())
            .get_results::<__DATA_CLASS_STRUCT_NAME__>(&mut self.get_db())?;

        Ok(__MODULE_NAME__s)
    }

    pub fn find_one_with_application_id(
        &self,
        __MODULE_NAME__id: i32,
        __MODULE_NAME___application_id: i32,
    ) -> Result<Option<__DATA_CLASS_STRUCT_NAME__>> {
        let __MODULE_NAME__ = __MODULE_NAME__s::table
            .filter(id.eq(__MODULE_NAME__id))
            .filter(application_id.eq(__MODULE_NAME___application_id))
            .filter(is_deleted.eq(false))
            .get_result::<__DATA_CLASS_STRUCT_NAME__>(&mut self.get_db())
            .optional()?;

        Ok(__MODULE_NAME__)
    }

    pub fn insert(
        &self,
        new___MODULE_NAME__: __NEW_DATA_CLASS_STRUCT_NAME__,
    ) -> Result<__DATA_CLASS_STRUCT_NAME__> {
        let __MODULE_NAME__ = diesel::insert_into(__MODULE_NAME_PLURAL__::table)
            .values(&new___MODULE_NAME__)
            .get_result(&mut self.get_db())?;

        Ok(__MODULE_NAME__)
    }

    pub fn update(
        &self,
        updated___MODULE_NAME__: &__DATA_CLASS_STRUCT_NAME__,
    ) -> Result<__DATA_CLASS_STRUCT_NAME__> {
        let __MODULE_NAME__ = diesel::update(updated___MODULE_NAME__)
            .set(updated___MODULE_NAME__)
            .get_result(&mut self.get_db())?;

        Ok(__MODULE_NAME__)
    }
}
