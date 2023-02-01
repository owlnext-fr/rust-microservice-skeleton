use std::collections::HashMap;

use crate::domain::model::user::{User, ROLE_USER, ROLE_USER_ADMIN};

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

    pub fn has_access(&self, subject: &str, right: &str, user: &User) -> bool {
        if let Some(voter) = self.voters.get(subject) {
            return voter.has_access(right, user);
        }

        false
    }
}

pub trait SecurityVoter<'a>: Send + Sync {
    /// Gets the "subject" supported by the voter
    fn supports(&self) -> &'a str;
    /// Takes a given right and a user, and checks if the user has_access to the action represented by the right.
    fn has_access(&self, right: &str, user: &User) -> bool;
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
