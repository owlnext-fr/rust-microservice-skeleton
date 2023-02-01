use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use thiserror::Error;

use crate::{
    core::response::ErrorMessage,
    domain::model::user::User,
    middlewares::user_middleware::{JWTAuthenticationError, UserMiddleware},
};

#[derive(Debug, Clone)]
pub struct ConnectedUser {
    pub user: User,
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("JWT header not found")]
    HeaderNotFound,
    #[error("Invalid JWT header")]
    InvalidHeader,
    #[error("Invalid JWT header")]
    MalformedHeader,
    #[error("Invalid JWT token, perhaps malformatted or outdated")]
    InvalidJWT,
    #[error("JWT token user not found")]
    UserNotFound,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConnectedUser {
    type Error = AuthenticationError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_middleware = req.rocket().state::<UserMiddleware>().unwrap();

        let jwt_header = req.headers().get("Authorization").next();

        if jwt_header.is_none() {
            req.local_cache(|| ErrorMessage {
                message: "JWT header not found".into(),
            });
            return Outcome::Failure((Status::Unauthorized, AuthenticationError::HeaderNotFound));
        }

        let jwt_token_unparsed = jwt_header.unwrap();

        if !jwt_token_unparsed.to_lowercase().contains("bearer ") {
            req.local_cache(|| ErrorMessage {
                message: "Invalid JWT header".into(),
            });
            return Outcome::Failure((Status::BadRequest, AuthenticationError::InvalidHeader));
        }

        let jwt_token_parsed = jwt_token_unparsed.split(' ').collect::<Vec<&str>>();

        if jwt_token_parsed.len() != 2 {
            req.local_cache(|| ErrorMessage {
                message: "Malformed JWT header".into(),
            });
            return Outcome::Failure((Status::BadRequest, AuthenticationError::MalformedHeader));
        }

        let jwt_token = *jwt_token_parsed.get(1).unwrap();

        let authenticated_user_result =
            req.local_cache(|| user_middleware.authenticate_user_from_jwt(jwt_token));

        match authenticated_user_result {
            Ok(user) => {
                return Outcome::Success(ConnectedUser { user: user.clone() });
            }
            Err(error) => match error.downcast_ref::<JWTAuthenticationError>().unwrap() {
                JWTAuthenticationError::InvalidToken => {
                    req.local_cache(|| ErrorMessage {
                        message: "Invalid JWT token, perhaps malformatted or outdated".into(),
                    });
                    return Outcome::Failure((
                        Status::Unauthorized,
                        AuthenticationError::InvalidJWT,
                    ));
                }
                JWTAuthenticationError::UserNotFound(_) => {
                    req.local_cache(|| ErrorMessage {
                        message: "Invalid JWT token".into(),
                    });
                    return Outcome::Failure((Status::NotFound, AuthenticationError::UserNotFound));
                }
            },
        }
    }
}
