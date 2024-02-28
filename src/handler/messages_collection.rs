mod mongo;
use super::{config::Config, webhook::message::Message};
use mongo::MongoDB;

use mongodb::Collection;

pub struct MessagesCollection {}

impl MessagesCollection {
    pub async fn try_new(config: &Config) -> Result<Collection<Message>, mongodb::error::Error> {
        let db = MongoDB::new(config).await?;
        let messages: Collection<Message> = db.db.collection("messages");
        Ok(messages)
    }
}
