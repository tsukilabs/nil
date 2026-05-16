// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(
  const_clone,
  const_cmp,
  const_convert,
  const_trait_impl,
  derive_const,
  nonpoison_mutex,
  str_as_str,
  sync_nonpoison
)]

mod database;
pub mod error;
mod migration;
pub mod model;
mod schema;
pub mod sql_types;

pub use database::{BlockingDatabase, Database};
