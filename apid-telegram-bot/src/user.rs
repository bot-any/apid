use serde::{Deserialize, Serialize};

/// This object represents a Telegram user or bot.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    ///
    /// This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it.
    /// But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    /// True, if this user is a bot
    pub is_bot: bool,

    /// User's or bot's first name
    pub first_name: String,

    /// User's or bot's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// User's or bot's username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// *True*, if this user is a Telegram Premium user
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub is_premium: bool,

    /// *True*, if this user added the bot to the attachment menu
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub added_to_attachment_menu: bool,

    /// *True*, if the bot can be invited to groups. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub can_join_groups: bool,

    /// *True*, if privacy mode is disabled for the bot. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub can_read_all_group_messages: bool,

    /// *True*, if the bot supports inline queries. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub supports_inline_queries: bool,
}
