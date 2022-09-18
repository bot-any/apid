use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    // TODO:
    __never_happen: String,
}
