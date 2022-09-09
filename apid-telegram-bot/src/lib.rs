//! # apid-telegram-bot
//!
//! This crate provides types for Telegram API.
//! The tpyes implementing [`Debug`], [`Serialize`](serde::Serialize) and [`Deserialize`](serde::Deserialize) traits for covenience.
//! Also, carefully designed enum variants for ergonomics.
//!
//! ## Telegram Parity
//!
//! Current type definitions are written with August 12, 2022 (Bot API 6.2)

#![warn(missing_docs)]

pub use callback_query::*;
pub use chat::*;
pub use chat_join_request::*;
pub use chat_member_updated::*;
pub use inline_query::*;
pub use message::*;
pub use poll::*;
pub use pre_checkout_query::*;
pub use shipping_query::*;
pub use update::*;
pub use user::*;
pub use web_app::*;
pub use webhook_info::*;

mod callback_query;
mod chat;
mod chat_join_request;
mod chat_member_updated;
mod inline_query;
mod message;
mod poll;
mod pre_checkout_query;
mod shipping_query;
mod update;
mod user;
mod web_app;
mod webhook_info;

mod util;
