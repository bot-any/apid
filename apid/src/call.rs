use serde::de::DeserializeOwned;

pub trait Call {
    type Response: DeserializeOwned;
}
