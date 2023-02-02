use rocket::{http::Status, serde::json::Json, State};

use crate::core::validation::Validated;
use crate::domain::dto::user::NewUserInputDTO;
use crate::http_ok;
use crate::{
    core::{
        guards::{connected_user::ConnectedUser, pagination::Pagination},
        response::ApiResponse,
        security::{Security, SecurityVoter},
    },
    deny_access_unless_granted,
    domain::dto::user::{UserDetailsDTO, UserListItemDTO},
    exceptions::dto::http_exception::HttpException,
    http_exception,
    middlewares::user_middleware::UserMiddleware,
};

#[get("/users", format = "json")]
pub fn user_list(
    connected_user: ConnectedUser,
    user_middleware: &State<UserMiddleware>,
    security: &State<Security<dyn SecurityVoter>>,
    pagination: Pagination,
) -> Result<ApiResponse<Vec<UserListItemDTO>>, ApiResponse<HttpException>> {
    let user = &connected_user.user;

    deny_access_unless_granted!(security, user, "user", "list");

    let list =
        user_middleware.find_for_user(user, pagination.page.into(), pagination.per_page.into());

    if list.is_err() {
        http_exception!(Status::InternalServerError);
    }

    let list = list.unwrap();

    let dto_list = user_middleware.to_list_dto(list);

    http_ok!(dto_list);
}

#[get("/users/<id>", format = "json")]
pub fn user_details(
    id: String,
    connected_user: ConnectedUser,
    user_middleware: &State<UserMiddleware>,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<UserDetailsDTO>, ApiResponse<HttpException>> {
    let user = &connected_user.user;

    deny_access_unless_granted!(security, user, "user", "details");

    let user = user_middleware.find_one_for_user(&id, user);

    if user.is_err() {
        http_exception!(Status::InternalServerError);
    }

    let user = user.unwrap();

    if user.is_none() {
        http_exception!(Status::NotFound);
    }

    let user = user.unwrap();
    let user_details_dto = user_middleware.to_details_dto(&user);

    http_ok!(user_details_dto);
}

#[post("/users", format = "json", data = "<input>")]
pub fn user_create(
    input: Validated<Json<NewUserInputDTO>>,
    connected_user: ConnectedUser,
    user_middleware: &State<UserMiddleware>,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<UserDetailsDTO>, ApiResponse<HttpException>> {
    let creator = &connected_user.user;

    deny_access_unless_granted!(security, creator, "user", "create");

    let dto = input.into_deep_inner();

    let created_user = user_middleware.create_from_user_input(creator, dto);

    if created_user.is_err() {
        if format!("{}", created_user.err().unwrap().root_cause()).contains("already exists") {
            http_exception!(Status::BadRequest, "A user with this login already exists.");
        } else {
            http_exception!(Status::InternalServerError);
        }
    }

    let created_user = created_user.unwrap();

    let output = user_middleware.to_details_dto(&created_user);

    http_ok!(output);
}
