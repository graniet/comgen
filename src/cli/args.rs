use super::error::CliError;
use clap::Parser;
use std::path::PathBuf;

/// Command line arguments parser for the cllm application
#[derive(Parser, Debug)]
#[command(name = "comgen")]
pub struct Cli {
    /// Path to the configuration file
    #[arg(long, default_value = "~/.comgen/config.yaml")]
    pub config: PathBuf,

    /// Optional prefix to add to commit messages
    #[arg(short = 'p', long = "prefix", default_value = "")]
    pub prefix: String,

    /// Whether to automatically push changes after commit
    #[arg(short = 'a', long = "auto-push", default_value = "false")]
    pub auto_push: bool,

    /// Whether to handle multiple files in a single commit
    #[arg(short = 'm', long = "multi-file", default_value = "false")]
    pub multi_file: bool,

    /// Whether to perform security audit on changes
    #[arg(long = "audit", default_value = "true")]
    pub audit: bool,

    /// Minimum severity level for audit findings
    #[arg(long = "audit-level", default_value = "MEDIUM")]
    pub audit_level: String,

    /// Whether to force operations without confirmation
    #[arg(long = "force", default_value = "false")]
    pub force: bool,
}

impl Cli {
    /// Parse command line arguments into a Cli instance
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Get the configuration file path as a String
    ///
    /// # Returns
    /// - Ok(String) containing the config path
    /// - Err(CliError) if the path is invalid
    pub fn get_config_path(&self) -> Result<String, CliError> {
        self.config
            .to_str()
            .map(String::from)
            .ok_or_else(|| CliError::InvalidInput("Invalid config path".to_string()))
    }
}
