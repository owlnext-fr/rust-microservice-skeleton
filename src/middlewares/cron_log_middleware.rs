use anyhow::Result;
use chrono::Utc;

use crate::domain::{
    model::cron_log::{CronLog, NewCronLog},
    repository::cron_log_repository::CronLogRepository,
};

#[derive(Default, Clone, Copy)]
pub struct CronLogMiddleware<CronLogRepository> {
    repository: CronLogRepository,
}

impl CronLogMiddleware<CronLogRepository> {
    pub fn new(repository: CronLogRepository) -> Self {
        Self { repository }
    }

    pub fn create_new(&self, command: &str, command_args: &str) -> Result<CronLog> {
        let new_cron_log = NewCronLog {
            command,
            command_args,
            started_at: Utc::now(),
        };

        let log = self.repository.insert(new_cron_log)?;

        Ok(log)
    }

    pub fn close(
        &self,
        cron_log: &CronLog,
        exit_status: i32,
        exit_message: Option<String>,
    ) -> Result<CronLog> {
        let log = self
            .repository
            .update_from_completion(cron_log, exit_status, exit_message)?;

        Ok(log)
    }
}