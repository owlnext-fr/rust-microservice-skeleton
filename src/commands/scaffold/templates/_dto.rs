use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::model::__MODULE_NAME__::__DATA_CLASS_STRUCT_NAME__;

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct __DATA_CLASS_STRUCT_NAME__ListItemDTO {
    pub id: i32,
    // ...
    pub created_date: DateTime<Utc>,
}

impl From<&__DATA_CLASS_STRUCT_NAME__> for __DATA_CLASS_STRUCT_NAME__ListItemDTO {
    fn from(value: &__DATA_CLASS_STRUCT_NAME__) -> Self {
        Self {
            id: value.id,
            // ...
            created_date: value.created_date,
        }
    }
}

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct __DATA_CLASS_STRUCT_NAME__DetailsDTO {
    pub id: i32,
    // ...
    pub created_date: DateTime<Utc>,
}

impl From<&__DATA_CLASS_STRUCT_NAME__> for __DATA_CLASS_STRUCT_NAME__DetailsDTO {
    fn from(value: &__DATA_CLASS_STRUCT_NAME__) -> Self {
        Self {
            id: value.id,
            // ...
            created_date: value.created_date,
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct New__DATA_CLASS_STRUCT_NAME__InputDTO {
    // ...
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct Update__DATA_CLASS_STRUCT_NAME__InputDTO {
    // ...
}
