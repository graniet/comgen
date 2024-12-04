use thiserror::Error;

/// Represents errors that can occur during Git operations
#[derive(Error, Debug)]
pub enum GitError {
    /// Error that occurs during git diff command execution
    #[error("Git diff error: {0}")]
    DiffError(String),

    /// Error that occurs when staging files fails
    #[error("Git stage error: {0}")]
    StageError(String),

    /// Error that occurs during commit operation
    #[error("Git commit error: {0}")]
    CommitError(String),

    /// Error that occurs when pushing changes fails
    #[error("Git push error: {0}")]
    PushError(String),

    /// Underlying IO error from std::io
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error converting Git command output to UTF-8
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}
