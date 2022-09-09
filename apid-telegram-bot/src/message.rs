use serde::{Deserialize, Serialize};

use crate::{Chat, Poll, User};

fn is_false(value: &bool) -> bool {
    *value == false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i32,

    /// Sender of the message; empty for messages sent to channels.
    /// For backward compatibility, the field contains a fake sender user in non-channel chats,
    /// if the message was sent on behalf of a chat.
    pub from: Option<User>,

    /// Sender of the message, sent on behalf of a chat.
    /// For example,
    /// the channel itself for channel posts,
    /// the supergroup itself for messages from anonymous group administrators,
    /// the linked channel for messages automatically forwarded to the discussion group.
    /// For backward compatibility, the field *from* contains a fake sender user in non-channel chats,
    /// if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,

    /// Date the message was sent in Unix time
    pub date: i32,

    /// Conversation the message belongs to
    pub chat: Chat,

    /// For forwarded messages, sender of the original message
    pub forward_from: Option<User>,

    /// For messages forwarded from channels or from anonymous administrators, information about the original sender chat
    pub forward_from_chat: Option<Chat>,

    /// For messages forwarded from channels, identifier of the original message in the channel
    pub forward_from_message_id: Option<i32>,

    /// For forwarded messages that were originally sent in channels or by an anonymous chat administrator, signature of the message sender if present
    pub forward_signature: Option<String>,

    /// Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    pub forward_sender_name: Option<String>,

    /// For forwarded messages, date the original message was sent in Unix time
    pub forward_date: Option<i32>,

    /// True, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_automatic_forward: bool,

    /// For replies, the original message.
    /// Note that the Message object in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,

    /// Bot through which the message was sent
    pub via_bot: Option<User>,

    /// Date the message was last edited in Unix time
    pub edit_date: Option<i32>,

    /// True, if the message can't be forwarded
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_protected_content: bool,

    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,

    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<String>,

    #[serde(flatten)]
    pub content: MessageContent,

    /// Service message: data sent by a Web App
    web_app_data: Option<WebAppData>,

    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text {
        /// For text messages, the actual UTF-8 text of the message
        text: String,
        /// For text messages, special entities like usernames, URLs, bot commands, etc.
        /// that appear in the text
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        entities: Vec<MessageEntity>,
    },
    Animation {
        /// Message is an animation, information about the animation.
        /// For backward compatibility, when this field is set, the *document* field will also be set
        animation: Animation,

        /// Message is a general file, information about the file
        document: Document,

        #[serde(flatten)]
        /// Caption for the animation, audio, document, photo, video or voice
        caption: Option<CaptionBundle>,
    },
    Audio {
        /// Message is an audio file, information about the file
        audio: Audio,

        #[serde(flatten)]
        /// Caption for the animation, audio, document, photo, video or voice
        caption: Option<CaptionBundle>,
    },
    Document {
        /// Message is a general file, information about the file
        document: Document,

        #[serde(flatten)]
        /// Caption for the animation, audio, document, photo, video or voice
        caption: Option<CaptionBundle>,
    },
    Photo {
        /// Message is a photo, available sizes of the photo
        photo: Vec<PhotoSize>,

        #[serde(flatten)]
        /// Caption for the animation, audio, document, photo, video or voice
        caption: Option<CaptionBundle>,
    },
    Sticker {
        /// Message is a sticker, information about the sticker
        sticker: Sticker,
    },
    Video {
        /// Message is a video, information about the video
        video: Video,

        #[serde(flatten)]
        /// Caption for the animation, audio, document, photo, video or voice
        caption: Option<CaptionBundle>,
    },
    VideoNote {
        /// Message is a [video note](https://telegram.org/blog/video-messages-and-telescope), information about the video message
        video_note: VideoNote,
    },
    Voice {
        /// Message is a voice message, information about the file
        voice: Voice,

        #[serde(flatten)]
        /// Caption for the animation, audio, document, photo, video or voice
        caption: Option<CaptionBundle>,
    },
    Contact {
        /// Message is a shared contact, information about the contact
        contact: Contact,
    },
    Dice {
        /// Message is a dice with random value
        dice: Dice,
    },
    Game {
        /// Message is a game, information about the game.
        /// [More about games »](https://core.telegram.org/bots/api#games)
        game: Game,
    },
    Poll {
        /// Message is a native poll, information about the poll
        poll: Poll,
    },
    Venue {
        /// Message is a venue, information about the venue.
        /// For backward compatibility, when this field is set, the *location* field will also be set
        venue: Venue,

        /// Message is a shared location, information about the location
        location: Location,
    },
    Location {
        /// Message is a shared location, information about the location
        location: Location,
    },
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
    VideoChatScheduled {
        /// Service message: video chat scheduled
        video_chat_scheduled: VideoChatScheduled,
    },
    VideoChatStarted {
        /// Service message: video chat started
        video_chat_started: VideoChatStarted,
    },
    VideoChatEnded {
        /// Service message: video chat ended
        video_chat_ended: VideoChatEnded,
    },
    VideoChatParticipantsInvited {
        /// Service message: new participants invited to a video chat
        video_chat_participants_invited: VideoChatParticipantsInvited,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEntity {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Animation {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audio {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoSize {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sticker {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoNote {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Voice {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dice {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {}

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
pub struct VideoChatScheduled {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatStarted {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatEnded {}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatParticipantsInvited {}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebAppData {}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptionBundle {
    /// Caption for the animation, audio, document, photo, video or voice
    #[serde(rename = "caption")]
    text: String,

    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    caption_entities: Vec<MessageEntity>,
}