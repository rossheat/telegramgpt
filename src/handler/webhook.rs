pub mod message;
use lambda_http::{Request, RequestPayloadExt};
use serde::{Deserialize, Serialize};

use message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct Webhook {
    pub update_id: i64,
    pub message: Message,
}

impl Webhook {
    pub fn from_request(request: Request) -> Result<Self, String> {
        let webhook_result = request.payload::<Webhook>();
        match webhook_result {
            Ok(Some(body)) => Ok(body),
            Ok(None) => Err("Request body not found".to_string()),
            Err(err) => Err(err.to_string()),
        }
    }
}
