use std::path::PathBuf;
use tracing::info;

use super::error::ConfigError;
use super::model::Config;

/// Configuration loader that handles reading and parsing config files
pub struct ConfigLoader {
    /// Path to the configuration file
    config_path: PathBuf,
}

impl ConfigLoader {
    /// Creates a new ConfigLoader instance
    ///
    /// # Arguments
    /// * `config_path` - Path to the configuration file
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path }
    }

    /// Expands the configuration path, resolving home directory references
    ///
    /// # Returns
    /// * `Ok(PathBuf)` - The expanded absolute path
    /// * `Err(ConfigError)` - If environment variables are missing
    pub fn expand_path(&self) -> Result<PathBuf, ConfigError> {
        if self.config_path.starts_with("~/") || self.config_path.starts_with("~\\") {
            let home = if cfg!(windows) {
                std::env::var("USERPROFILE")
                    .map_err(|_| ConfigError::EnvVarError("USERPROFILE".to_string()))?
            } else {
                std::env::var("HOME").map_err(|_| ConfigError::EnvVarError("HOME".to_string()))?
            };

            Ok(PathBuf::from(home).join(&self.config_path.to_str().unwrap()[2..]))
        } else {
            Ok(self.config_path.clone())
        }
    }

    /// Loads and parses the configuration file
    ///
    /// # Returns
    /// * `Ok(Config)` - The parsed configuration
    /// * `Err(ConfigError)` - If reading or parsing fails
    pub fn load(&self) -> Result<Config, ConfigError> {
        let expanded_path = self.expand_path()?;
        info!("Loading config from: {:?}", expanded_path);

        let config_str =
            std::fs::read_to_string(&expanded_path).map_err(ConfigError::FileReadError)?;

        let config: Config = serde_yaml::from_str(&config_str).map_err(ConfigError::ParseError)?;

        if let Err(msg) = config.validate() {
            return Err(ConfigError::InvalidPath(msg));
        }

        Ok(config)
    }
}

/// Helper function to load configuration from a path
///
/// # Arguments
/// * `config_path` - String path to the configuration file
///
/// # Returns
/// * `Ok(Config)` - The parsed configuration
/// * `Err(ConfigError)` - If loading or parsing fails
pub fn load_config(config_path: &str) -> Result<Config, ConfigError> {
    let loader = ConfigLoader::new(PathBuf::from(config_path));
    loader.load()
}
