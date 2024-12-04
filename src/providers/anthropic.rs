use super::AIProvider;

/// Provider implementation for Anthropic's AI models
pub struct AnthropicProvider {
    /// The model identifier to use for requests
    model: String,
    /// API key for authentication with Anthropic
    api_key: String,
}

impl AnthropicProvider {
    /// Creates a new instance of the Anthropic provider
    ///
    /// # Arguments
    /// * `model` - The model identifier to use
    /// * `api_key` - API key for authentication
    pub fn new(model: &str, api_key: &str) -> Self {
        Self {
            model: model.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

impl AIProvider for AnthropicProvider {
    /// Generates a response from the Anthropic API for the given prompt
    ///
    /// # Arguments
    /// * `prompt` - The input prompt to send to the model
    ///
    /// # Returns
    /// * `Ok(String)` - The generated response text
    /// * `Err(Box<dyn Error>)` - If the API request fails
    fn generate_response(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Create the request body
        let request_body = serde_json::json!({
            "model": self.model,
            "prompt": format!("\n\nHuman: {}\n\nAssistant:", prompt),
            "max_tokens_to_sample": 1000,
            "temperature": 0.7
        });

        // Make synchronous HTTP POST request to Anthropic API
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.anthropic.com/v1/complete")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request_body)
            .send()?;

        // Parse the response
        let response_json: serde_json::Value = response.json()?;

        // Extract the completion text
        let completion = response_json["completion"]
            .as_str()
            .ok_or("Failed to get completion text")?
            .to_string();

        Ok(completion)
    }
}
