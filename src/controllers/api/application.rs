use rocket::{http::Status, serde::json::Json, State};

use crate::{
    core::{guards::connected_user::ConnectedUser, response::ApiResponse},
    domain::dto::application::{ApplicationDetailsDTO, ApplicationListItemDTO},
    exceptions::dto::http_exception::HttpException,
    middlewares::application_middleware::ApplicationMiddleware,
};

#[get("/applications", format = "json")]
pub fn application_list(
    connected_user: ConnectedUser,
    application_middleware: &State<ApplicationMiddleware>,
) -> Result<ApiResponse<Vec<ApplicationListItemDTO>>, ApiResponse<HttpException>> {
    todo!();
}

#[get("/applications/<id>", format = "json")]
pub fn application_details(
    id: String,
    connected_user: ConnectedUser,
    application_middleware: &State<ApplicationMiddleware>,
) -> Result<ApiResponse<ApplicationDetailsDTO>, ApiResponse<HttpException>> {
    todo!();
}
