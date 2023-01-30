use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::domain::model::application::Application;

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct ApplicationListItemDTO {}

impl From<&Application> for ApplicationListItemDTO {
    fn from(value: &Application) -> Self {
        Self {}
    }
}

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct ApplicationDetailsDTO {}

impl From<&Application> for ApplicationDetailsDTO {
    fn from(value: &Application) -> Self {
        Self {}
    }
}
