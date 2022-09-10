use serde::{Deserialize, Serialize};

use crate::{
    Animation, Chat, Contact, Dice, Game, MessageEntity, PhotoSize, Poll, User, VideoChatEnded,
    VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted, WebAppData,
};

/// This object represents a message.
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i32,

    /// Sender of the message; empty for messages sent to channels.
    /// For backward compatibility, the field contains a fake sender user in non-channel chats,
    /// if the message was sent on behalf of a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,

    /// Sender of the message, sent on behalf of a chat.
    /// For example,
    /// the channel itself for channel posts,
    /// the supergroup itself for messages from anonymous group administrators,
    /// the linked channel for messages automatically forwarded to the discussion group.
    /// For backward compatibility, the field *from* contains a fake sender user in non-channel chats,
    /// if the message was sent on behalf of a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Chat>,

    /// Date the message was sent in Unix time
    pub date: i32,

    /// Conversation the message belongs to
    pub chat: Chat,

    /// For forwarded messages, sender of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from: Option<User>,

    /// For messages forwarded from channels or from anonymous administrators, information about the original sender chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_chat: Option<Chat>,

    /// For messages forwarded from channels, identifier of the original message in the channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_message_id: Option<i32>,

    /// For forwarded messages that were originally sent in channels or by an anonymous chat administrator, signature of the message sender if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_signature: Option<String>,

    /// Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_sender_name: Option<String>,

    /// For forwarded messages, date the original message was sent in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_date: Option<i32>,

    /// True, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub is_automatic_forward: bool,

    /// For replies, the original message.
    /// Note that the Message object in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,

    /// Bot through which the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,

    /// Date the message was last edited in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i32>,

    /// True, if the message can't be forwarded
    #[serde(default, skip_serializing_if = "crate::util::is_false")]
    pub has_protected_content: bool,

    /// The unique identifier of a media message group this message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,

    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,

    /// The content of the message
    #[serde(flatten)]
    pub content: MessageContent,

    /// Service message: data sent by a Web App
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,

    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// The object representing message content
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// Message is a text message
    Text {
        /// For text messages, the actual UTF-8 text of the message
        text: String,
        /// For text messages, special entities like usernames, URLs, bot commands, etc.
        /// that appear in the text
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        entities: Vec<MessageEntity>,
    },

    /// Message is an animation
    Animation {
        /// Message is an animation, information about the animation.
        /// For backward compatibility, when this field is set, the *document* field will also be set
        animation: Animation,

        /// Message is a general file, information about the file
        document: Document,

        /// Caption for the animation, audio, document, photo, video or voice
        #[serde(flatten, skip_serializing_if = "Option::is_none")]
        caption: Option<Caption>,
    },

    /// Message is an audio file
    Audio {
        /// Message is an audio file, information about the file
        audio: Audio,

        /// Caption for the animation, audio, document, photo, video or voice
        #[serde(flatten, skip_serializing_if = "Option::is_none")]
        caption: Option<Caption>,
    },

    /// Message is a general file
    Document {
        /// Message is a general file, information about the file
        document: Document,

        /// Caption for the animation, audio, document, photo, video or voice
        #[serde(flatten, skip_serializing_if = "Option::is_none")]
        caption: Option<Caption>,
    },

    /// Message is a photo
    Photo {
        /// Message is a photo, available sizes of the photo
        photo: Vec<PhotoSize>,

        /// Caption for the animation, audio, document, photo, video or voice
        #[serde(flatten, skip_serializing_if = "Option::is_none")]
        caption: Option<Caption>,
    },

    /// Message is a sticker
    Sticker(#[serde(rename = "sticker")] Sticker),

    /// Message is a video
    Video {
        /// Message is a video, information about the video
        video: Video,

        /// Caption for the animation, audio, document, photo, video or voice
        #[serde(flatten, skip_serializing_if = "Option::is_none")]
        caption: Option<Caption>,
    },

    /// Message is a [video note](https://telegram.org/blog/video-messages-and-telescope)
    VideoNote(#[serde(rename = "video_note")] VideoNote),

    /// Message is a voice message
    Voice {
        /// Message is a voice message, information about the file
        voice: Voice,

        /// Caption for the animation, audio, document, photo, video or voice
        #[serde(flatten, skip_serializing_if = "Option::is_none")]
        caption: Option<Caption>,
    },

    /// Message is a shared contact
    Contact(#[serde(rename = "contact")] Contact),

    /// Message is a dice with random value
    Dice(#[serde(rename = "dice")] Dice),

    /// Message is a game.
    /// [More about games »](https://core.telegram.org/bots/api#games)
    Game(#[serde(rename = "game")] Game),

    /// Message is a native poll
    Poll(#[serde(rename = "poll")] Poll),

    /// Message is a venue.
    Venue {
        /// Message is a venue, information about the venue.
        /// For backward compatibility, when this field is set, the *location* field will also be set
        venue: Venue,

        /// information about the location
        location: Location,
    },

    /// Message is a shared location
    Location(#[serde(rename = "location")] Location),

    NewChatMembers {
        /// New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
        new_chat_members: Vec<User>,
    },

    LeftChatMember {
        /// A member was removed from the group, information about them (this member may be the bot itself)
        left_chat_member: User,
    },

    NewChatTitle {
        /// A chat title was changed to this value
        new_chat_title: String,
    },

    NewChatPhoto {
        /// A chat photo was change to this value
        new_chat_photo: Vec<PhotoSize>,
    },

    DeleteChatPhoto {
        /// Service message: the chat photo was deleted
        delete_chat_photo: bool,
    },

    GroupChatCreated {
        /// Service message: the group has been created
        group_chat_created: bool,
    },

    SupergroupChatCreated {
        /// Service message: the supergroup has been created.
        /// This field can't be received in a message coming through updates,
        /// because bot can't be a member of a supergroup when it is created.
        /// It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
        supergroup_chat_created: bool,
    },

    ChannelChatCreated {
        /// Service message: the channel has been created.
        /// This field can't be received in a message coming through updates,
        /// because bot can't be a member of a channel when it is created.
        /// It can only be found in reply_to_message if someone replies to a very first message in a channel.
        channel_chat_created: bool,
    },

    MessageAutoDeleteTimerChanged {
        /// Service message: auto-delete timer settings changed in the chat
        message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged,
    },

    MigrateToChatId {
        /// The group has been migrated to a supergroup with the specified identifier.
        /// This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it.
        /// But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
        migrate_to_chat_id: i64,
    },

    MigrateFromChatId {
        /// The supergroup has been migrated from a group with the specified identifier.
        /// This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it.
        /// But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
        migrate_from_chat_id: i64,
    },

    PinnedMessage {
        /// Specified message was pinned.
        /// Note that the Message object in this field will not contain further *reply_to_message* fields even if it is itself a reply.
        pinned_message: Box<Message>,
    },

    Invoice {
        /// Message is an invoice for a [payment](https://core.telegram.org/bots/api#payments), information about the invoice.
        /// [More about payments »](https://core.telegram.org/bots/api#payments)
        invoice: Invoice,
    },

    SuccessfulPayment {
        /// Message is a service message about a successful payment, information about the payment.
        /// [More about payments »](https://core.telegram.org/bots/api#payments)
        successful_payment: SuccessfulPayment,
    },

    Login {
        /// The domain name of the website on which the user has logged in.
        /// [More about Telegram Login »](https://core.telegram.org/widgets/login)
        connected_website: String,

        /// Telegram Passport data
        passport_data: PassportData,
    },

    ProximityAlertTriggered {
        /// Service message.
        /// A user in the chat triggered another user's proximity alert while sharing Live Location.
        proximity_alert_triggered: ProximityAlertTriggered,
    },

    /// Service message: video chat scheduled
    VideoChatScheduled(#[serde(rename = "video_chat_scheduled")] VideoChatScheduled),

    /// Service message: video chat started
    VideoChatStarted(#[serde(rename = "video_chat_started")] VideoChatStarted),

    /// Service message: video chat ended
    VideoChatEnded(#[serde(rename = "video_chat_ended")] VideoChatEnded),

    /// Service message: new participants invited to a video chat
    VideoChatParticipantsInvited(
        #[serde(rename = "video_chat_participants_invited")] VideoChatParticipantsInvited,
    ),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audio {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sticker {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoNote {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Voice {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Venue {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessfulPayment {}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassportData {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProximityAlertTriggered {}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {}

/// Caption for the animation, audio, document, photo, video or voice
#[derive(Debug, Serialize, Deserialize)]
pub struct Caption {
    /// Caption text
    #[serde(rename = "caption")]
    pub text: String,

    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,
}
