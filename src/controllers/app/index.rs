use crate::core::{configuration::PublicConfiguration, response::ApiResponse};
use config::Config;
use rocket::{http::Status, serde::json::Json, State};

#[get("/")]
pub fn index(config: &State<Config>) -> ApiResponse<PublicConfiguration> {
    let content = Json(PublicConfiguration::from_config(config));

    ApiResponse {
        json: content,
        status: Status::Ok,
    }
}
