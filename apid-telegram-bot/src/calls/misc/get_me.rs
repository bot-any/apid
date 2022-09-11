use apid::Call;
use serde::{Deserialize, Serialize};

use crate::types::User;

/// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetMe {}

impl Call for GetMe {
    type Response = User;
}
