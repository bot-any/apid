use serde::{Deserialize, Serialize};

use crate::{MessageEntity, PollOption};

/// This object contains information about a poll.
#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,

    /// Poll question, 1-300 characters
    pub question: String,

    /// List of poll options
    pub options: Vec<PollOption>,

    /// Total number of users that voted in the poll
    pub total_voter_count: i32,

    /// *True*, if the poll is closed
    pub is_closed: bool,

    /// *True*, if the poll is anonymous
    pub is_anonymous: bool,

    /// Poll type, currently can be “regular” or “quiz”
    #[serde(flatten)]
    pub kind: PollKind,

    /// *True*, if the poll allows multiple answers
    pub allows_multiple_answers: bool,

    /// Amount of time in seconds the poll will be active after creation
    pub open_period: Option<i32>,

    /// Point in time (Unix timestamp) when the poll will be automatically closed
    pub close_date: Option<i32>,
}

/// The kind of poll
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PollKind {
    /// The regular poll
    Regular,
    /// The poll with preset correct option
    Quiz {
        /// 0-based identifier of the correct answer option.
        /// Available only for polls in the quiz mode, which are closed,
        /// or was sent (not forwarded) by the bot or to the private chat with the bot.
        correct_option_id: Option<i32>,

        /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
        explanation: Option<String>,

        /// Special entities like usernames, URLs, bot commands, etc.
        /// that appear in the explanation
        explanation_entities: Option<Vec<MessageEntity>>,
    },
}
