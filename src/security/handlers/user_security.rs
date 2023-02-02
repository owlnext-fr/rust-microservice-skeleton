use std::collections::HashMap;

use anyhow::bail;
use anyhow::Result;

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

    fn has_access(
        &self,
        right: &str,
        user: &User,
        context: Option<HashMap<&str, String>>,
    ) -> Result<bool> {
        match right {
            "list" => Ok(is_user(user)),
            "details" => Ok(is_user(user)),
            "create" => Ok(is_admin(user)),
            "update" => {
                let updated_id = self.get_context(context, "updated_id");

                if updated_id.is_none() {
                    bail!("No \"updated_id\" context provided to security voter.");
                }

                let updated_id = updated_id.unwrap().parse::<i32>().unwrap();

                let same_user_for_standard_user = user.id == updated_id;

                if !is_admin(user) && !same_user_for_standard_user {
                    bail!("You cannot update a user that is not you.");
                }

                Ok(is_admin(user) || same_user_for_standard_user)
            }
            "delete" => Ok(is_admin(user)),
            _ => bail!(
                "No right \"{right}\" found for subject \"{}\"",
                self.supports()
            ),
        }
    }
}
