use rocket::{http::Status, serde::json::Json, State};

use crate::core::validation::Validated;
use crate::extract_message;
use crate::{
    core::{
        guards::{connected_user::ConnectedUser, pagination::Pagination},
        response::{ApiResponse, NoContentResponse},
        security::{Security, SecurityVoter},
    },
    deny_access_unless_granted,
    domain::dto::__MODULE_NAME__::{
        New__DATA_CLASS_STRUCT_NAME__InputDTO, Update__DATA_CLASS_STRUCT_NAME__InputDTO,
        __DATA_CLASS_STRUCT_NAME__DetailsDTO, __DATA_CLASS_STRUCT_NAME__ListItemDTO,
    },
    exceptions::dto::http_exception::HttpException,
    http_exception, http_no_content, http_ok,
    middlewares::__MODULE_NAME___middleware::__DATA_CLASS_STRUCT_NAME__Middleware,
};

#[get("/__MODULE_NAME__s", format = "json")]
pub fn __MODULE_NAME___list(
    connected_user: ConnectedUser,
    __MODULE_NAME___middleware: &State<__DATA_CLASS_STRUCT_NAME__Middleware>,
    security: &State<Security<dyn SecurityVoter>>,
    pagination: Pagination,
) -> Result<ApiResponse<Vec<__DATA_CLASS_STRUCT_NAME__ListItemDTO>>, ApiResponse<HttpException>> {
    let user = &connected_user.user;

    deny_access_unless_granted!(security, user, "__MODULE_NAME__", "list");

    let list = __MODULE_NAME___middleware.find_for_user(
        user,
        pagination.page.into(),
        pagination.per_page.into(),
    );

    if list.is_err() {
        http_exception!(Status::InternalServerError, &extract_message!(list));
    }

    let list = list.unwrap();

    let dto_list = __MODULE_NAME___middleware.to_list_dto(list);

    http_ok!(dto_list);
}

