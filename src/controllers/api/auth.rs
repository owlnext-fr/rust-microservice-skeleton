use rocket::{http::Status, serde::json::Json, State};
use rocket_validation::Validated;

use crate::{
    core::response::ApiResponse,
    domain::{
        dto::auth::{JWTTokenOutputDTO, LoginInputDTO, RefreshTokenInputDTO},
        repository::{
            refresh_token_repository::RefreshTokenRepository, user_repository::UserRepository,
        },
    },
    exceptions::dto::http_exception::HttpException,
    middlewares::{
        refresh_token_middleware::RefreshTokenMiddleware, user_middleware::UserMiddleware,
    },
};

use crate::middlewares::refresh_token_middleware::JWTRefreshTokenValidationError;

#[post("/token", format = "json", data = "<input>")]
pub fn token(
    input: Validated<Json<LoginInputDTO>>,
    user_middleware: &State<UserMiddleware<UserRepository>>,
    refresh_token_middleware: &State<RefreshTokenMiddleware<RefreshTokenRepository>>,
) -> Result<ApiResponse<JWTTokenOutputDTO>, ApiResponse<HttpException>> {
    let real_input = input.into_inner().into_inner();

    let auth_result = user_middleware.authenticate_user_from_input(&real_input);

    if auth_result.is_err() {
        return Err(ApiResponse::from_status(Status::NotFound));
    }

    let current_user = auth_result.unwrap();

    let jwt_token_result = user_middleware.create_jwt_for_user(&current_user);

    if jwt_token_result.is_err() {
        return Err(ApiResponse::from_status(Status::NotFound));
    }

    let refresh_token_result = refresh_token_middleware.generate_for_user(&current_user);

    if refresh_token_result.is_err() {
        return Err(ApiResponse::from_status(Status::NotFound));
    }

    Ok(ApiResponse::ok(Json(JWTTokenOutputDTO {
        token: jwt_token_result.unwrap(),
        refresh_token: refresh_token_result.unwrap().token,
    })))
}

#[post("/refresh-token", format = "json", data = "<input>")]
pub fn refresh_token(
    input: Validated<Json<RefreshTokenInputDTO>>,
    user_middleware: &State<UserMiddleware<UserRepository>>,
    refresh_token_middleware: &State<RefreshTokenMiddleware<RefreshTokenRepository>>,
) -> Result<ApiResponse<JWTTokenOutputDTO>, ApiResponse<HttpException>> {
    let input = input.into_inner().into_inner();

    let token_valid = refresh_token_middleware.is_valid(&input.refresh_token);

    if let Err(token_error) = token_valid {
        match token_error {
            JWTRefreshTokenValidationError::NotFound(_) => {
                return Err(ApiResponse::from_status(Status::NotFound));
            }
            JWTRefreshTokenValidationError::Expired(_) => {
                return Err(ApiResponse::from_status_with_message(
                    Status::BadRequest,
                    "token expired".into(),
                ));
            }
        }
    }

    let token = token_valid.unwrap();

    let user_query_result = user_middleware.find_by_id(token.user_id);

    if user_query_result.is_err() {
        return Err(ApiResponse::from_status(Status::NotFound));
    }

    let current_user = user_query_result.unwrap();

    let jwt_token_result = user_middleware.create_jwt_for_user(&current_user);

    if jwt_token_result.is_err() {
        return Err(ApiResponse::from_status(Status::NotFound));
    }

    let refresh_token_result = refresh_token_middleware.generate_for_user(&current_user);

    if refresh_token_result.is_err() {
        return Err(ApiResponse::from_status(Status::NotFound));
    }

    Ok(ApiResponse::ok(Json(JWTTokenOutputDTO {
        token: jwt_token_result.unwrap(),
        refresh_token: refresh_token_result.unwrap().token,
    })))
}
