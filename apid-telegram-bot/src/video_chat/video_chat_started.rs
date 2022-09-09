use serde::{Deserialize, Serialize};

/// This object represents a service message about a video chat started in the chat.
/// Currently holds no information.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChatStarted {}
