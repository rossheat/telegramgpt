use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub title: Option<String>,
}
