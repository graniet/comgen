use super::AIProvider;

/// Provider implementation for Ollama's local AI models
pub struct OllamaProvider {
    /// The model identifier to use for requests
    model: String,
    /// Base URL for the Ollama API endpoint
    base_url: String,
}

impl OllamaProvider {
    /// Creates a new instance of the Ollama provider
    ///
    /// # Arguments
    /// * `model` - The model identifier to use
    /// * `base_url` - Base URL for the Ollama API
    pub fn new(model: &str, base_url: &str) -> Self {
        Self {
            model: model.to_string(),
            base_url: base_url.to_string(),
        }
    }
}

impl AIProvider for OllamaProvider {
    /// Generates a response from the Ollama API for the given prompt
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
            "prompt": prompt,
            "stream": false
        });

        // Make synchronous HTTP POST request to Ollama API
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request_body)
            .send()?;

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        // Parse the response
        let response_json: serde_json::Value = response.json()?;

        // Extract the response text with better error handling
        let response_text = response_json["response"]
            .as_str()
            .ok_or("Response field not found or not a string")?
            .to_string();

        Ok(response_text)
    }
}
