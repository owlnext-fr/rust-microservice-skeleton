use crate::{
    core::{configuration::ConfigState, database::DBRequestResultError, jwt, password},
    domain::{
        dto::auth::LoginInputDTO,
        model::user::User,
        repository::{cron_log_repository::CronLogRepository, user_repository::UserRepository},
    },
};

#[derive(Default)]
pub struct CronLogMiddleware<CronLogRepository> {
    repository: CronLogRepository,
}

impl CronLogMiddleware<CronLogRepository> {
    pub fn new(repository: CronLogRepository) -> Self {
        Self { repository }
    }
}
