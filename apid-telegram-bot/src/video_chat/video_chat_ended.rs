use serde::{Deserialize, Serialize};

/// This object represents a service message about a video chat ended in the chat.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatEnded {
    /// Video chat duration in seconds
    pub duration: i32,
}
