use crate::{
    core::commands::console_command::{CommandResult, ConsoleCommand},
    domain::repository::cron_log_repository::CronLogRepository,
    middlewares::cron_log_middleware::CronLogMiddleware,
};

use anyhow::Result;
use std::collections::HashMap;

pub struct TestCommand {
    cron_log_middleware: CronLogMiddleware<CronLogRepository>,
}

impl TestCommand {
    pub fn new(cron_log_middleware: CronLogMiddleware<CronLogRepository>) -> Self {
        Self {
            cron_log_middleware,
        }
    }
}

#[async_trait]
impl ConsoleCommand for TestCommand {
    fn get_name(&self) -> String {
        "app:test".into()
    }

    fn get_cron_middleware(&self) -> &CronLogMiddleware<CronLogRepository> {
        &self.cron_log_middleware
    }

    async fn do_run(&self, args: &HashMap<String, Option<String>>) -> Result<CommandResult> {
        println!("{args:#?}");

        Ok(CommandResult::SUCCESS)
    }
}
