use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct JWTTokenOutputDTO {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct LoginInputDTO {
    #[validate(length(min = 1))]
    pub login: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct RefreshTokenInputDTO {
    #[validate(length(equal = 128))]
    pub refresh_token: String,
}
