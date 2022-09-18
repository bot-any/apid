use apid::Call;
use serde::{Deserialize, Serialize};

use crate::types::{Update, UpdateKind};

/// Use this method to receive incoming updates using long polling ([wiki](https://en.wikipedia.org/wiki/Push_technology#Long_polling)).
/// Returns an Array of [`Update`] objects.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUpdates {
    /// Identifier of the first update to be returned.
    /// Must be greater by one than the highest among the identifiers of previously received updates.
    /// By default, updates starting with the earliest unconfirmed update are returned.
    /// An update is considered confirmed as soon as [getUpdates](https://core.telegram.org/bots/api#getupdates) is called with an *offset* higher than its *update_id*.
    /// The negative offset can be specified to retrieve updates starting from -*offset* update from the end of the updates queue.
    /// All previous updates will forgotten.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Limits the number of updates to be retrieved.
    /// Values between 1-100 are accepted.
    /// Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Timeout in seconds for long polling.
    /// Defaults to 0, i.e. usual short polling.
    /// Should be positive, short polling should be used for testing purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,

    /// A JSON-serialized list of the update types you want your bot to receive.
    /// For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types.
    /// See [`UpdateKind`] for a complete list of available update types.
    /// Specify an empty list to receive all update types except *chat_member* (default).
    /// If not specified, the previous setting will be used.
    ///
    /// Please note that this parameter doesn't affect updates created before the call to the getUpdates, so unwanted updates may be received for a short period of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<UpdateKind>>,
}

impl Call for GetUpdates {
    type Response = Vec<Update>;
}
