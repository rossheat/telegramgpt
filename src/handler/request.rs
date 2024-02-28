use lambda_http::Request;

pub trait RequestExt {
    fn authorize(&self, expected_token: &str) -> Result<(), String>;
}

impl RequestExt for Request {
    fn authorize(&self, expected_token: &str) -> Result<(), String> {
        if self
            .headers()
            .get("X-Telegram-Bot-Api-Secret-Token")
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
            != expected_token
        {
            Err("Unauthorized: Invalid secret token".to_string())
        } else {
            Ok(())
        }
    }
}
