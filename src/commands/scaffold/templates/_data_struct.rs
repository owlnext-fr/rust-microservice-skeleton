use crate::domain::schema::*;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize, Clone, Default, AsChangeset)]
#[diesel(table_name = __MODULE_NAME_PLURAL__)]
pub struct __DATA_CLASS_STRUCT_NAME__ {
    pub id: i32,
    // ...
    pub application_id: i32,
    pub created_date: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub deleted_by: Option<i32>,
    pub is_deleted: bool,
}

#[derive(Debug, Deserialize, Insertable, Clone)]
#[diesel(table_name = __MODULE_NAME_PLURAL__)]
pub struct __NEW_DATA_CLASS_STRUCT_NAME__ {
    // ...
    pub application_id: i32,
    pub created_date: DateTime<Utc>,
    pub created_by: Option<i32>,
    pub deleted_date: Option<DateTime<Utc>>,
    pub deleted_by: Option<i32>,
    pub is_deleted: bool,
}
