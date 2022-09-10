use serde::{Deserialize, Serialize};

use crate::types::{KeyboardButtonPollType, WebAppInfo};

/// This object represents one button of the reply keyboard.
/// For simple text buttons String can be used instead of this object to specify text of the button.
/// Optional fields *web_app*, *request_contact*, *request_location*, and *request_poll* are mutually exclusive.
#[derive(Debug, Serialize, Deserialize)]
pub struct Keyboardbutton {
    /// Text of the button.
    /// If none of the optional fields are used, it will be sent as a message when the button is pressed
    pub text: String,

    /// The action to be performed when a keyboard button is pressed.
    #[serde(flatten)]
    pub action: KeyboardbuttonAction,
}

/// The action to be performed when a keyboard button is pressed.
///
/// **Note**:
/// *request_contact* and *request_location* options will only work in Telegram versions released after 9 April, 2016.
/// Older clients will display *unsupported message*.
///
/// **Note**:
/// *request_poll* option will only work in Telegram versions released after 23 January, 2020.
/// Older clients will display *unsupported message*.
///
/// **Note**:
/// *web_app* option will only work in Telegram versions released after 16 April, 2022.
/// Older clients will display *unsupported message*.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyboardbuttonAction {
    /// If *True*, the user's phone number will be sent as a contact when the button is pressed.
    /// Available in private chats only.
    RequestContact(#[serde(rename = "request_contact")] bool),
    /// If *True*, the user's current location will be sent when the button is pressed.
    /// Available in private chats only.
    RequestLocation(#[serde(rename = "request_location")] bool),
    /// If specified, the user will be asked to create a poll and send it to the bot when the button is pressed.
    /// Available in private chats only.
    RequestPoll(#[serde(rename = "request_poll")] KeyboardButtonPollType),
    /// If specified, the described [Web App](https://core.telegram.org/bots/webapps) will be launched when the button is pressed.
    /// The Web App will be able to send a “web_app_data” service message.
    /// Available in private chats only.
    WebApp(#[serde(rename = "web_app")] WebAppInfo),
}
