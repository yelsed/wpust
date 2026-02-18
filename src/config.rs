use color_eyre::eyre::{Result, eyre, WrapErr};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(default)]
    pub browser: Option<String>,
    #[serde(default)]
    pub wp_admin_path: Option<String>,
    #[serde(default)]
    pub basic_auth: Vec<BasicAuthRule>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BasicAuthRule {
    pub pattern: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn find_basic_auth(&self, site: &str) -> Option<&BasicAuthRule> {
        self.basic_auth.iter().find(|rule| site.contains(&rule.pattern))
    }
}

pub fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| eyre!("Could not determine config directory"))?;
    Ok(config_dir.join("wpust").join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;

    if !path.exists() {
        return Ok(Config::default());
    }

    let contents = std::fs::read_to_string(&path)
        .wrap_err_with(|| format!("Failed to read config file: {}", path.display()))?;

    let config: Config = toml::from_str(&contents)
        .wrap_err_with(|| format!("Failed to parse config file: {}", path.display()))?;

    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = config_path()?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .wrap_err_with(|| format!("Failed to create config directory: {}", parent.display()))?;
    }

    let contents = toml::to_string_pretty(config)
        .wrap_err("Failed to serialize config")?;

    std::fs::write(&path, contents)
        .wrap_err_with(|| format!("Failed to write config file: {}", path.display()))?;

    Ok(())
}
