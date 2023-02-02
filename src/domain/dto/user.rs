use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{core::password, domain::model::user::User};

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct UserListItemDTO {
    pub id: i32,
    pub email: Option<String>,
    pub login: String,
    pub application_id: i32,
    pub created_date: DateTime<Utc>,
}

impl From<&User> for UserListItemDTO {
    fn from(value: &User) -> Self {
        Self {
            id: value.id,
            email: value.email.clone(),
            login: value.login.clone(),
            application_id: value.application_id,
            created_date: value.created_date,
        }
    }
}

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct UserDetailsDTO {
    pub id: i32,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub login: String,
    pub roles: Vec<String>,
    pub application_id: i32,
    pub created_date: DateTime<Utc>,
}

impl From<&User> for UserDetailsDTO {
    fn from(value: &User) -> Self {
        Self {
            id: value.id,
            email: value.email.clone(),
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            login: value.login.clone(),
            roles: value.roles.clone(),
            application_id: value.application_id,
            created_date: value.created_date,
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct NewUserInputDTO {
    #[validate(length(min = 1, max = 200))]
    pub first_name: String,
    #[validate(length(min = 1, max = 200))]
    pub last_name: String,
    #[validate(email, length(min = 1, max = 180))]
    pub email: String,
    #[validate(length(min = 1, max = 180))]
    pub login: String,
    #[validate(length(min = 1, max = 255), custom = "validate_password")]
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if !password::is_strong(password.into()) {
        return Err(ValidationError::new("Password is not strong enough, it must contain at least 8 char, a capital letter, a number and a special symbol."));
    }

    Ok(())
}
