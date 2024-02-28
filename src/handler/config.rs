use hcl::from_str;
use lambda_http;
use serde::Deserialize;
use std::{env, fs};

#[derive(Deserialize, Debug)]
// #[serde(deny_unknown_fields)]
pub struct Config {
    pub telegram_bot_token: String,
    pub openai_api_key: String,
    pub mongodb_uri: String,
    pub allowed_chat_ids: Vec<i64>,
    pub bot_name: String,
    pub message_history_length: i32,
    pub openai_model_name: String,
    pub system_prompt: String,
    pub max_tokens: i64,
    pub production: bool,
    pub telegram_webhook_secret_token: String,
}

impl Config {
    #[tracing::instrument()]
    pub fn from_env() -> Result<Self, lambda_http::Error> {
        let config = if env::var("PRODUCTION").is_ok() {
            // Running in production, load environment variables
            Config {
                telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN")?,
                openai_api_key: env::var("OPENAI_API_KEY")?,
                mongodb_uri: env::var("MONGODB_URI")?,
                allowed_chat_ids: serde_json::from_str(&env::var("ALLOWED_CHAT_IDS")?)?,
                bot_name: env::var("BOT_NAME")?,
                message_history_length: env::var("MESSAGE_HISTORY_LENGTH")?.parse()?,
                openai_model_name: env::var("OPENAI_MODEL_NAME")?,
                system_prompt: env::var("SYSTEM_PROMPT")?,
                max_tokens: env::var("MAX_TOKENS")?.parse()?,
                telegram_webhook_secret_token: env::var("TELEGRAM_WEBHOOK_SECRET_TOKEN")?,
                production: true,
            }
        } else {
            // Running locally, load from .tfvars file
            let file_path = "./infra/terraform.tfvars";
            let file_contents = fs::read_to_string(file_path)?;
            from_str::<Config>(&file_contents)?
        };

        return Ok(config);
    }
}
