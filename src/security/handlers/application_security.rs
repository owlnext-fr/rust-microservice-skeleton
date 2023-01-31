use crate::{
    core::security::{is_user, SecurityVoter},
    domain::model::user::User,
};

#[derive(Default)]
pub struct ApplicationSecurityVoter {}

impl<'a> SecurityVoter<'a> for ApplicationSecurityVoter {
    fn supports(&self) -> &'a str {
        "application"
    }

    fn has_access(&self, right: &str, user: &User) -> bool {
        match right {
            "list" => is_user(user),
            "details" => is_user(user),
            _ => false,
        }
    }
}
