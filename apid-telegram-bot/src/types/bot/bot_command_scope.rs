use serde::{Deserialize, Serialize};

/// This object represents the scope to which bot commands are applied.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum BotCommandScope {
    /// Represents the default [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands.
    /// Default commands are used if no commands with a narrower scope are specified for the user.
    Default,
    /// Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands,
    /// covering all private chats.
    AllPrivateChats,
    /// Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands,
    /// covering all group and supergroup chats.
    AllGroupChats,
    /// Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands,
    /// covering all group and supergroup chat administrators.
    AllChatAdministrators,
    /// Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands,
    /// covering a specific chat.
    Chat {
        /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
        chat_id: ChatId,
    },
    /// Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands,
    /// covering all administrators of a specific group or supergroup chat.
    ChatAdministrators {
        /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
        chat_id: ChatId,
    },
    /// Represents the [scope](https://core.telegram.org/bots/api#botcommandscope) of bot commands,
    /// covering a specific member of a group or supergroup chat.
    ChatMember {
        /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
        chat_id: ChatId,
        /// Unique identifier of the target user
        user_id: i64,
    },
}

/// The chat id either an integer or a string
#[derive(Debug, Serialize, Deserialize)]
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
