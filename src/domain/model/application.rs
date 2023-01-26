use crate::domain::schema::*;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::account::Account;

#[derive(
    Debug,
    Queryable,
    Identifiable,
    Serialize,
    Deserialize,
    Clone,
    Default,
    AsChangeset,
    Associations,
)]
#[diesel(belongs_to(Account, foreign_key = account_id))]
#[diesel(table_name = application)]
pub struct Application {
    pub id: i32,
    pub ulid: String,
    pub name: String,
    pub contact_email: String,
    pub account_id: i32,
    pub created_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = application)]
pub struct NewApplication<'a> {
    pub ulid: &'a str,
    pub name: &'a str,
    pub contact_email: &'a str,
    pub account_id: i32,
    pub created_date: DateTime<Utc>,
    pub is_deleted: bool,
}

impl<'a> NewApplication<'a> {
    pub fn new(ulid: &'a str, name: &'a str, contact_email: &'a str, account_id: i32) -> Self {
        Self {
            ulid,
            name,
            contact_email,
            account_id,
            created_date: Utc::now(),
            is_deleted: false,
        }
    }
}
