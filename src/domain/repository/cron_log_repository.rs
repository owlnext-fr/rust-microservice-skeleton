use diesel::{prelude::*, result::Error};

use crate::{
    core::database::{DbPoolState, DB},
    domain::{
        model::cron_log::CronLog,
        schema::{cron_logs::*, *},
    },
};

#[derive(Clone)]
pub struct CronLogRepository {
    db_conn: DbPoolState,
}

impl CronLogRepository {
    pub fn new(db_pool: DbPoolState) -> Self {
        Self { db_conn: db_pool }
    }

    fn get_db(&self) -> DB {
        self.db_conn.db_pool.get().unwrap()
    }
}
