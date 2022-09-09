use serde::{Deserialize, Serialize};

use crate::User;

/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEntity {
    /// The kind of message entity, and its metadata
    pub kind: MessageEntityKind,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i32,
    /// Length of the entity in UTF-16 code units
    pub length: i32,
}

/// This object represents the kind of message entity, and its metadata.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum MessageEntityKind {
    /// `@username`
    Mention,
    /// `#hashtag`
    Hashtag,
    /// `$USD`
    Cashtag,
    /// `/start@jobs_bot`
    BotCommand,
    /// `https://telegram.org`
    Url,
    /// `do-not-reply@telegram.org`
    Email,
    /// `+1-212-555-0123`
    PhoneNumber,
    /// **bold text**
    Bold,
    /// *italic text*
    Italic,
    /// <u>underline</u>
    Underline,
    /// ~~strikethrough text~~
    Strikethrough,
    /// spoiler message
    /// <span
    ///     id="message-tntity-kind-spoiler"
    ///     style="background: currentColor; cursor: pointer; border-radius: 0.25em; transition: all 0.2s ease-in-out;"
    ///     onClick="const element = document.getElementById('message-tntity-kind-spoiler'); element.style.background = null; element.style.cursor = null;"
    /// >that can be revealed with click</span>
    Spoiler,
    /// `monowidth string`
    Code,
    /// ```
    /// monowidth block
    /// ```
    Pre {
        /// For “pre” only, the programming language of the entity text
        language: String,
    },
    /// for clickable text URLs
    TextLink {
        /// For “text_link” only, URL that will be opened after user taps on the text
        url: String,
    },
    /// for users [without usernames](https://telegram.org/blog/edit#new-mentions)
    TextMention {
        /// For “text_mention” only, the mentioned user
        user: User,
    },
    /// for inline custom emoji stickers
    CustomEmoji {
        /// For “custom_emoji” only, unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker
        custom_emoji_id: String,
    },
}
