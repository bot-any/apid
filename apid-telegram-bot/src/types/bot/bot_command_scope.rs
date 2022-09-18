use serde::{Deserialize, Serialize};

use crate::types::ChatId;

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
