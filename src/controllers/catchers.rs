use rocket::serde::json::Json;

use crate::exceptions::dto::http_exception::HttpException;
use rocket::http::Status;
use rocket::Request;

#[catch(default)]
pub fn default_catcher(status: Status, _request: &Request) -> Json<HttpException> {
    Json(HttpException::from_code(status.code))
}
