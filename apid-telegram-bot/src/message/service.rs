use serde::{Deserialize, Serialize};

/// This object represents a service message about a change in auto-delete timer settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat; in seconds
    pub message_auto_delete_time: i32,
}
