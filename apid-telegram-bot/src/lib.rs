//! # apid-telegram-bot
//!
//! This crate provides types for Telegram API.
//! The tpyes implementing [`Debug`], [`Serialize`](serde::Serialize) and [`Deserialize`](serde::Deserialize) traits for covenience.
//! Also, carefully designed enum variants for ergonomics.
//!
//! ## Telegram Parity
//!
//! Current type definitions are written for Bot API 6.2 (August 12, 2022)
//!
//! Warning:
//! Currently, the crate  has lack of types and very WIP.

#![warn(missing_docs)]

pub mod types;

mod util;
