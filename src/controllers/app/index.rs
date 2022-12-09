use crate::base::configuration::PublicConfiguration;
use config::Config;
use rocket::{serde::json::Json, State};

#[get("/")]
pub fn index(config: &State<Config>) -> Json<PublicConfiguration> {
    Json(PublicConfiguration::from_config(config))
}
