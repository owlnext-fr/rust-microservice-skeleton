use crate::domain::schema::*;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Queryable,
    Identifiable,
    Serialize,
    Deserialize,
    Clone,
    Default,
    AsChangeset,
    QueryableByName,
)]
#[diesel(table_name = account)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub created_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub created_date: DateTime<Utc>,
    pub is_deleted: bool,
}

impl<'a> NewAccount<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            created_date: Utc::now(),
            is_deleted: false,
        }
    }
}
