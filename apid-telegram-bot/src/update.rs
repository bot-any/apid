use serde::{Deserialize, Serialize};

use crate::{
    CallbackQuery, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message,
    Poll, PollAnswer, PreCheckoutQuery, ShippingQuery,
};

/// This [object](https://core.telegram.org/bots/api#available-types) represents an incoming update.
/// At most **one** of the optional parameters can be present in any given update.
#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    /// The update's unique identifier.
    /// Update identifiers start from a certain positive number and increase sequentially.
    /// This ID becomes especially handy if you're using webhooks,
    /// since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order.
    /// If there are no new updates for at least a week,
    /// then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i32,

    /// The event encoded in the update.
    /// As specified above, the optional parameters can be present at most one.
    /// Thus we provide event separated for ease of use.
    #[serde(flatten)]
    pub event: Option<UpdateEvent>,
}

/// The update event can be taken from an [`Update`].
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateEvent {
    /// New incoming message of any kind - text, photo, sticker, etc.
    Message(#[serde(rename = "message")] Message),
    /// New version of a message that is known to the bot and was edited
    MessageEdit(#[serde(rename = "edited_message")] Message),
    /// New incoming channel post of any kind - text, photo, sticker, etc.
    ChannelPost(#[serde(rename = "channel_post")] Message),
    /// New version of a channel post that is known to the bot and was edited
    ChannelPostEdit(#[serde(rename = "edited_channel_post")] Message),
    /// New incoming [inline](https://core.telegram.org/bots/api#inline-mode) query
    InlineQuery(#[serde(rename = "inline_query")] InlineQuery),
    /// The result of an [inline](https://core.telegram.org/bots/api#inline-mode) query that was chosen by a user and sent to their chat partner.
    /// Please see our documentation on the [feedback collecting](https://core.telegram.org/bots/inline#collecting-feedback) for details on how to enable these updates for your bot.
    ChosenInlineResult(#[serde(rename = "chosen_inline_result")] ChosenInlineResult),
    /// New incoming callback query
    CallbackQuery(#[serde(rename = "callback_query")] CallbackQuery),
    /// New incoming shipping query. Only for invoices with flexible price
    ShippingQuery(#[serde(rename = "shipping_query")] ShippingQuery),
    /// New incoming pre-checkout query. Contains full information about checkout
    PreCheckoutQuery(#[serde(rename = "pre_checkout_query")] PreCheckoutQuery),
    /// New poll state.
    /// Bots receive only updates about stopped polls and polls, which are sent by the bot
    Poll(#[serde(rename = "poll")] Poll),
    /// A user changed their answer in a non-anonymous poll.
    /// Bots receive new votes only in polls that were sent by the bot itself.
    PollAnswer(#[serde(rename = "poll_answer")] PollAnswer),
    /// The bot's chat member status was updated in a chat.
    /// For private chats, this update is received only when the bot is blocked or unblocked by the user.
    PrivateChatMemberUpdated(#[serde(rename = "my_chat_member")] ChatMemberUpdated),
    /// A chat member's status was updated in a chat.
    /// The bot must be an administrator in the chat and must explicitly specify “chat_member” in the list of *allowed_updates* to receive these updates.
    ChatMemberUpdated(#[serde(rename = "chat_member")] ChatMemberUpdated),
    /// A request to join the chat has been sent.
    /// The bot must have the *can_invite_users* administrator right in the chat to receive these updates.
    ChatJoinRequest(#[serde(rename = "chat_join_request")] ChatJoinRequest),
}
