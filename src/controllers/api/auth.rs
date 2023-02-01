use crate::{core::validation::Validated, http_exception};
use rocket::{http::Status, serde::json::Json, State};

use crate::{
    core::response::ApiResponse,
    domain::dto::auth::{JWTTokenOutputDTO, LoginInputDTO, RefreshTokenInputDTO},
    exceptions::dto::http_exception::HttpException,
    middlewares::{
        refresh_token_middleware::RefreshTokenMiddleware, user_middleware::UserMiddleware,
    },
};

use crate::middlewares::refresh_token_middleware::JWTRefreshTokenValidationError;

#[post("/token", format = "json", data = "<input>")]
pub fn token(
    input: Validated<Json<LoginInputDTO>>,
    user_middleware: &State<UserMiddleware>,
    refresh_token_middleware: &State<RefreshTokenMiddleware>,
) -> Result<ApiResponse<JWTTokenOutputDTO>, ApiResponse<HttpException>> {
    let real_input = input.into_inner().into_inner();

    let auth_result = user_middleware.authenticate_user_from_input(&real_input);

    if auth_result.is_err() {
        http_exception!(Status::NotFound);
    }

    let current_user = auth_result.unwrap();

    let jwt_token_result = user_middleware.create_jwt_for_user(&current_user);

    if jwt_token_result.is_err() {
        http_exception!(Status::InternalServerError, "Could not create JWT token");
    }

    let refresh_token_result = refresh_token_middleware.generate_for_user(&current_user);

    if refresh_token_result.is_err() {
        http_exception!(
            Status::InternalServerError,
            "Could not create refresh token"
        );
    }

    Ok(ApiResponse::ok(Json(JWTTokenOutputDTO {
        token: jwt_token_result.unwrap(),
        refresh_token: refresh_token_result.unwrap().token,
    })))
}

#[post("/refresh-token", format = "json", data = "<input>")]
pub fn refresh_token(
    input: Validated<Json<RefreshTokenInputDTO>>,
    user_middleware: &State<UserMiddleware>,
    refresh_token_middleware: &State<RefreshTokenMiddleware>,
) -> Result<ApiResponse<JWTTokenOutputDTO>, ApiResponse<HttpException>> {
    let input = input.into_inner().into_inner();

    let token_valid = refresh_token_middleware.is_valid(&input.refresh_token);

    if let Err(token_error) = token_valid {
        match token_error {
            JWTRefreshTokenValidationError::NotFound(_) => {
                http_exception!(Status::NotFound);
            }
            JWTRefreshTokenValidationError::Expired(_) => {
                http_exception!(Status::BadRequest, "Token expired");
            }
        }
    }

    let token = token_valid.unwrap();

    let user_query_result = user_middleware.find_one_by_id(&format!("{}", token.user_id));

    if user_query_result.is_err() {
        http_exception!(Status::InternalServerError);
    }

    let current_user = user_query_result.unwrap();

    if current_user.is_none() {
        http_exception!(Status::NotFound);
    }

    let current_user = current_user.unwrap();

    let jwt_token_result = user_middleware.create_jwt_for_user(&current_user);

    if jwt_token_result.is_err() {
        http_exception!(Status::InternalServerError, "Could not create JWT token");
    }

    let refresh_token_result = refresh_token_middleware.generate_for_user(&current_user);

    if refresh_token_result.is_err() {
        http_exception!(
            Status::InternalServerError,
            "Could not create refresh token"
        );
    }

    Ok(ApiResponse::ok(Json(JWTTokenOutputDTO {
        token: jwt_token_result.unwrap(),
        refresh_token: refresh_token_result.unwrap().token,
    })))
}
