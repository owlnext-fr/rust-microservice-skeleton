use map_macro::map;
use rocket::{http::Status, serde::json::Json, State};

use crate::core::security::is_admin;
use crate::core::validation::Validated;
use crate::extract_message;
use crate::{
    core::{
        guards::{connected_user::ConnectedUser, pagination::Pagination},
        response::{ApiResponse, NoContentResponse},
        security::{Security, SecurityVoter},
    },
    deny_access_unless_granted,
    domain::dto::user::{NewUserInputDTO, UpdateUserInputDTO, UserDetailsDTO, UserListItemDTO},
    exceptions::dto::http_exception::HttpException,
    http_exception, http_no_content, http_ok,
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
        http_exception!(Status::InternalServerError, &extract_message!(list));
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
        http_exception!(Status::InternalServerError, &extract_message!(user));
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
        let error_message = extract_message!(created_user);

        if error_message.contains("already exists") {
            http_exception!(Status::BadRequest, "A user with this login already exists.");
        } else {
            http_exception!(Status::InternalServerError, &error_message);
        }
    }

    let created_user = created_user.unwrap();

    let output = user_middleware.to_details_dto(&created_user);

    http_ok!(output);
}

#[put("/users/<id>", format = "json", data = "<input>")]
pub fn user_update(
    id: i32,
    input: Validated<Json<UpdateUserInputDTO>>,
    connected_user: ConnectedUser,
    user_middleware: &State<UserMiddleware>,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<UserDetailsDTO>, ApiResponse<HttpException>> {
    let updater = &connected_user.user;
    let updated_id = format!("{id}");

    deny_access_unless_granted!(
        security,
        updater,
        "user",
        "update",
        map! {
          "updated_id" => updated_id.clone(),
        }
    );

    if !is_admin(updater) && id != updater.id {
        http_exception!(
            Status::Forbidden,
            "You cannot update a user other than yourself."
        );
    }

    let to_update = user_middleware.find_one_by_id(&updated_id);

    if to_update.is_err() {
        http_exception!(Status::InternalServerError, &extract_message!(to_update));
    }

    let to_update = to_update.unwrap();

    if to_update.is_none() {
        http_exception!(Status::NotFound, "Cannot find user to update.");
    }

    let to_update = to_update.unwrap();

    let dto = input.into_deep_inner();

    let updated_user = user_middleware.update_from_user_input(updater, &to_update, dto);

    if updated_user.is_err() {
        let root_cause_message = format!("{}", updated_user.err().unwrap().root_cause());

        if root_cause_message.contains("forbidden") || root_cause_message.contains("already exists")
        {
            http_exception!(Status::BadRequest, &root_cause_message);
        } else {
            http_exception!(Status::InternalServerError, &root_cause_message);
        }
    }

    let updated_user = updated_user.unwrap();

    let output = user_middleware.to_details_dto(&updated_user);

    http_ok!(output);
}

#[delete("/users/<id>", format = "json")]
pub fn user_delete(
    id: i32,
    connected_user: ConnectedUser,
    user_middleware: &State<UserMiddleware>,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<NoContentResponse>, ApiResponse<HttpException>> {
    let deleter = &connected_user.user;

    deny_access_unless_granted!(security, deleter, "user", "delete");

    let parsed_id = format!("{id}");

    let user_to_delete = user_middleware.find_one_by_id(&parsed_id);

    if user_to_delete.is_err() {
        http_exception!(
            Status::InternalServerError,
            &extract_message!(user_to_delete)
        );
    }

    let user_to_delete = user_to_delete.unwrap();

    if user_to_delete.is_none() {
        http_exception!(Status::NotFound, "Cannot find user to delete.");
    }

    let user_to_delete = user_to_delete.unwrap();

    let is_deleted = user_middleware.delete(&user_to_delete, deleter);

    if is_deleted.is_err() {
        let message = extract_message!(is_deleted);

        if message.contains("yourself") {
            http_exception!(Status::BadRequest, &message);
        }

        http_exception!(Status::InternalServerError, &message);
    }

    http_no_content!()
}
