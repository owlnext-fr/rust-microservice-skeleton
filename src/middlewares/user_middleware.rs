use anyhow::bail;
use argon2::password_hash::SaltString;
use chrono::Utc;
use thiserror::Error;

use crate::{
    core::{
        configuration::ConfigState,
        jwt,
        password::{self, generate_salt, hash},
        security::is_admin,
    },
    domain::{
        dto::{
            auth::LoginInputDTO,
            user::{NewUserInputDTO, UpdateUserInputDTO, UserDetailsDTO, UserListItemDTO},
        },
        model::user::{NewUser, User, ROLE_USER, ROLE_USER_ADMIN},
        repository::user_repository::UserRepository,
    },
};

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("User {} not found.", _0)]
    UserNotFound(String),
    #[error("Wrong password for user {}", _0)]
    WrongPassword(i32),
}

#[derive(Debug, Error)]
pub enum JWTAuthenticationError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("User {} not found.", _0)]
    UserNotFound(i32),
}

#[derive(Clone)]
pub struct UserMiddleware {
    repository: UserRepository,
    configuration: ConfigState,
}

impl UserMiddleware {
    pub fn new(repository: UserRepository, configuration: ConfigState) -> Self {
        Self {
            repository,
            configuration,
        }
    }

    pub fn authenticate_user_from_input(&self, input: &LoginInputDTO) -> anyhow::Result<User> {
        let user_found = self.repository.load_user_by_login(input.login.as_str());

        if let Ok(user) = user_found {
            let is_password_valid = password::compare_hashed(&input.password, &user.password);

            if is_password_valid {
                return Ok(user);
            }

            return Err(AuthenticationError::WrongPassword(user.id).into());
        }

        Err(AuthenticationError::UserNotFound(input.login.clone()).into())
    }

    pub fn authenticate_user_from_jwt(&self, jwt_token: &str) -> anyhow::Result<User> {
        let issuer = self.configuration.get_string("package.name").unwrap();

        let jwt_validation_result = jwt::decode(jwt_token, issuer.as_str());

        if jwt_validation_result.is_err() {
            return Err(JWTAuthenticationError::InvalidToken.into());
        }

        let jwt_claims = jwt_validation_result.unwrap();

        let user_fetch_result = self.repository.find_one_by_id(jwt_claims.custom.user_id)?;

        if user_fetch_result.is_none() {
            return Err(JWTAuthenticationError::UserNotFound(jwt_claims.custom.user_id).into());
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

    pub fn find_one_by_id(&self, user_id: &str) -> anyhow::Result<Option<User>> {
        let user_real_id = user_id.parse::<i32>()?;

        let user_found = self.repository.find_one_by_id(user_real_id)?;

        Ok(user_found)
    }

    pub fn find_one_by_login(&self, login: &str) -> anyhow::Result<Option<User>> {
        let user = self.repository.find_one_by_login(login)?;

        Ok(user)
    }

    pub fn find_for_user(
        &self,
        user: &User,
        page: i32,
        per_page: i32,
    ) -> anyhow::Result<Vec<User>> {
        if is_admin(user) {
            let users =
                self.repository
                    .find_all_for_application_id(user.application_id, page, per_page)?;

            Ok(users)
        } else if page == 1 {
            let itself = user.clone();
            Ok(vec![itself])
        } else {
            Ok(vec![])
        }
    }

    pub fn find_one_for_user(&self, id: &str, user: &User) -> anyhow::Result<Option<User>> {
        let id_parsed = id.parse::<i32>()?;

        if is_admin(user) {
            let user = self
                .repository
                .find_one_for_user_and_application(id_parsed, user.application_id)?;

            Ok(user)
        } else if user.id == id_parsed {
            let cloned = user.clone();
            return Ok(Some(cloned));
        } else {
            Ok(None)
        }
    }

    pub fn promote(&self, user: &User) -> anyhow::Result<User> {
        if user.roles.contains(&ROLE_USER_ADMIN.into()) {
            bail!("User already promoted !");
        }

        let mut user = user.clone();
        user.roles.push(ROLE_USER_ADMIN.into());

        let user = self.update(&user)?;

        Ok(user)
    }

    pub fn demote(&self, user: &User) -> anyhow::Result<User> {
        if !user.roles.contains(&ROLE_USER_ADMIN.into()) {
            bail!("User not promoted !");
        }

        let mut user = user.clone();

        let mut roles = user.roles.clone();
        roles.retain(|elem| elem != ROLE_USER_ADMIN);

        user.roles = roles;

        let user = self.update(&user)?;

        Ok(user)
    }

    pub fn create_from_user_input(
        &self,
        creator: &User,
        dto: NewUserInputDTO,
    ) -> anyhow::Result<User> {
        let salt = generate_salt();

        let new_user = NewUser {
            email: Some(&dto.email),
            first_name: Some(&dto.first_name),
            last_name: Some(&dto.last_name),
            login: &dto.login,
            roles: vec![ROLE_USER],
            password: &dto.password,
            salt: Some(salt.as_str()),
            application_id: creator.application_id,
            created_date: Utc::now(),
            created_by: Some(creator.id),
            deleted_date: None,
            deleted_by: None,
            is_deleted: false,
        };

        self.create(new_user)
    }

    pub fn update_from_user_input(
        &self,
        updater: &User,
        to_update: &User,
        dto: UpdateUserInputDTO,
    ) -> anyhow::Result<User> {
        if updater.application_id != to_update.application_id {
            bail!("Update of a user outside of same application is forbidden.");
        }

        let mut user = to_update.clone();

        user.email = Some(dto.email);
        user.first_name = Some(dto.first_name);
        user.last_name = Some(dto.last_name);

        self.update(&user)
    }

    pub fn create(&self, new_user: NewUser) -> anyhow::Result<User> {
        let mut new_user = new_user.clone();

        if self.find_one_by_login(new_user.login)?.is_some() {
            bail!("A user with login {} already exists !", new_user.login);
        }

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

    pub fn update(&self, user: &User) -> anyhow::Result<User> {
        self.repository.update(user)
    }

    pub fn delete(&self, user_to_delete: &User, deleter: &User) -> anyhow::Result<bool> {
        let mut cloned = user_to_delete.clone();

        if cloned.id == deleter.id {
            bail!("You cannot delete yourself.");
        }

        cloned.deleted_date = Some(Utc::now());
        cloned.deleted_by = Some(deleter.id);
        cloned.is_deleted = true;

        self.update(&cloned)?;

        Ok(true)
    }

    pub fn to_list_dto(&self, users: Vec<User>) -> Vec<UserListItemDTO> {
        let mut list = Vec::<UserListItemDTO>::new();

        for user in users.iter() {
            list.push(UserListItemDTO::from(user));
        }

        list
    }

    pub fn to_details_dto(&self, application: &User) -> UserDetailsDTO {
        UserDetailsDTO::from(application)
    }
}
