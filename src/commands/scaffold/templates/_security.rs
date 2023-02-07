use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::{
    core::security::{is_admin, is_user, SecurityVoter},
    domain::model::user::User,
};

#[derive(Default)]
pub struct __DATA_CLASS_STRUCT_NAME__SecurityVoter {}

impl<'a> SecurityVoter<'a> for __DATA_CLASS_STRUCT_NAME__SecurityVoter {
    fn supports(&self) -> &'a str {
        "__MODULE_NAME__"
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
            "update" => Ok(is_admin(user)),
            "delete" => Ok(is_admin(user)),
            _ => bail!(
                "No right \"{right}\" found for subject \"{}\"",
                self.supports()
            ),
        }
    }
}
