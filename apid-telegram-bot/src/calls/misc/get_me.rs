use serde::{Deserialize, Serialize};

/// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetMe {}
