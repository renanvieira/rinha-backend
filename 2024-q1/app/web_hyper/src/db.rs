use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DBConfig {
    pub pg: deadpool_postgres::Config,
}

impl DBConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
