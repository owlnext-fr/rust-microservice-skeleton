use anyhow::Result;
use config::Config;
use rocket::serde::Serialize;

#[derive(Default, Clone, Debug)]
pub struct ConfigState {
    configuration: Config,
}

impl ConfigState {
    pub fn load() -> Self {
        dotenv::from_filename(".env.local").ok();

        let configuration = Config::builder()
            .add_source(config::File::with_name("Cargo.toml"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        Self { configuration }
    }

    pub fn get_string(&self, key: &str) -> Result<String> {
        let value = self.configuration.get_string(key)?;

        Ok(value)
    }

    pub fn get_bool_or_default(&self, key: &str, default: bool) -> bool {
        self.configuration.get_bool(key).unwrap_or(default)
    }

    pub fn get_string_or_default(&self, key: &str, default: &str) -> String {
        self.configuration
            .get_string(key)
            .unwrap_or_else(|_| default.into())
    }

    pub fn get_int_or_default(&self, key: &str, default: i64) -> i64 {
        self.configuration.get_int(key).unwrap_or(default)
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PublicConfiguration {
    pub name: String,
    pub version: String,
}

impl PublicConfiguration {
    pub fn from_config_state(configuration: &ConfigState) -> Self {
        PublicConfiguration {
            name: configuration.get_string("package.name").unwrap(),
            version: configuration.get_string("package.version").unwrap(),
        }
    }
}
