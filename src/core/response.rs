use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;

use crate::exceptions::dto::http_exception::HttpException;

#[derive(Debug)]
pub struct ApiResponse<T> {
    pub json: Json<T>,
    pub status: Status,
}

impl<T> ApiResponse<T> {
    pub fn ok(output: Json<T>) -> Self {
        ApiResponse {
            json: output,
            status: Status::Ok,
        }
    }

    pub fn custom(output: Json<T>, status: Status) -> Self {
        ApiResponse {
            json: output,
            status,
        }
    }
}

impl ApiResponse<HttpException> {
    pub fn from_status(status: Status) -> Self {
        ApiResponse {
            json: Json(HttpException {
                code: status.code,
                message: status.reason_lossy().to_string(),
            }),
            status,
        }
    }

    pub fn from_status_with_message(status: Status, message: String) -> Self {
        ApiResponse {
            json: Json(HttpException {
                code: status.code,
                message,
            }),
            status,
        }
    }
}

pub struct NoContentResponse;

impl ApiResponse<NoContentResponse> {
    pub fn no_content() -> Self {
        ApiResponse {
            json: Json(NoContentResponse {}),
            status: Status::NoContent,
        }
    }
}

#[rocket::async_trait]
impl<'r, T: serde::Serialize> Responder<'r, 'r> for ApiResponse<T> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        if self.status == Status::NoContent {
            return Response::build()
                .status(self.status)
                .header(ContentType::JSON)
                .ok();
        }

        Response::build_from(self.json.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
