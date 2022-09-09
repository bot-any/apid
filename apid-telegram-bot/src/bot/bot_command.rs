use serde::{Deserialize, Serialize};

/// This object represents a bot command.
#[derive(Debug, Serialize, Deserialize)]
pub struct BotCommand {
    /// Text of the command; 1-32 characters.
    /// Can contain only lowercase English letters, digits and underscores.
    pub command: String,

    /// Description of the command; 1-256 characters.
    pub description: String,
}
