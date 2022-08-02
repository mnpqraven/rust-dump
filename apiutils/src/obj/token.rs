use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenCollection {
    pub code: i8,
    pub result: Token,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}
