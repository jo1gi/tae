use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Config {
    pub rules: Vec<Rule>
}

#[derive(Default, Deserialize)]
pub struct Rule {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
    /// Command to run
    pub command: Option<String>,
    /// Redirect to another url
    pub redirect: Option<String>,
}

/// Load config file on disk
pub fn load_config(config_file: &Path) -> Result<Config, crate::Error> {
    let config_path = if config_file != &PathBuf::from("config.toml") && config_file.exists() {
        config_file.to_path_buf()
    } else {
        dirs::config_dir()
            .ok_or(crate::Error::ConfigNotFound)?
            .join("tae")
            .join(config_file)
    };
    if !config_path.exists() { return Err(crate::Error::ConfigNotFound); }
    let content = std::fs::read_to_string(config_path)
        // TODO Change error?
        .map_err(|_| crate::Error::ParseConfig)?;
    toml::from_str(&content)
        .map_err(|_| crate::Error::ParseConfig)
}
