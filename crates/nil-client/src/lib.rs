// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(addr_parse_ascii, try_blocks)]

mod client;
mod error;
mod http;
mod server;
mod websocket;

pub use client::Client;
pub use error::{AnyResult, Error, Result};
pub use server::ServerAddr;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
