use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct AccountListDTO {
    pub name: String,
    pub created_date: DateTime<Utc>,
}
