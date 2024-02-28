mod request;
mod response;
mod result;

use request::Request;
use response::Response;

use super::webhook::message::Message;
use reqwest::Client;

pub struct Moderation {
    openai_api_key: String,
    client: Client,
}

impl Moderation {
    pub fn new(openai_api_key: &str) -> Self {
        Moderation {
            openai_api_key: openai_api_key.to_string(),
            client: Client::new(),
        }
    }

    pub async fn moderate(&self, message: &mut Message) -> Result<bool, reqwest::Error> {
        let request_body = Request {
            input: message.text.clone(),
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/moderations")
            .header("Authorization", format!("Bearer {}", &self.openai_api_key))
            .json(&request_body)
            .send()
            .await?;

        let moderation_response: Response = response.json().await?;

        if moderation_response
            .results
            .iter()
            .any(|result| result.flagged)
        {
            message.is_inappropriate = Some(true);
            Ok(true)
        } else {
            message.is_inappropriate = Some(false);
            Ok(false)
        }
    }
}
