use crate::domain::model::user::{User, ROLE_USER, ROLE_USER_ADMIN};
use anyhow::{bail, Result};
use std::collections::HashMap;

/// Security policy registry.
///
/// This struct will store any SecurityVoter to handle security testing and validation against requests and connected users.
pub struct Security<'a, T: ?Sized + Send + Sync> {
    voters: HashMap<&'a str, Box<T>>,
}

impl<'a, T> Default for Security<'a, T>
where
    T: SecurityVoter<'a> + ?Sized + Send + Sync,
{
    /// Creates an "empty" Security policy registry
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Security<'a, T>
where
    T: SecurityVoter<'a> + ?Sized + Send + Sync,
{
    /// Creates an new "empty" Security policy registry
    pub fn new() -> Self {
        Self {
            voters: HashMap::new(),
        }
    }

    /// Adds a policy (e.g. a SecurityVoter) to the security registry.
    ///
    /// The handler must be a boxed `SecurityVoter<'a> + ?Sized + Send + Sync`.
    pub fn add_voter(&mut self, handler: Box<T>) -> &mut Self {
        self.voters.insert(handler.supports(), handler);

        self
    }

    /// shorthand method to check if a user have all the roles in the `roles` vector.
    ///
    /// It can be used :
    /// ```rust
    /// security::is_granted(&user, vec!['ROLE_ADMINISTRATOR']);
    /// ```
    /// or :
    /// ```rust
    /// security::is_granted(&user, vec!['ROLE_A', 'ROLE_B', ...]);
    /// ```
    pub fn is_granted(user: &User, roles: Vec<&str>) -> bool {
        roles
            .iter()
            .all(|item| user.roles.contains(&item.to_string()))
    }

    /// Shorthand method to check if a given user has access to a particular right on a subject. It can optionally use a context to specify rights.
    ///
    /// It can be used :
    /// ```rust
    /// security::has_access("books", "list", &user);
    /// ```
    /// or :
    /// ```rust
    /// security::has_access("books", "borrow", &user, map!{"book_id" => String::from(book.id)});
    /// ```
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

/// trait contract to specify security policies (e.g. Security voters) behaviour
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

    /// Gets a given value in a `context` specified in the `has_access` method.
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

/// shorthand method to check if a user has the role `ROLE_USER`.
pub fn is_user(user: &User) -> bool {
    user.roles.contains(&ROLE_USER.into())
}

/// shorthand method to check if a user has the role `ROLE_USER_ADMIN`.
pub fn is_admin(user: &User) -> bool {
    user.roles.contains(&ROLE_USER_ADMIN.into())
}

/// shorthand method to check if a user has the specified role.
pub fn is_a(role: &str, user: &User) -> bool {
    user.roles.contains(&role.into())
}
