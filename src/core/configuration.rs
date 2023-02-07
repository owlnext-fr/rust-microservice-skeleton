use anyhow::Result;
use config::Config;
use rocket::serde::Serialize;

/// A struct representing a shared configuration state.
/// Note that this state is immutable after launching the rocket.
#[derive(Default, Clone, Debug)]
pub struct ConfigState {
    /// configuration handler
    configuration: Config,
}

impl ConfigState {
    /// function that loads the configuration from multiple sources :
    ///
    /// - from environment variables either given by current execution or a `.env.local` file.
    pub fn load() -> Self {
        // if any env file is provided, load it into the environment context
        dotenv::from_filename(".env.local").ok();

        // actually loads all environment variables starting with APP
        let configuration = Config::builder()
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        Self { configuration }
    }

    /// gets a string from configuration as a Result<>
    pub fn get_string(&self, key: &str) -> Result<String> {
        let value = self.configuration.get_string(&key.to_lowercase())?;

        Ok(value)
    }

    /// gets a string value from the configuration or default value
    pub fn get_string_or_default(&self, key: &str, default: &str) -> String {
        self.configuration
            .get_string(&key.to_lowercase())
            .unwrap_or_else(|_| default.into())
    }

    /// gets a boolean value from config or default value
    pub fn get_bool_or_default(&self, key: &str, default: bool) -> bool {
        self.configuration
            .get_bool(&key.to_lowercase())
            .unwrap_or(default)
    }

    /// gets an integer (e.g. i64) value from configuration or a default one.
    pub fn get_int_or_default(&self, key: &str, default: i64) -> i64 {
        self.configuration
            .get_int(&key.to_lowercase())
            .unwrap_or(default)
    }
}

/// A struct representing public configuration, to display on the root (e.g. `GET /`) of the API.
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PublicConfiguration {
    pub name: String,
    pub version: String,
}

impl PublicConfiguration {
    /// generates a PublicConfiguration representation from the current ConfigState of the API.
    pub fn from_config_state(configuration: &ConfigState) -> Self {
        PublicConfiguration {
            name: configuration.get_string("PACKAGE_NAME").unwrap(),
            version: configuration.get_string("PACKAGE_VERSION").unwrap(),
        }
    }
}
