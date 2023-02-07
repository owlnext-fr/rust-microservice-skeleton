use chrono::{Duration, Utc};
use thiserror::Error;

use crate::{
    core::{configuration::ConfigState, password},
    domain::{
        model::{
            refresh_token::{NewRefreshToken, RefreshToken},
            user::User,
        },
        repository::refresh_token_repository::RefreshTokenRepository,
    },
};

/// Error states for refresh token validation.
#[derive(Debug, Error)]
pub enum JWTRefreshTokenValidationError {
    #[error("Token not found : {} ", _0)]
    NotFound(String),
    #[error("Token expired since : {} ", _0)]
    Expired(String),
}

/// RefreshToken middleware.
#[derive(Clone)]
pub struct RefreshTokenMiddleware {
    repository: RefreshTokenRepository,
    config: ConfigState,
}

impl RefreshTokenMiddleware {
    /// constructor.
    pub fn new(repository: RefreshTokenRepository, config: ConfigState) -> Self {
        Self { repository, config }
    }

    /// Generates a valid refresh token for the given user, registering it into the database.
    pub fn generate_for_user(&self, user: &User) -> anyhow::Result<RefreshToken> {
        let token = password::generate_simple_sized(128);
        let refresh_ttl = self.config.get_int_or_default("jwt_refresh_ttl", 86400);
        let validity_date = Utc::now() + Duration::seconds(refresh_ttl);

        let new_refresh_token = NewRefreshToken {
            token: token.as_str(),
            user_id: user.id,
            validity_date,
        };

        let refresh_token = self.repository.insert(new_refresh_token)?;

        Ok(refresh_token)
    }

    /// checks if a given refresh token is a valid one.
    pub fn is_valid(
        &self,
        refresh_token: &str,
    ) -> Result<RefreshToken, JWTRefreshTokenValidationError> {
        let token = self.repository.find_by_token(refresh_token);

        if token.is_err() {
            return Err(JWTRefreshTokenValidationError::NotFound(
                refresh_token.into(),
            ));
        }

        let token = token.unwrap();

        let now_utc = Utc::now();

        if now_utc > token.validity_date {
            return Err(JWTRefreshTokenValidationError::Expired(
                token.validity_date.to_rfc3339(),
            ));
        }

        Ok(token)
    }
}
