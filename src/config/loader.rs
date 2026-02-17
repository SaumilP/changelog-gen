use crate::config::schema::Config;
use anyhow::Result;
use std::fs;

pub fn load_config() -> Result<Config> {
    if std::path::Path::new("changelog.toml").exists() {
        let content = fs::read_to_string("changelog.toml")?;
        return Ok(toml::from_str(&content)?);
    }

    if std::path::Path::new("changelog.yaml").exists() {
        let content = fs::read_to_string("changelog.yaml")?;
        return Ok(serde_yaml::from_str(&content)?);
    }

    Ok(Config::default())
}
