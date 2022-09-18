use apid::Call;
use serde::{Deserialize, Serialize};

use crate::types::{ChatId, Message};

/// Use this method to send text messages. On success, the sent [`Message`] is returned.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Text of the message to be sent, 1-4096 characters after entities parsing
    pub text: String,

    /// Mode for parsing entities in the message text.
    /// See formatting options for more details.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "parse_mode")]
pub enum ParseMode {}

impl Call for SendMessage {
    type Response = Message;
}
