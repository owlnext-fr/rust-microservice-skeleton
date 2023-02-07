use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use stopwatch::Stopwatch;

use super::lock::OneAccessLock;

use crate::{
    core::commands::console_command_utils::ConsoleIO, domain::model::cron_log::CronLog,
    middlewares::cron_log_middleware::CronLogMiddleware,
};

use super::lock::FileLock;

/// shorthand type for command arguments structure.
type CommandArgs = HashMap<String, Option<String>>;

/// Final status of a command.
pub enum CommandResult {
    /// the command successfully terminated.
    SUCCESS,
    /// the command terminated with an error.
    ERROR(String),
    /// the command was skipped due to external requirements, probably a lock race-condition.
    SKIPPED(String),
}

/// A trait defining behaviours of a console command (e.g. a command runnable in the ConsoleCommandRegistry context).
#[async_trait]
pub trait ConsoleCommand: Send + Sync {
    /// gets the unique name of the command.
    fn get_name(&self) -> String;
    /// gets the cron middleware.
    fn get_cron_middleware(&self) -> &CronLogMiddleware;
    /// the main entrypoint of the command, consider it as the `fn main` of the command.
    async fn do_run(&self, args: &HashMap<String, Option<String>>) -> Result<CommandResult>;

    /// function executed before the entrypoint of a command.
    ///
    /// This will :
    /// - acquire a lock for the current command
    /// - create a cron log entry in the database
    /// - return the log for the current command
    async fn begin(&self, unicity_key: &str, args_as_str: &str) -> Result<CronLog> {
        FileLock::try_acquire(unicity_key).await?;

        let log = self
            .get_cron_middleware()
            .create_new(self.get_name().as_str(), args_as_str)?;

        Ok(log)
    }

    /// function executed after the entrypoint of a command.
    ///
    /// This will :
    /// - try to release the lock for the current command.
    /// - evaluate the exit status of the command.
    /// - register the termination status and message if any in the log.
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

    /// simulated entrypoint of a command. This method plays the `begin`, `do_run` and `end` functions of a command.
    ///
    /// This command will also trigger a stopwatch to monitor command time, intercept errors from `do_run` and exit properly.
    async fn run(&self, args: &CommandArgs) -> Result<()> {
        let sw = Stopwatch::start_new();
        let io = ConsoleIO::new();

        let args_as_str = self.get_args_as_str(args);
        let key = self.generate_unicity_key(&args_as_str);

        let cron_log = self.begin(&key, &args_as_str).await?;

        let result = self.do_run(args).await;

        if let Err(error) = &result {
            let error_text = error.to_string();
            io.error(&error_text);

            self.end(&key, &cron_log, CommandResult::ERROR(error_text))
                .await?;

            return Ok(());
        }

        let command_result = result.unwrap();
        self.end(&key, &cron_log, command_result).await?;

        println!();
        println!("-- Elapsed: {:.3}sec --", sw.elapsed().as_secs_f32());

        Ok(())
    }

    /// transforms a CommandArgs payload into a string, for lock purposes.
    fn get_args_as_str(&self, args: &CommandArgs) -> String {
        serde_json::to_string(&args).unwrap_or(String::from_str("{}").unwrap())
    }

    /// generates a unicity key for the current command.
    fn generate_unicity_key(&self, args_as_str: &str) -> String {
        format!("{}_{}", self.get_name(), args_as_str)
    }
}
