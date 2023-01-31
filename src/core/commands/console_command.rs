use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::Result;

use super::lock::OneAccessLock;

use crate::{
    domain::model::cron_log::CronLog, middlewares::cron_log_middleware::CronLogMiddleware,
};

use super::lock::FileLock;

type CommandArgs = HashMap<String, Option<String>>;

pub enum CommandResult {
    SUCCESS,
    ERROR(String),
    SKIPPED(String),
}

#[async_trait]
pub trait ConsoleCommand: Send + Sync {
    fn get_name(&self) -> String;
    fn get_cron_middleware(&self) -> &CronLogMiddleware;
    async fn do_run(&self, args: &HashMap<String, Option<String>>) -> Result<CommandResult>;

    async fn begin(&self, unicity_key: &str, args_as_str: &str) -> Result<CronLog> {
        FileLock::try_aquire(unicity_key).await?;

        let log = self
            .get_cron_middleware()
            .create_new(self.get_name().as_str(), args_as_str)?;

        Ok(log)
    }

    async fn end(
        &self,
        unicity_key: &str,
        cron_log: &CronLog,
        result: CommandResult,
    ) -> Result<()> {
        FileLock::try_release(unicity_key).await?;

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

    async fn run(&self, args: &CommandArgs) -> Result<()> {
        let args_as_str = self.get_args_as_str(args);
        let key = self.generate_unicity_key(&args_as_str);

        let cron_log = self.begin(&key, &args_as_str).await?;

        let result = self.do_run(args).await;

        if let Err(error) = &result {
            self.end(&key, &cron_log, CommandResult::ERROR(error.to_string()))
                .await?;
        }

        let command_result = result.unwrap();
        self.end(&key, &cron_log, command_result).await?;

        Ok(())
    }

    fn get_args_as_str(&self, args: &CommandArgs) -> String {
        serde_json::to_string(&args).unwrap_or(String::from_str("{}").unwrap())
    }

    fn generate_unicity_key(&self, args_as_str: &str) -> String {
        format!("{}_{}", self.get_name(), args_as_str)
    }
}

pub struct CommandHandler<T: ConsoleCommand + ?Sized + Send + Sync> {
    pub command: Arc<T>,
}

impl<T> Clone for CommandHandler<T>
where
    T: ConsoleCommand + ?Sized + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            command: self.command.clone(),
        }
    }
}
