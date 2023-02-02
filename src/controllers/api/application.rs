use rocket::{http::Status, serde::json::Json, State};

use crate::{
    core::{
        guards::{connected_user::ConnectedUser, pagination::Pagination},
        response::ApiResponse,
        security::{Security, SecurityVoter},
    },
    deny_access_unless_granted,
    domain::dto::application::{ApplicationDetailsDTO, ApplicationListItemDTO},
    exceptions::dto::http_exception::HttpException,
    http_exception, http_ok,
    middlewares::application_middleware::ApplicationMiddleware,
};

#[get("/applications", format = "json")]
pub fn application_list(
    connected_user: ConnectedUser,
    application_middleware: &State<ApplicationMiddleware>,
    security: &State<Security<dyn SecurityVoter>>,
    pagination: Pagination,
) -> Result<ApiResponse<Vec<ApplicationListItemDTO>>, ApiResponse<HttpException>> {
    let user = &connected_user.user;

    deny_access_unless_granted!(security, user, "application", "list");

    let list = application_middleware.find_for_user(user, pagination.page, pagination.per_page);

    if list.is_err() {
        http_exception!(Status::InternalServerError);
    }

    let list = list.unwrap();

    let dto_list = application_middleware.to_list_dto(list);

    http_ok!(dto_list);
}

#[get("/applications/<id>", format = "json")]
pub fn application_details(
    id: String,
    connected_user: ConnectedUser,
    application_middleware: &State<ApplicationMiddleware>,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<ApplicationDetailsDTO>, ApiResponse<HttpException>> {
    let user = &connected_user.user;

    deny_access_unless_granted!(security, user, "application", "details");

    let application = application_middleware.find_one_for_user(&id, user);

    if application.is_err() {
        http_exception!(Status::InternalServerError);
    }

    let application = application.unwrap();

    if application.is_none() {
        http_exception!(Status::NotFound);
    }

    let account = application.unwrap();
    let account_details_dto = application_middleware.to_details_dto(&account);

    http_ok!(account_details_dto);
}
