use failure::Fail;

use crate::{
    core::database::DbPoolState,
    domain::{
        dto::auth::LoginInputDTO,
        model::user::User,
        repository::{
            traits::database_enabled_repository_trait::DatabaseEnabledRepositoryTrait,
            user_repository::UserRepository,
        },
    },
};

use super::traits::db_enabled_middleware_trait::DbEnabledMiddlewareTrait;

#[derive(Debug, Fail)]
#[fail(display = "Authentication error: {}.", _0)]
pub struct AuthenticationError(String);

#[derive(Default)]
pub struct UserMiddleware<UserRepository> {
    repository: UserRepository,
}

impl DbEnabledMiddlewareTrait<UserRepository> for UserMiddleware<UserRepository> {
    fn setup_db(&mut self, db_conn: DbPoolState) -> &mut Self {
        let mut repository = UserRepository::default();
        repository.set_db(db_conn);

        self.repository = repository;

        self
    }

    fn get_repository(&self) -> UserRepository {
        self.repository.clone()
    }
}

impl UserMiddleware<UserRepository> {
    pub fn authenticate_user_from_input(
        &self,
        input: &LoginInputDTO,
    ) -> Result<User, AuthenticationError> {
        let user_found = self
            .get_repository()
            .load_user_by_login(input.login.as_str());

        if let Ok(user) = user_found {
            return Ok(user);
        }

        Err(AuthenticationError("User not found".into()))
    }
}
