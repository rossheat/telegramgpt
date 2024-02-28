pub mod chat;
pub mod user;

use crate::handler::gpt::message::{GPTMessage, ToGPTFormat};
use chat::Chat;
use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(alias = "_id")]
    #[serde(rename(serialize = "_id"))]
    pub message_id: i64,
    pub from: User,
    pub chat: Chat,
    pub date: i64,
    #[serde(default = "default_text")]
    pub text: String,
    pub is_inappropriate: Option<bool>,
}

fn default_text() -> String {
    "<DEFAULT-TEXT>".to_string()
}

impl Message {
    pub fn format_for_gpt(&self) -> GPTMessage {
        let naive_datetime = NaiveDateTime::from_timestamp_opt(self.date, 0).unwrap_or_default();
        let date = Utc.from_utc_datetime(&naive_datetime);
        let date = date.format("%H:%M:%S").to_string();

        let sender_name = format!("{} {}", self.from.first_name, self.from.last_name)
            .trim()
            .to_string();

        let text = if self.is_inappropriate.unwrap_or(false) {
            "<MESSAGE-REDACTED-FOR-INAPPROPRIATE-CONTENT>".to_string()
        } else {
            self.text.clone()
        };

        let role = if self.from.is_bot {
            "assistant"
        } else {
            "user"
        };

        GPTMessage {
            role: role.to_string(),
            content: format!("{} at {}: {}", sender_name, date, text),
        }
    }
}

impl ToGPTFormat for Vec<Message> {
    fn gpt_format(&self, system_prompt: &str) -> Vec<GPTMessage> {
        let mut formatted_messages = vec![GPTMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        }];
        for message in self.iter().rev() {
            formatted_messages.push(message.format_for_gpt());
        }
        formatted_messages
    }
}
