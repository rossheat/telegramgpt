use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GPTMessage {
    pub role: String,
    pub content: String,
}
pub trait ToGPTFormat {
    fn gpt_format(&self, system_prompt: &str) -> Vec<GPTMessage>;
}
