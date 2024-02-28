use serde::{Deserialize, Serialize};

use super::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub model: String,
    pub results: Vec<Result>,
}
