use anyhow::Result;
use chrono::Utc;

use crate::domain::{
    model::cron_log::{CronLog, NewCronLog},
    repository::cron_log_repository::CronLogRepository,
};

/// CronLog middleware
#[derive(Clone)]
pub struct CronLogMiddleware {
    repository: CronLogRepository,
}

impl CronLogMiddleware {
    /// constructor.
    pub fn new(repository: CronLogRepository) -> Self {
        Self { repository }
    }

    /// creates and registers a new CronLog given a command and its arguments
    pub fn create_new(&self, command: &str, command_args: &str) -> Result<CronLog> {
        let new_cron_log = NewCronLog {
            command,
            command_args,
            started_at: Utc::now(),
        };

        let log = self.repository.insert(new_cron_log)?;

        Ok(log)
    }

    /// closes a given log with an exit status an optionnally an exit message.
    pub fn close(
        &self,
        cron_log: &CronLog,
        exit_status: i32,
        exit_message: Option<String>,
    ) -> Result<CronLog> {
        let mut updated_cron_log = cron_log.clone();
        updated_cron_log.exit_status = Some(exit_status);
        updated_cron_log.exit_message = exit_message;

        let log = self.repository.update(cron_log)?;

        Ok(log)
    }
}
