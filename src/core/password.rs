use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

use super::password_generator;

use fancy_regex::Regex;

/// Generates a strong password. This 24 char password will have upper and lower cased chars, numbers and symbols.
pub fn generate() -> String {
    password_generator::get().generate_one().unwrap()
}

/// Generates a strong password. This `size` char password will have upper and lower cased chars, numbers and symbols.
pub fn generate_sized(size: usize) -> String {
    password_generator::get_sized(size).generate_one().unwrap()
}

/// Generates an unsafe password. This `size` char password will only have upper and lower cased chars and numbers.
pub fn generate_simple_sized(size: usize) -> String {
    password_generator::get_simple_sized(size)
        .generate_one()
        .unwrap()
}

/// Generates a salt "string" struct for password hashing.
pub fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

/// hashes a password.
pub fn hash(clear_password: &str, salt: SaltString) -> String {
    Argon2::default()
        .hash_password(clear_password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

/// Compares a clean password (from user input) to a hashed password.
pub fn compare_hashed(clear_password: &str, hashed_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed_password).unwrap();

    Argon2::default()
        .verify_password(clear_password.as_bytes(), &parsed_hash)
        .is_ok()
}

/// Validates that a password is a "strong" password.
pub fn is_strong(password: String) -> bool {
    let re =
        Regex::new(r"(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*#?&_-])[A-Za-z\d@$!%*#?&_-]{8,50}")
            .unwrap();

    re.is_match(&password).unwrap_or(false)
}
