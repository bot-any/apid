//! This module contains types representing Telegram API objects.
pub use bool_const::*;
///
pub use bot::*;
pub use callback_query::*;
pub use chat::*;
pub use chat_join_request::*;
pub use chat_member_updated::*;
pub use game::*;
pub use inline_mode::*;
pub use media::*;
pub use message::*;
pub use passport::*;
pub use poll::*;
pub use pre_checkout_query::*;
pub use shipping_query::*;
pub use update::*;
pub use user::*;
pub use web_app::*;
pub use webhook_info::*;

mod bool_const;
mod bot;
mod callback_query;
mod chat;
mod chat_join_request;
mod chat_member_updated;
mod game;
mod inline_mode;
mod media;
mod message;
mod passport;
mod poll;
mod pre_checkout_query;
mod shipping_query;
mod update;
mod user;
mod web_app;
mod webhook_info;
