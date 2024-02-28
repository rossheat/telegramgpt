pub mod message;
pub mod response;

use message::GPTMessage;
use response::GPTResponse;

use reqwest::Client;

pub struct GPT {
    openai_api_key: String,
    model_name: String,
    max_tokens: i64,
    client: reqwest::Client,
}

impl GPT {
    pub fn new(openai_api_key: &str, model_name: &str, max_tokens: i64) -> Self {
        GPT {
            openai_api_key: openai_api_key.to_string(),
            model_name: model_name.to_string(),
            client: Client::new(),
            max_tokens,
        }
    }

    pub async fn chat(&self, messages: Vec<GPTMessage>) -> Result<GPTResponse, reqwest::Error> {
        let request_json = &serde_json::json!({
            "model": &self.model_name,
            "messages": messages,
            "response_format": {"type": "json_object"},
            "max_tokens": self.max_tokens
        });

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.openai_api_key))
            .json(request_json)
            .send()
            .await?;

        let gpt_response: GPTResponse = response.json().await?;
        Ok(gpt_response)
    }
}
