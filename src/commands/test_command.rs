use crate::{
    core::commands::command::{Command, CommandResult},
    domain::repository::cron_log_repository::CronLogRepository,
    middlewares::cron_log_middleware::CronLogMiddleware,
};
use anyhow::Result;
use std::collections::HashMap;

/// a testing purpose command
pub struct TestCommand {
    pub name: String,
    pub args: Option<HashMap<String, String>>,
    pub cron_log_middleware: CronLogMiddleware<CronLogRepository>,
}

#[async_trait]
impl Command for TestCommand {
    fn get_command_name(&self) -> String {
        self.name.clone()
    }

    fn get_command_args(&self) -> Option<HashMap<String, String>> {
        self.args.clone()
    }

    fn get_cron_middleware(&self) -> CronLogMiddleware<CronLogRepository> {
        self.cron_log_middleware.clone()
    }

    async fn do_run(&self) -> Result<CommandResult> {
        debug!("executed !");

        Ok(CommandResult::SUCCESS)
    }
}
