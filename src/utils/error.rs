use thiserror::Error;

/// Custom error types for utility operations
#[derive(Error, Debug)]
pub enum UtilsError {
    /// Error that occurred during logging operations
    #[error("Logging error: {0}")]
    LoggingError(String),

    /// Error that occurred during I/O operations
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
