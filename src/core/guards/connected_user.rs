use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::{
    domain::{model::user::User, repository::user_repository::UserRepository},
    middlewares::user_middleware::{JWTAuthenticationError, UserMiddleware},
};

#[derive(Debug, Clone)]
pub struct ConnectedUser {
    pub user: User,
}

#[derive(Debug)]
pub enum AuthenticationError {
    HeaderNotFound,
    InvalidHeader,
    MalformedHeader,
    InvalidJWT,
    UserNotFound,
    NotEnoughPrivileges(String),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConnectedUser {
    type Error = AuthenticationError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_middleware = req
            .rocket()
            .state::<UserMiddleware<UserRepository>>()
            .unwrap();

        let jwt_header = req.headers().get("Authorization").next();

        if jwt_header.is_none() {
            return Outcome::Failure((Status::Unauthorized, AuthenticationError::HeaderNotFound));
        }

        let jwt_token_unparsed = jwt_header.unwrap();

        if !jwt_token_unparsed.to_lowercase().contains("bearer ") {
            return Outcome::Failure((Status::BadRequest, AuthenticationError::InvalidHeader));
        }

        let jwt_token_parsed = jwt_token_unparsed.split(' ').collect::<Vec<&str>>();

        if jwt_token_parsed.len() != 2 {
            return Outcome::Failure((Status::BadRequest, AuthenticationError::MalformedHeader));
        }

        let jwt_token = *jwt_token_parsed.get(1).unwrap();

        let authenticated_user_result =
            req.local_cache(|| user_middleware.authenticate_user_from_jwt(jwt_token));

        match authenticated_user_result {
            Ok(user) => {
                return Outcome::Success(ConnectedUser { user: user.clone() });
            }
            Err(error) => match error {
                JWTAuthenticationError::InvalidToken => {
                    return Outcome::Failure((
                        Status::Unauthorized,
                        AuthenticationError::InvalidJWT,
                    ));
                }
                JWTAuthenticationError::UserNotFound(_) => {
                    return Outcome::Failure((Status::NotFound, AuthenticationError::UserNotFound));
                }
            },
        }
    }
}
