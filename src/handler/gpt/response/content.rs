use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::handler::{
    config::Config,
    webhook::message::{chat::Chat, user::User, Message},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub has_response: bool,
    pub message: String,
}

impl Content {
    pub fn to_message(&self, config: &Config, chat_id: i64) -> Message {
        let current_time = Utc::now().timestamp();
        let placeholder_user = User {
            id: -1,
            is_bot: true,
            first_name: config.bot_name.clone(),
            last_name: String::from("Bot"),
        };

        let placeholder_chat = Chat {
            id: chat_id,
            title: Some(String::from("Bot Placeholder")),
        };

        Message {
            message_id: current_time,
            from: placeholder_user,
            chat: placeholder_chat,
            date: current_time,
            text: self.message.clone(),
            is_inappropriate: Some(false),
        }
    }
}
