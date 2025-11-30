use serde::Deserialize;
use config;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub debug: bool,
    pub storage: String,
    pub log_level: String,
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
    pub fn load(config_file: &str) -> Result<Self, config::ConfigError> {
        let mut s = config::Config::builder()
            .set_default("debug", false)?
            .set_default("storage", "JSON:snippets.json")?
            .set_default("log_level", "info")?
            .add_source(config::File::with_name(config_file).required(false))
            .add_source(config::Environment::with_prefix("CONF"))
            .build()?;

        s.try_deserialize()
    }
}
