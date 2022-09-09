use serde::{Deserialize, Serialize};

/// This object represents a unique message identifier.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: i32,
}
