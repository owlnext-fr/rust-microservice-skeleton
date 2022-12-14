use crate::domain::schema::*;
use chrono::{DateTime, FixedOffset, Utc};
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub login: String,
    pub roles: Vec<String>,
    pub password: String,
    pub salt: Option<String>,
    pub created_date: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub deleted_by: Option<i32>,
    pub is_deleted: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub login: String,
    pub roles: Vec<String>,
    pub password: String,
    pub salt: Option<String>,
    pub created_date: DateTime<FixedOffset>,
    pub created_by: Option<i32>,
    pub deleted_date: Option<DateTime<FixedOffset>>,
    pub deleted_by: Option<i32>,
    pub is_deleted: bool,
}
