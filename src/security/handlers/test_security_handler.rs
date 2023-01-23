use crate::{core::security::SecurityVoter, domain::model::user::User};

#[derive(Default)]
pub struct TestSecurityHandler {}

impl<'a> SecurityVoter<'a> for TestSecurityHandler {
    fn supports(&self) -> &'a str {
        "security_test"
    }

    fn has_access(&self, right: &str, _user: &User) -> bool {
        if right == "test_secured" {
            return true;
        }

        false
    }
}
