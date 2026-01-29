// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(str_as_str, try_blocks)]

mod client;
mod error;
mod http;
mod server;
mod websocket;

pub use client::Client;
pub use error::{AnyResult, Error, Result};
pub use server::ServerAddr;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
