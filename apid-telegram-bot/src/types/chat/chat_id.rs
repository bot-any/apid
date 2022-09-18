use serde::{Deserialize, Serialize};

/// The chat id either an integer or a string
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatId {
    /// The integer chat id
    Int(i64),
    /// The string chat id in the format of `@supergroupusername`
    String(String),
}

impl From<i64> for ChatId {
    fn from(value: i64) -> Self {
        ChatId::Int(value)
    }
}

impl From<String> for ChatId {
    fn from(value: String) -> Self {
        ChatId::String(value)
    }
}
