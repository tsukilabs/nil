// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(
  duration_constructors,
  try_blocks,
  try_blocks_heterogeneous,
  try_trait_v2
)]

mod app;
mod env;
mod error;
mod middleware;
mod response;
mod router;
mod server;
mod websocket;

pub use app::App;
pub use error::{Error, Result};
pub use server::{local, remote};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
