use config::Config;
use rocket::serde::Serialize;

pub fn load() -> Config {
    dotenv::from_filename(".env.local").ok();

    Config::builder()
        .add_source(config::File::with_name("Cargo.toml"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap()
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PublicConfiguration {
    pub name: String,
    pub version: String,
}

impl PublicConfiguration {
    pub fn from_config(configuration: &Config) -> Self {
        PublicConfiguration {
            name: configuration.get_string("package.name").unwrap(),
            version: configuration.get_string("package.version").unwrap(),
        }
    }
}
