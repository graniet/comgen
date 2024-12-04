use super::AIProvider;

/// Provider implementation for OpenAI's language models
pub struct OpenAIProvider {
    /// The model identifier to use for requests
    model: String,
    /// API key for authentication with OpenAI
    api_key: String,
}

impl OpenAIProvider {
    /// Creates a new instance of the OpenAI provider
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

impl AIProvider for OpenAIProvider {
    /// Generates a response from the OpenAI API for the given prompt
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
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        });

        // Make synchronous HTTP POST request to OpenAI API
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()?;

        // Parse the response
        let response_json: serde_json::Value = response.json()?;

        // Extract the response text from the first choice
        let response_text = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Failed to get response text")?
            .to_string();

        Ok(response_text)
    }
}
