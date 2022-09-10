use serde::{Deserialize, Serialize};

/// This object contains information about one answer option in a poll.
#[derive(Debug, Serialize, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    text: String,

    /// Number of users that voted for this option
    voter_count: i32,
}
