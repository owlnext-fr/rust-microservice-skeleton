use rocket::serde::json::Json;

use crate::core::response::{ApiResponse, ErrorMessage};
use crate::exceptions::dto::http_exception::HttpException;
use rocket::http::Status;
use rocket::Request;

const DEFAULT_ERROR_MESSAGE: &str = "__DEFAULT__";

#[catch(default)]
pub fn default_catcher(status: Status, req: &Request) -> ApiResponse<HttpException> {
    let possible_reason = req.local_cache(|| ErrorMessage {
        message: DEFAULT_ERROR_MESSAGE.into(),
    });

    let mut reason: Option<String> = None;

    if possible_reason.message != DEFAULT_ERROR_MESSAGE {
        reason = Some(possible_reason.message.clone());
    }

    ApiResponse {
        json: Json(HttpException::from_code_with_reason(status.code, reason)),
        status,
    }
}
