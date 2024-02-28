mod choice;
mod content;
mod usage;

use crate::utils::ToJSON;
use choice::Choice;
use content::Content;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use usage::Usage;

#[derive(Serialize, Deserialize, Debug)]
pub struct GPTResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

impl GPTResponse {
    pub fn get_content(&self) -> Option<Content> {
        self.choices.get(0).and_then(|choice| {
            let content_str = choice.message.content.as_str();
            if content_str.is_empty() {
                None
            } else {
                match serde_json::from_str::<Content>(content_str) {
                    Ok(content) if content.has_response && !content.message.is_empty() => {
                        Some(content)
                    }
                    _ => None,
                }
            }
        })
    }
}

impl ToJSON for GPTResponse {
    fn json(&self) -> String {
        to_string(&self).unwrap_or("{}".to_string())
    }
}
