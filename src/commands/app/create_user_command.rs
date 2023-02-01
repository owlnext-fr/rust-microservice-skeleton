use std::collections::HashMap;

use crate::{
    command_error, command_success,
    core::{
        commands::{
            console_command::{CommandResult, ConsoleCommand},
            console_command_utils::{ConsoleIO, LIST_SEPARATOR},
        },
        password::{self, generate_salt},
    },
    domain::model::user::NewUser,
    middlewares::{
        account_middleware::AccountMiddleware, application_middleware::ApplicationMiddleware,
        cron_log_middleware::CronLogMiddleware, user_middleware::UserMiddleware,
    },
};

use anyhow::Result;
use chrono::Utc;

pub struct CreateUserCommand {
    cron_log_middleware: CronLogMiddleware,
    account_middleware: AccountMiddleware,
    application_middleware: ApplicationMiddleware,
    user_middleware: UserMiddleware,
}

impl CreateUserCommand {
    pub fn new(
        cron_log_middleware: CronLogMiddleware,
        account_middleware: AccountMiddleware,
        application_middleware: ApplicationMiddleware,
        user_middleware: UserMiddleware,
    ) -> Self {
        Self {
            cron_log_middleware,
            account_middleware,
            application_middleware,
            user_middleware,
        }
    }
}

#[async_trait]
impl ConsoleCommand for CreateUserCommand {
    fn get_name(&self) -> String {
        "app:create-user".into()
    }

    fn get_cron_middleware(&self) -> &CronLogMiddleware {
        &self.cron_log_middleware
    }

    async fn do_run(&self, _args: &HashMap<String, Option<String>>) -> Result<CommandResult> {
        let io = ConsoleIO::new();

        io.title("User structure creation");

        io.comment("This command will guide you through user creation");
        io.new_line();
        io.note("First user will be created as a \"standard\" user, meaning you have to use app:elevate-user to promote this user as an administrator role.");
        io.new_line();

        let user_password = password::generate_sized(16);

        let account_id = io.ask_question("ID of the account:");
        let application_id = io.ask_question("Name of the application:");
        let user_login = io.ask_question("Login of the first user:");

        io.new_line();

        io.step(1, 3, &format!("Gathering account {account_id}..."));

        let account = self.account_middleware.find_one_by_id(&account_id)?;

        if account.is_none() {
            command_error!(&format!("Cannot find an account for {account_id}"));
        }

        let account = account.unwrap();

        io.step(2, 3, &format!("Gathering application {application_id}..."));
        let application = self
            .application_middleware
            .find_one_by_id(&application_id)?;

        if application.is_none() {
            command_error!(&format!("Cannot find an application for {application_id}"));
        }

        let application = application.unwrap();

        io.step(3, 3, &format!("Creating user {user_login}..."));
        let user = self.user_middleware.create(NewUser {
            email: Some(&application.contact_email),
            first_name: Some(&user_login),
            last_name: Some(&user_login),
            login: &user_login,
            roles: vec!["ROLE_USER"],
            password: &user_password,
            salt: Some(generate_salt().as_str()),
            application_id: application.id,
            created_date: Utc::now(),
            created_by: None,
            deleted_date: None,
            deleted_by: None,
            is_deleted: false,
        })?;

        io.new_line();
        io.success("Account, application and user created !");
        io.new_line();

        let mut data = Vec::<(&str, String)>::new();
        data.push(("Account ID", format!("{}", &account.id)));
        data.push(("", LIST_SEPARATOR.into()));
        data.push(("Application ID", format!("{}", &application.id)));
        data.push(("", LIST_SEPARATOR.into()));
        data.push(("User ID", format!("{}", &user.id)));
        data.push(("User login", user.login));
        data.push(("User password", user_password));

        io.key_value_pair(data);

        command_success!();
    }
}
