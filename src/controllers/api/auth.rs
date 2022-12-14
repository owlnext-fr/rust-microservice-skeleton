use rocket::{http::Status, serde::json::Json, State};
use rocket_validation::Validated;

use crate::{
    core::{database::DbPoolState, jwt, response::ApiResponse},
    domain::dto::auth::{JWTTokenOutputDTO, LoginInputDTO},
    exceptions::dto::http_exception::HttpException,
    middlewares::{
        traits::db_enabled_middleware_trait::DbEnabledMiddlewareTrait,
        user_middleware::UserMiddleware,
    },
};

#[post("/token", format = "json", data = "<input>")]
pub fn token(
    input: Validated<Json<LoginInputDTO>>,
    db_pool: &State<DbPoolState>,
) -> Result<ApiResponse<JWTTokenOutputDTO>, ApiResponse<HttpException>> {
    let real_input = input.into_inner().into_inner();

    let mut middleware = UserMiddleware::default();
    middleware.setup_db(db_pool.inner().clone());

    let auth_result = middleware.authenticate_user_from_input(&real_input);

    if auth_result.is_err() {
        let status = Status::NotFound;
        return Err(ApiResponse::custom(
            Json(HttpException {
                code: status.code,
                message: "Not found".into(),
            }),
            status,
        ));
    }

    let current_user = auth_result.unwrap();

    let claim = jwt::APIClaim {
        user_id: current_user.id,
        roles: current_user.roles,
        username: current_user.login,
    };

    let jwt_token_result = jwt::encode(claim);

    if jwt_token_result.is_err() {
        let status = Status::NotFound;
        return Err(ApiResponse::custom(
            Json(HttpException {
                code: status.code,
                message: "Not found".into(),
            }),
            status,
        ));
    }

    let jwt_token = jwt_token_result.unwrap();

    Ok(ApiResponse::ok(Json(JWTTokenOutputDTO {
        token: jwt_token,
        refresh_token: "".into(),
    })))
}

//#[post("/refresh-token", format = "json", data = "<input>")]
//pub fn refresh_token(input: Json<LoginInputDTO>) -> ApiResponse<JWTTokenOutputDTO> {
//    token(input)
//}
