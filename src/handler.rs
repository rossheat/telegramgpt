mod bot;
mod config;
pub mod gpt;
mod messages_collection;
mod moderation;
mod request;
mod webhook;

use crate::handler::gpt::message::ToGPTFormat;
use bot::Bot;
use config::Config;
use futures::TryStreamExt;
use gpt::{message::GPTMessage, response::GPTResponse, GPT};
use lambda_http::{
    http::{Response, StatusCode},
    Body, Error, Request,
};
use messages_collection::MessagesCollection;
use moderation::Moderation;
use mongodb::{bson::doc, options::FindOptions};
use request::RequestExt;
use tracing::info;
use webhook::{message::Message, Webhook};

#[tracing::instrument()]
pub async fn lambda_handler(request: Request) -> Result<Response<Body>, Error> {
    let config: Config = Config::from_env()?;
    info!("Config: {:?}", config);

    if let Err(message) = request.authorize(&config.telegram_webhook_secret_token) {
        info!("request.authorize error message: {}", message);
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(message.into())?);
    }

    let mut webhook = match Webhook::from_request(request) {
        Ok(v) => v,
        Err(message) => {
            info!("Webhook::from_request error message: {}", message);
            return Ok(Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body(message.into())?);
        }
    };
    info!("Webhook: {:?}", webhook);

    let bot = Bot::new(&config.telegram_bot_token, !config.production);
    let chat_id = webhook.message.chat.id;
    if !config.allowed_chat_ids.contains(&chat_id) {
        info!("Unknown chat id: {}", chat_id);
        bot.send_message(
            chat_id,
            format!("I don't talk to strangers ({})", chat_id).to_string(),
        )
        .await?;
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Processed webhook - Stranger".into())
            .map_err(Box::new)?);
    }

    let moderation = Moderation::new(&config.openai_api_key);
    let is_inappropriate = moderation.moderate(&mut webhook.message).await?;
    info!("is_inappropriate: {}", is_inappropriate);
    if is_inappropriate {
        bot.send_message(chat_id, "Please be appropriate.".to_string())
            .await?;
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body("Processed webhook - Inappropriate".into())
            .map_err(Box::new)?);
    }

    let messages = MessagesCollection::try_new(&config).await?;
    let insertion_result = messages.insert_one(&webhook.message, None).await;
    if let Err(err) = insertion_result {
        info!("insertion_result Err(err): {}", err);
        // Ignore duplicate key error (Telegram is resending data)
        if !err.to_string().contains("duplicate key") {
            return Err(Box::new(err));
        }
    }

    let filter = doc! { "chat.id": chat_id };
    let find_options = FindOptions::builder()
        .sort(doc! { "date": -1 })
        .limit(config.message_history_length as i64)
        .build();
    let mut cursor = messages.find(filter, find_options).await?;
    let mut recent_messages: Vec<Message> = Vec::new();
    while let Some(message) = cursor.try_next().await? {
        recent_messages.push(message);
    }
    info!("recent_messages: {:?}", &recent_messages);

    let gpt_messages: Vec<GPTMessage> = recent_messages.gpt_format(&config.system_prompt);
    info!("gpt_messages: {:?}", &gpt_messages);
    let gpt = GPT::new(
        &config.openai_api_key,
        &config.openai_model_name,
        config.max_tokens,
    );
    let gpt_response: GPTResponse = gpt.chat(gpt_messages).await?;
    info!("gpt_response: {:?}", &gpt_response);

    let mut message_content = String::new();
    if let Some(content) = gpt_response.get_content() {
        message_content = content.message.clone();
        info!("GPT response message_content: {}", &message_content);
        bot.send_message(chat_id, message_content.clone()).await?;
        let message: Message = content.to_message(&config, chat_id);
        messages.insert_one(message, None).await?;
    } else {
        info!("GPT response message content is empty");
    }

    println!("message_content: {}", message_content);

    info!("Returning response with StatusCode::OK");
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(message_content.into())
        .map_err(Box::new)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http::{header, HeaderValue};
    use test_log::test;

    #[test(tokio::test)]
    pub async fn test_lambda_handler() {
        let _ = env_logger::builder().is_test(true).try_init();

        let config: Config = Config::from_env().expect("Failed to create Config");

        let input = include_str!("../test_data/update.json");
        let mut request = lambda_http::request::from_str(input).expect("failed to create request");

        request.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        request.headers_mut().insert(
            "X-Telegram-Bot-Api-Secret-Token",
            HeaderValue::from_str(&config.telegram_webhook_secret_token)
                .expect("Failed to create header value for secret token"),
        );

        let response = lambda_handler(request)
            .await
            .expect("Failed to handle request");

        assert_eq!(StatusCode::OK, response.status());
    }
}
