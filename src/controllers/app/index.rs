use crate::core::{
    configuration::{ConfigState, PublicConfiguration},
    response::ApiResponse,
};
use rocket::{http::Status, serde::json::Json, State};

#[get("/")]
pub fn index(config: &State<ConfigState>) -> ApiResponse<PublicConfiguration> {
    let content = Json(PublicConfiguration::from_config_state(config));

    ApiResponse {
        json: content,
        status: Status::Ok,
    }
}
