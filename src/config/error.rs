use thiserror::Error;

/// Represents errors that can occur during configuration operations
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Error that occurs when reading the configuration file fails
    #[error("Failed to read config file: {0}")]
    FileReadError(#[from] std::io::Error),

    /// Error that occurs when parsing the configuration file fails
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] serde_yaml::Error),

    /// Error that occurs when a required environment variable is not set
    #[error("Environment variable not set: {0}")]
    EnvVarError(String),

    /// Error that occurs when the configuration file path is invalid
    #[error("Invalid config path: {0}")]
    InvalidPath(String),
}
