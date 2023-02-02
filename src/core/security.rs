use crate::domain::model::user::{User, ROLE_USER, ROLE_USER_ADMIN};
use anyhow::{bail, Result};
use std::collections::HashMap;

pub struct Security<'a, T: ?Sized + Send + Sync> {
    voters: HashMap<&'a str, Box<T>>,
}

impl<'a, T> Default for Security<'a, T>
where
    T: SecurityVoter<'a> + ?Sized + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Security<'a, T>
where
    T: SecurityVoter<'a> + ?Sized + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            voters: HashMap::new(),
        }
    }

    pub fn add_voter(&mut self, handler: Box<T>) -> &mut Self {
        self.voters.insert(handler.supports(), handler);

        self
    }

    pub fn is_granted(user: &User, roles: Vec<&str>) -> bool {
        roles
            .iter()
            .all(|item| user.roles.contains(&item.to_string()))
    }

    pub fn has_access(
        &self,
        subject: &str,
        right: &str,
        user: &User,
        context: Option<HashMap<&str, String>>,
    ) -> Result<bool> {
        if let Some(voter) = self.voters.get(subject) {
            return voter.has_access(right, user, context);
        }

        bail!("Cannot find a security handler for {subject}: {right}");
    }
}

pub trait SecurityVoter<'a>: Send + Sync {
    /// Gets the "subject" supported by the voter
    fn supports(&self) -> &'a str;
    /// Takes a given right and a user, and checks if the user has_access to the action represented by the right.
    fn has_access(
        &self,
        right: &str,
        user: &User,
        context: Option<HashMap<&str, String>>,
    ) -> Result<bool>;

    fn get_context(&self, context: Option<HashMap<&str, String>>, key: &str) -> Option<String> {
        let mut result: Option<String> = None;

        if context.is_some() {
            if let Some(unwrapped_context) = context {
                if unwrapped_context.contains_key(key) {
                    result = Some(unwrapped_context.get(key).unwrap().clone());
                }
            }
        }

        result
    }
}

pub fn is_user(user: &User) -> bool {
    user.roles.contains(&ROLE_USER.into())
}
pub fn is_admin(user: &User) -> bool {
    user.roles.contains(&ROLE_USER_ADMIN.into())
}
pub fn is_a(role: &str, user: &User) -> bool {
    user.roles.contains(&role.into())
}
