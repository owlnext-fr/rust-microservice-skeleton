use crate::core::{guards::connected_user::ConnectedUser, response::ApiResponse};
use ::serde::Serialize;
use rocket::{http::Status, serde::json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DemoDTO {
    pub user_id: i32,
}

#[get("/test-connected")]
pub fn test_connected(connected: ConnectedUser) -> ApiResponse<DemoDTO> {
    ApiResponse {
        json: Json(DemoDTO {
            user_id: connected.user.id,
        }),
        status: Status::Ok,
    }
}
