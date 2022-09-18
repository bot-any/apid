use serde::{Deserialize, Serialize};

use crate::types::{ChatKind, ChatLocation, ChatPhoto, Message};

/// This object represents a chat.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    /// Unique identifier for this chat.
    ///
    /// This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it.
    /// But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    /// Type of chat,
    /// can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub kind: ChatKind,

    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,

    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,

    /// The user chat with in a private chat
    #[serde(flatten)]
    pub chat_user: Option<ChatUser>,

    /// Chat photo.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    pub photo: Option<ChatPhoto>,

    /// *True*, if privacy settings of the other party in the private chat allows to use `tg://user?id=<user_id>` links only in chats with the user.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub has_private_forwards: bool,

    /// *True*, if the privacy settings of the other party restrict sending voice and video note messages in the private chat.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub has_restricted_voice_and_video_messages: bool,

    /// *True*, if users need to join the supergroup before they can send messages.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub join_to_send_messages: bool,

    /// *True*, if all users directly joining the supergroup need to be approved by supergroup administrators.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub join_by_request: bool,

    /// Description, for groups, supergroups and channel chats.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    pub description: Option<String>,

    /// Primary invite link, for groups, supergroups and channel chats.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    pub invite_link: Option<String>,

    /// The most recent pinned message (by sending date).
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    pub pinned_message: Option<Box<Message>>,

    /// Default chat member permissions, for groups and supergroups.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    pub permissions: Option<ChatPermissions>,

    /// For supergroups, the minimum allowed delay between consecutive messages sent by each unpriviledged user; in seconds.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    pub slow_mode_delay: Option<i32>,

    /// The time after which all messages sent to the chat will be automatically deleted; in seconds.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    pub message_auto_delete_time: Option<i32>,

    /// *True*, if messages from the chat can't be forwarded to other chats.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub has_protected_content: bool,

    /// For supergroups, name of group sticker set.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    pub sticker_set_name: Option<String>,

    /// *True*, if the bot can change the group sticker set.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub can_set_sticker_set: bool,

    /// Unique identifier for the linked chat,
    /// i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats.
    /// This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it.
    /// But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    pub linked_chat_id: Option<i64>,

    /// For supergroups, the location to which the supergroup is connected.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    pub location: Option<ChatLocation>,
}

/// This object respresents an information about user from private chat
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatUser {
    /// First name of the other party in a private chat
    pub first_name: String,

    /// Last name of the other party in a private chat
    pub last_name: Option<String>,

    /// Bio of the other party in a private chat.
    /// Returned only in [getChat](https://core.telegram.org/bots/api#getchat).
    pub bio: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatPermissions {}
