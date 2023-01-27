use rocket::State;

use crate::{
    core::{guards::connected_user::ConnectedUser, response::ApiResponse},
    domain::{dto::account::AccountListDTO, repository::account_repository::AccountRepository},
    exceptions::dto::http_exception::HttpException,
    middlewares::account_middleware::{self, AccountMiddleware},
};

#[get("/account", format = "json")]
pub fn list(
    connected_user: ConnectedUser,
    account_middleware: &State<AccountMiddleware<AccountRepository>>,
) -> Result<ApiResponse<Vec<AccountListDTO>>, ApiResponse<HttpException>> {
    let list = account_middleware.list_account_for_user(&connected_user.user);

    todo!();
}
