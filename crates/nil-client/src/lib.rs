// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(try_blocks)]

mod client;
mod error;
mod http;
mod server;
mod websocket;

pub use client::Client;
pub use error::{Error, Result};
pub use server::ServerAddr;
