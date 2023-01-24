use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::Result;

use super::lock::OneAccessLock;

use crate::{
    domain::{model::cron_log::CronLog, repository::cron_log_repository::CronLogRepository},
    middlewares::cron_log_middleware::CronLogMiddleware,
};

use super::lock::FileLock;

/// Synthetize a command execution result.
pub enum CommandResult {
    SUCCESS,
    ERROR(String),
    SKIPPED(String),
}

/// Trait to define structs as runnable async crons with tokio_scheduler
#[async_trait]
pub trait Command: Send + Sync {
    /// returns the current command name
    fn get_command_name(&self) -> String;

    /// returns the current command argument payload
    fn get_command_args(&self) -> Option<HashMap<String, String>>;

    /// returns the "cron_middleware"
    fn get_cron_middleware(&self) -> CronLogMiddleware<CronLogRepository>;

    /// real body for the command execution, must be overriden in impls.
    async fn do_run(&self) -> Result<CommandResult>;

    /// starts the command process by validating command lock, and registering an open cron log into database.
    async fn begin(&self) -> Result<CronLog> {
        let key = self.generate_unicity_key();

        FileLock::try_aquire(key.as_str()).await?;

        let log = self.get_cron_middleware().create_new(
            self.get_command_name().as_str(),
            self.get_command_args_as_string().as_str(),
        )?;

        Ok(log)
    }

    /// ends the command process by releasing command lock, and registering the result of the command to an opened cron log into database.
    async fn end(&self, cron_log: &CronLog, result: CommandResult) -> Result<()> {
        let key = self.generate_unicity_key();

        FileLock::try_release(key.as_str()).await?;

        let mut exit_status = 0;
        let mut exit_message = None;

        match result {
            CommandResult::ERROR(message) => {
                exit_status = 1;
                exit_message = Some(message);
            }
            CommandResult::SKIPPED(message) => {
                exit_status = 2;
                exit_message = Some(message);
            }
            _ => (),
        };

        self.get_cron_middleware()
            .close(cron_log, exit_status, exit_message)?;

        Ok(())
    }

    /// hidden runner of commands, uses begin, end and do_run, and will be used by runner.
    async fn run(&self) -> Result<()> {
        let cron_log = self.begin().await?;

        let result = self.do_run().await;

        if let Err(error) = &result {
            self.end(&cron_log, CommandResult::ERROR(error.to_string()))
                .await?;
        }

        let command_result = result.unwrap();
        self.end(&cron_log, command_result).await?;

        Ok(())
    }

    /// generates a unique key for this command name + args, for locks purposes
    fn generate_unicity_key(&self) -> String {
        format!(
            "{}_{}",
            self.get_command_name(),
            self.get_command_args_as_string()
        )
    }

    /// converts command args as a string payload
    #[allow(clippy::or_fun_call)]
    fn get_command_args_as_string(&self) -> String {
        let args = &self.get_command_args().unwrap_or(HashMap::new());

        serde_json::to_string(&args).unwrap_or(String::from_str("{}").unwrap())
    }
}

/// struct to move a command + its cron schedule into scheduler.
pub struct CommandHandle<T: Command + ?Sized + Send + Sync> {
    pub command: Arc<T>,
    pub schedule: String,
}

impl<T> Clone for CommandHandle<T>
where
    T: Command + ?Sized + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            command: self.command.clone(),
            schedule: self.schedule.clone(),
        }
    }
}
