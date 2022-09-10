use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

/// The kind of chat
#[derive(Debug, Serialize_enum_str, Deserialize_enum_str)]
pub enum ChatKind {
    /// Private chat with the inline query sender
    Sender,
    /// Private chat
    Private,
    /// Group
    Group,
    /// Supergroup
    Supergroup,
    /// Channel
    Channel,
}
