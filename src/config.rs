use color_eyre::eyre::{Result, eyre, WrapErr};
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub basic_auth: Vec<BasicAuthRule>,
}

#[derive(Deserialize)]
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

pub fn load_config() -> Result<Config> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| eyre!("Could not determine config directory"))?;
    let config_path = config_dir.join("wpust").join("config.toml");

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let contents = std::fs::read_to_string(&config_path)
        .wrap_err_with(|| format!("Failed to read config file: {}", config_path.display()))?;

    let config: Config = toml::from_str(&contents)
        .wrap_err_with(|| format!("Failed to parse config file: {}", config_path.display()))?;

    Ok(config)
}
