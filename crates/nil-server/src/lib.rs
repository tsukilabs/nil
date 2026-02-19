// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(
  iterator_try_collect,
  try_blocks,
  try_blocks_heterogeneous,
  try_trait_v2
)]

mod app;
mod error;
mod middleware;
mod response;
mod router;
mod server;
mod websocket;

pub use error::{Error, Result};
pub use server::{local, remote};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
