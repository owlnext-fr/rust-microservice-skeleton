use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::model::application::Application;

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct ApplicationListItemDTO {
    pub id: i32,
    pub name: String,
    pub created_date: DateTime<Utc>,
}

impl From<&Application> for ApplicationListItemDTO {
    fn from(value: &Application) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            created_date: value.created_date,
        }
    }
}

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct ApplicationDetailsDTO {
    pub id: i32,
    pub ulid: String,
    pub name: String,
    pub contact_email: String,
    pub created_date: DateTime<Utc>,
    pub account_id: i32,
}

impl From<&Application> for ApplicationDetailsDTO {
    fn from(value: &Application) -> Self {
        Self {
            id: value.id,
            ulid: value.ulid.clone(),
            name: value.name.clone(),
            contact_email: value.contact_email.clone(),
            created_date: value.created_date,
            account_id: value.account_id,
        }
    }
}
