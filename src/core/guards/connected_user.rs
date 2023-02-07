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

/// Connected user guard to get the connected user over controllers.
#[derive(Debug, Clone)]
pub struct ConnectedUser {
    /// the currently connected user.
    pub user: User,
}

/// Every error state that could happen during user authentication & validation.
#[derive(Debug, Error)]
pub enum AuthenticationError {
    /// No JWT header found in the request.
    #[error("JWT header not found")]
    HeaderNotFound,
    /// The JWT header is invalid.
    #[error("Invalid JWT header")]
    InvalidHeader,
    /// The JWT header is unparsable.
    #[error("Invalid JWT header")]
    MalformedHeader,
    /// The JWT token is not formatted properly, or outdated.
    #[error("Invalid JWT token, perhaps malformatted or outdated")]
    InvalidJWT,
    /// The user contained in the JWT token is invalid.
    #[error("JWT token user not found")]
    UserNotFound,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConnectedUser {
    /// error type returned in case of authentication error.
    type Error = AuthenticationError;

    /// Guard interceptor extracting connected user from the request.
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
