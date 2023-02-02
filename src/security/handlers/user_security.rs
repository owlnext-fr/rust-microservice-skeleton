use crate::{
    core::security::{is_admin, is_user, SecurityVoter},
    domain::model::user::User,
};

#[derive(Default)]
pub struct UserSecurityVoter {}

impl<'a> SecurityVoter<'a> for UserSecurityVoter {
    fn supports(&self) -> &'a str {
        "user"
    }

    fn has_access(&self, right: &str, user: &User) -> bool {
        match right {
            "list" => is_user(user),
            "details" => is_user(user),
            "create" => is_admin(user),
            "delete" => is_admin(user),
            _ => false,
        }
    }
}
