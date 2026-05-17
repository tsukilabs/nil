// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(
  addr_parse_ascii,
  const_clone,
  const_cmp,
  const_default,
  const_trait_impl,
  derive_const,
  lock_value_accessors,
  nonpoison_mutex,
  sync_nonpoison
)]

mod authorization;
mod circuit_breaker;
mod client;
mod error;
mod http;
mod request;
mod retry;
mod server;
mod websocket;

pub use client::Client;
pub use error::{AnyResult, Error, Result};
pub use nil_core as core;
pub use nil_crypto as crypto;
pub use nil_payload as payload;
pub use nil_server_types as server_types;
pub use request::Request;
pub use server::ServerAddr;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
