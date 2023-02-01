use std::collections::HashMap;

use crate::{
    command_error, command_success,
    core::commands::{
        console_command::{CommandResult, ConsoleCommand},
        console_command_utils::ConsoleIO,
    },
    middlewares::{cron_log_middleware::CronLogMiddleware, user_middleware::UserMiddleware},
};

use anyhow::Result;

pub struct PromoteUserCommand {
    cron_log_middleware: CronLogMiddleware,
    user_middleware: UserMiddleware,
}

impl PromoteUserCommand {
    pub fn new(cron_log_middleware: CronLogMiddleware, user_middleware: UserMiddleware) -> Self {
        Self {
            cron_log_middleware,
            user_middleware,
        }
    }
}

#[async_trait]
impl ConsoleCommand for PromoteUserCommand {
    fn get_name(&self) -> String {
        "app:user-promotion".into()
    }

    fn get_cron_middleware(&self) -> &CronLogMiddleware {
        &self.cron_log_middleware
    }

    async fn do_run(&self, args: &HashMap<String, Option<String>>) -> Result<CommandResult> {
        let io = ConsoleIO::new();

        io.title("User promotion");

        io.comment("This command will guide you through user promotion as administrator.");
        io.new_line();

        let user_id = if args.contains_key("user-id") {
            args.get("user-id").unwrap().as_ref().unwrap().clone()
        } else {
            io.ask_question("What is the user ID to promote:")
        };

        if user_id.parse::<i32>().is_err() {
            command_error!(&format!("{user_id} is not a valid user ID."));
        }

        io.step(1, 2, "Fetching user {user_id}...");

        let user = self.user_middleware.find_one_by_id(&user_id)?;

        if user.is_none() {
            command_error!(&format!("User ID {user_id} is not found."));
        }

        let user = user.unwrap();

        io.step(2, 2, "Promoting user {user_id}...");
        let user = self.user_middleware.promote(&user)?;

        io.success(&format!("User {} successfully promoted !", user.login));

        command_success!();
    }
}
