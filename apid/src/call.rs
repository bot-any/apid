use serde::de::DeserializeOwned;

/// This object could be used to call an API.
pub trait Call {
    /// The return type of the API on success.
    type Response: DeserializeOwned;
}
