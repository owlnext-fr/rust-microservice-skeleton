use rocket::serde::json::Json;

use crate::core::response::ApiResponse;
use crate::exceptions::dto::http_exception::HttpException;
use rocket::http::Status;
use rocket::Request;

#[catch(default)]
pub fn default_catcher(status: Status, _request: &Request) -> ApiResponse<HttpException> {
    ApiResponse {
        json: Json(HttpException::from_code(status.code)),
        status,
    }
}
