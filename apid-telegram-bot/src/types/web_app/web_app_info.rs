use serde::{Deserialize, Serialize};

/// Describes a [Web App](https://core.telegram.org/bots/webapps).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in [Initializing Web Apps](https://core.telegram.org/bots/webapps#initializing-web-apps)
    pub url: String,
}
