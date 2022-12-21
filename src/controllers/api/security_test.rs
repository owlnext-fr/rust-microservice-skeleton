use crate::{
    core::{
        guards::connected_user::ConnectedUser,
        response::ApiResponse,
        security::{Security, SecurityVoter},
    },
    exceptions::dto::http_exception::HttpException,
};
use ::serde::Serialize;
use rocket::{http::Status, serde::json::Json, State};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DemoDTO {
    pub user_id: i32,
}

#[get("/test-connected", format = "json")]
pub fn test_connected(connected: ConnectedUser) -> ApiResponse<DemoDTO> {
    ApiResponse {
        json: Json(DemoDTO {
            user_id: connected.user.id,
        }),
        status: Status::Ok,
    }
}

#[get("/test-secured", format = "json")]
pub fn test_secured(
    connected: ConnectedUser,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<DemoDTO>, ApiResponse<HttpException>> {
    if security.has_access("security_test", "test_secured", &connected.user) {
        return Ok(ApiResponse {
            json: Json(DemoDTO {
                user_id: connected.user.id,
            }),
            status: Status::Ok,
        });
    }

    Err(ApiResponse::from_status(Status::Forbidden))
}
