use crate::exceptions::dto::http_not_found_exception::HttpNotFoundException;
use rocket::serde::json::Json;

#[catch(404)]
pub fn not_found() -> Json<HttpNotFoundException> {
    Json(HttpNotFoundException::default())
}
