use crate::{
    core::security::{is_user, SecurityVoter},
    domain::model::user::User,
};
use anyhow::bail;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Default)]
pub struct ApplicationSecurityVoter {}

impl<'a> SecurityVoter<'a> for ApplicationSecurityVoter {
    fn supports(&self) -> &'a str {
        "application"
    }

    fn has_access(
        &self,
        right: &str,
        user: &User,
        _context: Option<HashMap<&str, String>>,
    ) -> Result<bool> {
        match right {
            "list" => Ok(is_user(user)),
            "details" => Ok(is_user(user)),
            _ => bail!(
                "No right \"{right}\" found for subject \"{}\"",
                self.supports()
            ),
        }
    }
}
