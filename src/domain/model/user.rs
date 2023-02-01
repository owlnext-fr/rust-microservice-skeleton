use crate::domain::schema::*;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::application::Application;

pub const ROLE_USER: &str = "ROLE_USER";
pub const ROLE_USER_ADMIN: &str = "ROLE_USER_ADMIN";

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
#[diesel(belongs_to(Application, foreign_key = application_id))]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub login: String,
    pub roles: Vec<String>,
    pub password: String,
    pub salt: Option<String>,
    pub application_id: i32,
    pub created_date: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub deleted_by: Option<i32>,
    pub is_deleted: bool,
}

#[derive(Debug, Deserialize, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: Option<&'a str>,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub login: &'a str,
    pub roles: Vec<&'a str>,
    pub password: &'a str,
    pub salt: Option<&'a str>,
    pub application_id: i32,
    pub created_date: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub deleted_by: Option<i32>,
    pub is_deleted: bool,
}
