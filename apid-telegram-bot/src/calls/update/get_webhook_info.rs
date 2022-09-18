use apid::Call;
use serde::{Deserialize, Serialize};

use crate::types::WebhookInfo;

/// Use this method to get current webhook status.
/// Requires no parameters.
/// On success, returns a [`WebhookInfo`] object.
/// If the bot is using getUpdates, will return an object with the url field empty.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetWebhookInfo {}

impl Call for GetWebhookInfo {
    type Response = WebhookInfo;
}
