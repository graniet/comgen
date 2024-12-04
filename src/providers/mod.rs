use tracing::info;

/// Common trait that must be implemented by all AI providers
pub trait AIProvider {
    /// Generates a response from the AI model for the given prompt
    ///
    /// # Arguments
    /// * `prompt` - The input text to send to the model
    ///
    /// # Returns
    /// * `Ok(String)` - The generated response text
    /// * `Err(Box<dyn Error>)` - If the request fails
    fn generate_response(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
}

pub mod anthropic;
pub mod ollama;
pub mod openai;

/// Creates a new AI provider instance based on the specified type
///
/// # Arguments
/// * `provider_type` - The type of provider to create ("openai", "anthropic", or "ollama")
/// * `config` - Application configuration containing provider settings
///
/// # Returns
/// * `Box<dyn AIProvider>` - The created provider instance
///
/// # Panics
/// * If an unknown provider type is specified
pub fn create_provider(provider_type: &str, config: &crate::config::Config) -> Box<dyn AIProvider> {
    info!("creating provider: {}", provider_type);
    match provider_type {
        "openai" => Box::new(openai::OpenAIProvider::new(
            &config.model,
            &config.openai_api_key,
        )),
        "anthropic" => Box::new(anthropic::AnthropicProvider::new(
            &config.model,
            &config.anthropic_api_key,
        )),
        "ollama" => Box::new(ollama::OllamaProvider::new(
            &config.model,
            &config.ollama_url,
        )),
        _ => panic!("Unknown provider type"),
    }
}
