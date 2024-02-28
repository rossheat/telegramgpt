use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContent {
    pub role: String,
    pub content: String,
}
