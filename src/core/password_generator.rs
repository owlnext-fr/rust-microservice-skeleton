use passwords::PasswordGenerator;

/// Shorthand method to generate a "safe" 24 char password generator
pub fn get() -> PasswordGenerator {
    get_sized(24)
}

/// Generates a strong password generator with the size given. The password will inclue lower and upper cased chars, numbers and symbols.
///
/// **Note:** you should only generate passords with size >= 10.
pub fn get_sized(size: usize) -> PasswordGenerator {
    PasswordGenerator {
        length: size,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    }
}

/// unsafe method to generate a password generator, as it is not "strong" certified. You may use `password_generator::get_sized` or `password_generator::get` instead.
pub fn get_simple_sized(size: usize) -> PasswordGenerator {
    PasswordGenerator {
        length: size,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    }
}
