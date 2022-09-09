use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    ///
    /// This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it.
    /// But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    id: i64,

    /// True, if this user is a bot
    is_bot: bool,

    /// User's or bot's first name
    first_name: String,

    /// User's or bot's last name
    last_name: Option<String>,

    /// User's or bot's username
    username: Option<String>,

    /// [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    language_code: Option<String>,

    /// *True*, if this user is a Telegram Premium user
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    is_premium: bool,

    /// *True*, if this user added the bot to the attachment menu
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    added_to_attachment_menu: bool,

    /// *True*, if the bot can be invited to groups. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    can_join_groups: bool,

    /// *True*, if privacy mode is disabled for the bot. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    can_read_all_group_messages: bool,

    /// *True*, if the bot supports inline queries. Returned only in [getMe](https://core.telegram.org/bots/api#getme).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    supports_inline_queries: bool,
}
