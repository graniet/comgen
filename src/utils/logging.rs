use super::error::UtilsError;
use std::path::PathBuf;
use tracing_appender::rolling::RollingFileAppender;

/// Logger utility for setting up application logging
pub struct Logger;

impl Logger {
    /// Sets up file-based logging for the application
    ///
    /// Creates a log file in the user's home directory under `.cllm/cllm.log`
    /// Uses the appropriate home directory path for Windows vs Unix systems
    ///
    /// # Returns
    /// * `Ok(())` - If logging was successfully configured
    /// * `Err(UtilsError)` - If there was an error setting up logging
    pub fn setup() -> Result<(), UtilsError> {
        let log_file = "cllm.log";
        let home = if cfg!(windows) {
            std::env::var("USERPROFILE").map_err(|_| {
                UtilsError::LoggingError("USERPROFILE environment variable not set".to_string())
            })?
        } else {
            std::env::var("HOME").map_err(|_| {
                UtilsError::LoggingError("HOME environment variable not set".to_string())
            })?
        };

        let directory = PathBuf::from(home).join(".cllm");
        let file_appender = RollingFileAppender::builder()
            .rotation(tracing_appender::rolling::Rotation::NEVER)
            .filename_prefix(log_file)
            .build(directory)
            .map_err(|e| UtilsError::LoggingError(e.to_string()))?;

        tracing_subscriber::fmt().with_writer(file_appender).init();

        Ok(())
    }
}
