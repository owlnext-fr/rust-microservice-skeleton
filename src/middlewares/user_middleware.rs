use std::error::Error;

use failure::Fail;

use crate::{
    core::{configuration::ConfigState, database::DBRequestResultError, jwt, password},
    domain::{
        dto::auth::LoginInputDTO, model::user::User, repository::user_repository::UserRepository,
    },
};

#[derive(Debug, Fail)]
pub enum AuthenticationError {
    #[fail(display = "User {} not found.", _0)]
    UserNotFound(String),
    #[fail(display = "Wrong password for user {}", _0)]
    WrongPassword(i32),
}

#[derive(Default)]
pub struct UserMiddleware<UserRepository> {
    repository: UserRepository,
    configuration: ConfigState,
}

impl UserMiddleware<UserRepository> {
    pub fn new(repository: UserRepository, configuration: ConfigState) -> Self {
        Self {
            repository,
            configuration,
        }
    }

    pub fn authenticate_user_from_input(
        &self,
        input: &LoginInputDTO,
    ) -> Result<User, AuthenticationError> {
        let user_found = self.repository.load_user_by_login(input.login.as_str());

        if let Ok(user) = user_found {
            let is_password_valid = password::compare_hashed(&input.password, &user.password);

            if is_password_valid {
                return Ok(user);
            }

            return Err(AuthenticationError::WrongPassword(user.id));
        }

        Err(AuthenticationError::UserNotFound(input.login.clone()))
    }

    pub fn create_jwt_for_user(&self, user: &User) -> Result<String, Box<dyn Error>> {
        let claim = jwt::APIClaim {
            user_id: user.id,
            roles: user.roles.clone(),
            username: user.login.clone(),
        };

        let jwt_ttl = self.configuration.get_int_or_default("jwt_ttl", 3600);
        let issuer = self.configuration.get_string("package.name")?;

        let jwt_token = jwt::encode(claim, jwt_ttl, &issuer)?;

        Ok(jwt_token)
    }

    pub fn find_by_id(&self, user_id: i32) -> Result<User, DBRequestResultError> {
        let user_found = self.repository.find_by_id(user_id);

        if user_found.is_err() {
            return Err(DBRequestResultError::NotFound);
        }

        Ok(user_found.unwrap())
    }
}
