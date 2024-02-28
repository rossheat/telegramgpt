use mongodb::{error::Error, options::ClientOptions, Client, Database};

use crate::handler::config::Config;
pub struct MongoDB {
    pub db: Database,
}

impl MongoDB {
    #[tracing::instrument()]
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let client_options = ClientOptions::parse(&config.mongodb_uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&config.bot_name);
        return Ok(Self { db });
    }
}
