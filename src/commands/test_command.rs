use crate::{
    core::commands::console_command::{CommandResult, ConsoleCommand},
    middlewares::cron_log_middleware::CronLogMiddleware,
};

use anyhow::Result;
use std::collections::HashMap;

pub struct TestCommand {
    cron_log_middleware: CronLogMiddleware,
}

impl TestCommand {
    pub fn new(cron_log_middleware: CronLogMiddleware) -> Self {
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

    fn get_cron_middleware(&self) -> &CronLogMiddleware {
        &self.cron_log_middleware
    }

    async fn do_run(&self, args: &HashMap<String, Option<String>>) -> Result<CommandResult> {
        println!("{args:#?}");

        Ok(CommandResult::SUCCESS)
    }
}
