use config::Config;
use rocket::serde::Serialize;

pub fn load() -> Config {
    Config::builder()
        .add_source(config::File::with_name("Cargo.toml"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap()
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PublicConfiguration {
    pub api_name: String,
    pub api_version: String,
}

impl PublicConfiguration {
    pub fn from_config(configuration: &Config) -> Self {
        PublicConfiguration {
            api_name: configuration.get_string("package.name").unwrap(),
            api_version: configuration.get_string("package.version").unwrap(),
        }
    }
}
