use chrono::Utc;
use reqwest::Client;

pub struct Bot {
    token: String,
    client: Client,
    debug: bool,
}

impl Bot {
    pub fn new(token: &str, debug: bool) -> Self {
        Self {
            token: token.to_string(),
            client: reqwest::Client::new(),
            debug,
        }
    }

    pub async fn send_message(
        &self,
        chat_id: i64,
        mut text: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);
        if self.debug {
            text = format!(
                "Message: {} \n\nMessage sent from Lambda function at {}",
                text,
                Utc::now()
            )
        }
        let params = [("chat_id", chat_id.to_string()), ("text", text.to_string())];
        self.client.post(url).form(&params).send().await
    }
}
