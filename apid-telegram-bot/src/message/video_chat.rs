use serde::{Deserialize, Serialize};

use crate::User;

/// This object represents a service message about a video chat scheduled in the chat.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    pub start_date: i32,
}

/// This object represents a service message about a video chat started in the chat.
/// Currently holds no information.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatStarted {}

/// This object represents a service message about new members invited to a video chat.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat
    pub users: Vec<User>,
}

/// This object represents a service message about a video chat ended in the chat.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatEnded {
    /// Video chat duration in seconds
    pub duration: i32,
}
