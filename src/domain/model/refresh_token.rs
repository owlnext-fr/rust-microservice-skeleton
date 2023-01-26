use crate::domain::model::user::User;
use crate::domain::schema::*;
use chrono::{DateTime, Utc};
use diesel::AsChangeset;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = refresh_token)]
#[diesel(belongs_to(User))]
pub struct RefreshToken {
    pub id: i32,
    pub token: String,
    pub user_id: i32,
    pub validity_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = refresh_token)]
pub struct NewRefreshToken<'a> {
    pub token: &'a str,
    pub user_id: i32,
    pub validity_date: DateTime<Utc>,
}
