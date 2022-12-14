use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;

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

#[rocket::async_trait]
impl<'r, T: serde::Serialize> Responder<'r, 'r> for ApiResponse<T> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
