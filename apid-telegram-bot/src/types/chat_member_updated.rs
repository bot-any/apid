use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    // TODO:
    __never_happen: String,
}