#[get("/__MODULE_NAME__s/<id>", format = "json")]
pub fn __MODULE_NAME___details(
    id: String,
    connected_user: ConnectedUser,
    __MODULE_NAME___middleware: &State<__DATA_CLASS_STRUCT_NAME__Middleware>,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<__DATA_CLASS_STRUCT_NAME__DetailsDTO>, ApiResponse<HttpException>> {
    let user = &connected_user.user;

    deny_access_unless_granted!(security, user, "__MODULE_NAME___", "details");

    let __MODULE_NAME__ = __MODULE_NAME___middleware.find_one_for_user(&id, user);

    if __MODULE_NAME__.is_err() {
        http_exception!(
            Status::InternalServerError,
            &extract_message!(__MODULE_NAME__)
        );
    }

    let __MODULE_NAME__ = __MODULE_NAME__.unwrap();

    if __MODULE_NAME__.is_none() {
        http_exception!(Status::NotFound);
    }

    let __MODULE_NAME__ = __MODULE_NAME__.unwrap();
    let __MODULE_NAME___details_dto = __MODULE_NAME___middleware.to_details_dto(&__MODULE_NAME__);

    http_ok!(__MODULE_NAME___details_dto);
}

#[post("/__MODULE_NAME__s", format = "json", data = "<input>")]
pub fn __MODULE_NAME___create(
    input: Validated<Json<New__DATA_CLASS_STRUCT_NAME__InputDTO>>,
    connected_user: ConnectedUser,
    __MODULE_NAME___middleware: &State<__DATA_CLASS_STRUCT_NAME__Middleware>,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<__DATA_CLASS_STRUCT_NAME__DetailsDTO>, ApiResponse<HttpException>> {
    let creator = &connected_user.user;

    deny_access_unless_granted!(security, creator, "__MODULE_NAME__", "create");

    let dto = input.into_deep_inner();

    let created___MODULE_NAME__ = __MODULE_NAME___middleware.create_from_user_input(creator, dto);

    if created___MODULE_NAME__.is_err() {
        let error_message = extract_message!(created___MODULE_NAME__);

        if error_message.contains("already exists") {
            http_exception!(Status::BadRequest, &error_message);
        } else {
            http_exception!(Status::InternalServerError, &error_message);
        }
    }

    let created___MODULE_NAME__ = created___MODULE_NAME__.unwrap();

    let output = __MODULE_NAME___middleware.to_details_dto(&created___MODULE_NAME__);

    http_ok!(output);
}

#[put("/__MODULE_NAME__s/<id>", format = "json", data = "<input>")]
pub fn __MODULE_NAME___update(
    id: i32,
    input: Validated<Json<Update__DATA_CLASS_STRUCT_NAME__InputDTO>>,
    connected_user: ConnectedUser,
    __MODULE_NAME___middleware: &State<__DATA_CLASS_STRUCT_NAME__Middleware>,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<__DATA_CLASS_STRUCT_NAME__DetailsDTO>, ApiResponse<HttpException>> {
    let updater = &connected_user.user;
    let updated_id = format!("{id}");

    deny_access_unless_granted!(security, updater, "__MODULE_NAME___", "update");

    let to_update = __MODULE_NAME___middleware.find_one_for_user(&updated_id, updater);

    if to_update.is_err() {
        http_exception!(Status::InternalServerError, &extract_message!(to_update));
    }

    let to_update = to_update.unwrap();

    if to_update.is_none() {
        http_exception!(Status::NotFound, "Cannot find __MODULE_NAME___ to update.");
    }

    let to_update = to_update.unwrap();

    let dto = input.into_deep_inner();

    let updated___MODULE_NAME__ =
        __MODULE_NAME___middleware.update_from_user_input(updater, &to_update, dto);

    if updated___MODULE_NAME__.is_err() {
        let root_cause_message = extract_message!(updated___MODULE_NAME__);

        if root_cause_message.contains("forbidden") || root_cause_message.contains("already exists")
        {
            http_exception!(Status::BadRequest, &root_cause_message);
        } else {
            http_exception!(Status::InternalServerError, &root_cause_message);
        }
    }

    let updated___MODULE_NAME__ = updated___MODULE_NAME__.unwrap();

    let output = __MODULE_NAME___middleware.to_details_dto(&updated___MODULE_NAME__);

    http_ok!(output);
}

#[delete("/__MODULE_NAME__s/<id>", format = "json")]
pub fn __MODULE_NAME___delete(
    id: i32,
    connected_user: ConnectedUser,
    __MODULE_NAME___middleware: &State<__DATA_CLASS_STRUCT_NAME__Middleware>,
    security: &State<Security<dyn SecurityVoter>>,
) -> Result<ApiResponse<NoContentResponse>, ApiResponse<HttpException>> {
    let deleter = &connected_user.user;

    deny_access_unless_granted!(security, deleter, "__MODULE_NAME__", "delete");

    let parsed_id = format!("{id}");

    let __MODULE_NAME___to_delete =
        __MODULE_NAME___middleware.find_one_for_user(&parsed_id, deleter);

    if __MODULE_NAME___to_delete.is_err() {
        http_exception!(
            Status::InternalServerError,
            &extract_message!(__MODULE_NAME___to_delete)
        );
    }

    let __MODULE_NAME___to_delete = __MODULE_NAME___to_delete.unwrap();

    if __MODULE_NAME___to_delete.is_none() {
        http_exception!(Status::NotFound, "Cannot find __MODULE_NAME__ to delete.");
    }

    let __MODULE_NAME___to_delete = __MODULE_NAME___to_delete.unwrap();

    let is_deleted = __MODULE_NAME___middleware.delete(&__MODULE_NAME___to_delete, deleter);

    if is_deleted.is_err() {
        let message = extract_message!(is_deleted);

        if message.contains("yourself") {
            http_exception!(Status::BadRequest, &message);
        }

        http_exception!(Status::InternalServerError, &message);
    }

    http_no_content!()
}
