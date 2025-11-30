use config;
use serde::Deserialize;

/// Global application configuration, loaded from config file or environment.
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// Whether debug mode is enabled.
    pub debug: bool,
    /// Storage backend specification: `JSON:path` or `SQLITE:path`.
    pub storage: String,
    /// Logging level (e.g. "info").
    pub log_level: String,
    /// Optional path to a file where logs are written.
    pub log_path: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            debug: false,
            storage: "JSON:snippets.json".into(),
            log_level: "info".into(),
            log_path: None,
        }
    }
}

impl AppConfig {
    /// Loads configuration from a file and environment variables.
    ///
    /// # Errors
    /// Returns a `ConfigError` if the config file cannot be read or deserialized.
    pub fn load(config_file: &str) -> Result<Self, config::ConfigError> {
        let s = config::Config::builder()
            .set_default("debug", false)?
            .set_default("storage", "JSON:snippets.json")?
            .set_default("log_level", "info")?
            .add_source(config::File::with_name(config_file).required(false))
            .add_source(config::Environment::with_prefix("CONF"))
            .build()?;

        s.try_deserialize()
    }
}
