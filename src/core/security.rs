use std::collections::HashMap;

use crate::domain::model::user::User;

pub struct Security<T: ?Sized + Send + Sync> {
    voters: HashMap<String, Box<T>>,
}

impl<T> Default for Security<T>
where
    T: SecurityVoter + ?Sized + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Security<T>
where
    T: SecurityVoter + ?Sized + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            voters: HashMap::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Box<T>) -> &mut Self {
        self.voters.insert(handler.supports(), handler);

        self
    }

    pub fn is_granted(user: &User, roles: Vec<&str>) -> bool {
        roles
            .iter()
            .all(|item| user.roles.contains(&item.to_string()))
    }

    pub fn has_access(&self, subject: &str, right: &str, user: &User) -> bool {
        if self.voters.contains_key(subject) {
            return self.voters.get(subject).unwrap().has_access(right, user);
        }

        false
    }
}

pub trait SecurityVoter: Send + Sync {
    /// Gets the "subject" supported by the voter
    fn supports(&self) -> String;
    /// Takes a given right and a user, and checks if the user has_access to the action represented by the right.
    fn has_access(&self, right: &str, user: &User) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestSecurityHandler {}

    impl SecurityVoter for TestSecurityHandler {
        fn supports(&self) -> String {
            "test".to_string()
        }

        fn has_access(&self, right: &str, _user: &User) -> bool {
            if right == "ACCEPT_ACCESS" {
                return true;
            }

            false
        }
    }

    #[derive(Default)]
    struct STestSecurityHandler {}

    impl SecurityVoter for STestSecurityHandler {
        fn supports(&self) -> String {
            "test 2".to_string()
        }

        fn has_access(&self, right: &str, _user: &User) -> bool {
            if right == "ACCEPT_ACCESS" {
                return true;
            }

            false
        }
    }

    #[test]
    fn test_custom_security_handler() {
        let mut security = Security::<dyn SecurityVoter>::new(); // <- line with the error

        security.add_handler(Box::new(TestSecurityHandler::default()));
        security.add_handler(Box::new(STestSecurityHandler::default()));

        let user = User::default();

        assert!(security.has_access("test", "ACCEPT_ACCESS", &user));
    }

    #[test]
    fn test_handler_not_found() {
        let mut security = Security::<dyn SecurityVoter>::new(); // <- line with the error

        security.add_handler(Box::new(TestSecurityHandler::default()));
        security.add_handler(Box::new(STestSecurityHandler::default()));

        let user = User::default();

        assert!(!security.has_access("foo", "bar", &user));
    }
}
