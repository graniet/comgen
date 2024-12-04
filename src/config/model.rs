use crate::config::error::ConfigError;
use serde::Deserialize;

/// Represents a commit message template with formatting rules and examples
#[derive(Deserialize, Clone, Debug)]
pub struct CommitTemplate {
    /// The actual template string
    pub template: String,
    /// Maximum allowed length for commit messages
    pub max_length: usize,
    /// Example commit messages following this template
    pub examples: Vec<String>,
}

/// Local template configuration containing commit types and output format
#[derive(Deserialize, Clone, Debug)]
pub struct LocalTemplate {
    /// List of valid commit types (e.g. "feat", "fix", etc)
    pub commit_types: Vec<String>,
    /// Template for formatting commit messages
    pub output_format: CommitTemplate,
}

/// Configuration for security audit functionality
#[derive(Deserialize, Clone, Debug)]
pub struct AuditConfig {
    /// Whether security auditing is enabled
    pub enabled: bool,
    /// Prompt template for security auditing
    pub prompt: String,
}

/// Main configuration struct for the application
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    /// AI provider to use (e.g. "openai", "anthropic", "ollama")
    pub provider: String,
    /// Model name to use with the provider
    pub model: String,
    /// Base prompt template for AI interactions
    pub base_prompt: String,
    /// Commit message templates configuration
    pub templates: LocalTemplate,
    /// API key for Anthropic services
    pub anthropic_api_key: String,
    /// API key for OpenAI services
    pub openai_api_key: String,
    /// URL for Ollama API endpoint
    pub ollama_url: String,
    /// Security audit configuration
    pub audit: AuditConfig,
}

impl Default for Config {
    /// Creates a default configuration with empty values and Ollama endpoint set to localhost
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            base_prompt: String::new(),
            templates: LocalTemplate {
                commit_types: Vec::new(),
                output_format: CommitTemplate {
                    template: String::new(),
                    max_length: 0,
                    examples: Vec::new(),
                },
            },
            anthropic_api_key: String::new(),
            openai_api_key: String::new(),
            ollama_url: "http://localhost:11434".to_string(),
            audit: AuditConfig {
                enabled: false,
                prompt: String::new(),
            },
        }
    }
}

impl Config {
    /// Validates the configuration based on the selected provider
    ///
    /// Returns an error if required credentials are missing for the selected provider
    pub fn validate(&self) -> Result<(), String> {
        match self.provider.as_str() {
            "openai" if self.openai_api_key.is_empty() => {
                Err("OpenAI API key is required when using OpenAI provider".to_string())
            }
            "anthropic" if self.anthropic_api_key.is_empty() => {
                Err("Anthropic API key is required when using Anthropic provider".to_string())
            }
            "ollama" if self.ollama_url.is_empty() => {
                Err("Ollama URL is required when using Ollama provider".to_string())
            }
            _ => Ok(()),
        }
    }

    /// Loads template configuration from a local file if it exists
    ///
    /// Looks for a file named "comgen.template" in the current directory and updates
    /// the configuration with its contents if found
    pub fn load_local_template(&mut self) -> Result<(), ConfigError> {
        let local_template_path = std::path::Path::new("comgen.template");

        if local_template_path.exists() {
            let template_content = std::fs::read_to_string(local_template_path)?;
            let local_template: LocalTemplate = serde_yaml::from_str(&template_content)?;

            // Replace existing templates with those from local file
            self.templates.commit_types = local_template.commit_types;
            self.templates.output_format = local_template.output_format;
        }

        Ok(())
    }
}
