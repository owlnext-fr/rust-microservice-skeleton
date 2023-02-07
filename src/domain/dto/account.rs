use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::model::account::Account;

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct AccountListItemDTO {
    pub id: i32,
    pub name: String,
    pub created_date: DateTime<Utc>,
}

impl From<&Account> for AccountListItemDTO {
    fn from(value: &Account) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            created_date: value.created_date,
        }
    }
}

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct AccountDetailsDTO {
    pub id: i32,
    pub name: String,
    pub created_date: DateTime<Utc>,
}

impl From<&Account> for AccountDetailsDTO {
    fn from(value: &Account) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            created_date: value.created_date,
        }
    }
}
