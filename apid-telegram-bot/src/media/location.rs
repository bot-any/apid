use serde::{Deserialize, Serialize};

/// This object represents a point on the map.
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f64,

    /// Latitude as defined by sender
    pub latitude: f64,

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,

    /// Time relative to the message sending date, during which the location can be updated; in seconds.
    /// For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i32>,

    /// The direction in which user is moving, in degrees; 1-360.
    /// For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i32>,

    /// The maximum distance for proximity alerts about approaching another chat member, in meters.
    /// For sent live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i32>,
}
