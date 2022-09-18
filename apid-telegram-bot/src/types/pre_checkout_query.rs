use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    // TODO:
    __never_happen: String,
}
