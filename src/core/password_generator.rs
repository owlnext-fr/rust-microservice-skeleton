use passwords::PasswordGenerator;

pub fn get() -> PasswordGenerator {
    get_sized(24)
}

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
