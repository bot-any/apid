use serde::{Deserialize, Serialize};

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub struct KeyboardButtonPollType {
    /// The actual kind
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub kind: Option<KeyboardButtonPollTypeKind>,
}

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum KeyboardButtonPollTypeKind {
    /// If quiz is passed, the user will be allowed to create only polls in the quiz mode.
    Quiz,
    /// If regular is passed, only regular polls will be allowed.
    Regular,
}
