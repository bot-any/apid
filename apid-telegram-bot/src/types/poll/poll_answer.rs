use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object represents an answer of a user in a non-anonymous poll.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,

    /// The user, who changed the answer to the poll
    pub user: User,

    /// 0-based identifiers of answer options, chosen by the user.
    /// May be empty if the user retracted their vote.
    pub option_ids: Vec<i32>,
}
