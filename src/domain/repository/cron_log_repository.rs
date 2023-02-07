use anyhow::Result;
use diesel::prelude::*;

use crate::{
    core::database::{DbPoolState, DB},
    domain::{
        model::cron_log::{CronLog, NewCronLog},
        schema::cron_logs,
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

    pub fn insert(&self, new_cron_log: NewCronLog) -> Result<CronLog> {
        let refresh_token = diesel::insert_into(cron_logs::table)
            .values(&new_cron_log)
            .get_result(&mut self.get_db())?;

        Ok(refresh_token)
    }

    pub fn update(&self, cron_log: &CronLog) -> Result<CronLog> {
        let completed = diesel::update(cron_log)
            .set(cron_log)
            .get_result(&mut self.get_db())?;

        Ok(completed)
    }
}
