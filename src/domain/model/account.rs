use crate::domain::schema::*;
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = account)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub created_date: DateTime<Utc>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}
