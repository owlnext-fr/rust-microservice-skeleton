use rocket::{http::Status, serde::json::Json, State};

use crate::{
    core::{
        guards::{connected_user::ConnectedUser, pagination::Pagination},
        response::ApiResponse,
        security::{Security, SecurityVoter},
    },
    domain::dto::account::{AccountDetailsDTO, AccountListItemDTO},
    exceptions::dto::http_exception::HttpException,
    middlewares::account_middleware::AccountMiddleware,
};

#[get("/accounts", format = "json")]
pub fn account_list(
    connected_user: ConnectedUser,
    account_middleware: &State<AccountMiddleware>,
    pagination: Pagination,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<Vec<AccountListItemDTO>>, ApiResponse<HttpException>> {
    let user = &connected_user.user;

    if !security.has_access("account", "list", user) {
        return Err(ApiResponse::from_status(Status::Unauthorized));
    }

    let list = account_middleware.find_for_user(user, pagination.page, pagination.per_page);

    if list.is_err() {
        return Err(ApiResponse::from_status(Status::InternalServerError));
    }

    let list = list.unwrap();

    let dto_list = account_middleware.to_list_dto(list);

    Ok(ApiResponse::ok(Json(dto_list)))
}

#[get("/accounts/<id>", format = "json")]
pub fn account_details(
    id: String,
    connected_user: ConnectedUser,
    account_middleware: &State<AccountMiddleware>,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<AccountDetailsDTO>, ApiResponse<HttpException>> {
    let user = &connected_user.user;

    if !security.has_access("account", "details", user) {
        return Err(ApiResponse::from_status(Status::Unauthorized));
    }

    let account = account_middleware.find_one_for_user(&id, user);

    if account.is_err() {
        return Err(ApiResponse::from_status(Status::InternalServerError));
    }

    let account = account.unwrap();

    if account.is_none() {
        return Err(ApiResponse::from_status(Status::NotFound));
    }

    let account = account.unwrap();
    let account_details_dto = account_middleware.to_details_dto(&account);

    Ok(ApiResponse::ok(Json(account_details_dto)))
}
