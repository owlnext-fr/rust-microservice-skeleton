use argon2::password_hash::SaltString;
use failure::Fail;

use crate::{
    core::{
        configuration::ConfigState,
        database::DBRequestResultError,
        jwt,
        password::{self, generate_salt, hash},
    },
    domain::{
        dto::auth::LoginInputDTO,
        model::user::{NewUser, User},
        repository::user_repository::UserRepository,
    },
};

#[derive(Debug, Fail)]
pub enum AuthenticationError {
    #[fail(display = "User {} not found.", _0)]
    UserNotFound(String),
    #[fail(display = "Wrong password for user {}", _0)]
    WrongPassword(i32),
}

#[derive(Debug, Fail)]
pub enum JWTAuthenticationError {
    #[fail(display = "Invalid token")]
    InvalidToken,
    #[fail(display = "User {} not found.", _0)]
    UserNotFound(i32),
}

#[derive(Default, Clone)]
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

    pub fn authenticate_user_from_jwt(
        &self,
        jwt_token: &str,
    ) -> Result<User, JWTAuthenticationError> {
        let issuer = self.configuration.get_string("package.name").unwrap();

        let jwt_validation_result = jwt::decode(jwt_token, issuer.as_str());

        if jwt_validation_result.is_err() {
            return Err(JWTAuthenticationError::InvalidToken);
        }

        let jwt_claims = jwt_validation_result.unwrap();

        let user_fetch_result = self.repository.find_by_id(jwt_claims.custom.user_id);

        if user_fetch_result.is_err() {
            return Err(JWTAuthenticationError::UserNotFound(
                jwt_claims.custom.user_id,
            ));
        }

        Ok(user_fetch_result.unwrap())
    }

    pub fn create_jwt_for_user(&self, user: &User) -> anyhow::Result<String> {
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

    pub fn find_by_login(&self, login: &str) -> anyhow::Result<User> {
        let user = self.repository.find_by_login(login)?;

        Ok(user)
    }

    pub fn create(&self, new_user: NewUser) -> anyhow::Result<User> {
        let mut new_user = new_user.clone();

        let maybe_clear_password = new_user.password;
        let new_salt = generate_salt();

        if new_user.salt.is_none() {
            new_user.salt = Some(new_salt.as_str());
        }

        let hashed_password = hash(
            maybe_clear_password,
            SaltString::new(new_user.salt.unwrap()).unwrap(),
        );

        if !maybe_clear_password.starts_with('$') {
            new_user.password = hashed_password.as_str();
        }

        let user = self.repository.insert(new_user)?;

        Ok(user)
    }
}
