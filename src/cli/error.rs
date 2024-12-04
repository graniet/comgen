use thiserror::Error;

/// Represents errors that can occur during CLI operations
#[derive(Error, Debug)]
pub enum CliError {
    /// Error that occurs during IO operations
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error that occurs when user input is invalid
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
