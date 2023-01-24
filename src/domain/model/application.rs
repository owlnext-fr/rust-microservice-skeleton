use crate::domain::schema::*;
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize, Clone, Default)]
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
