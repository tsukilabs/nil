// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(try_blocks)]

mod client;
mod error;
mod http;
mod websocket;

pub use client::Client;
pub use error::{Error, Result};
