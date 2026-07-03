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
  str_as_str
)]

pub mod error;
mod r#impl;
mod migration;
pub mod model;
mod schema;
pub mod sql_types;

use crate::error::Result;
use crate::migration::run_pending_migrations;
use diesel::SqliteConnection;
use diesel_async::AsyncConnection;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use std::fmt;
use std::sync::Arc;
use tokio::sync::Mutex;

type Conn = SyncConnectionWrapper<SqliteConnection>;

#[must_use]
#[derive(Clone)]
pub struct Database(Arc<Mutex<Conn>>);

impl Database {
  pub async fn new(url: &str) -> Result<Self> {
    let conn = Conn::establish(url).await?;
    let conn = run_pending_migrations(conn)?;
    Ok(Self(Arc::new(Mutex::new(conn))))
  }
}

impl fmt::Debug for Database {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("Database")
      .finish_non_exhaustive()
  }
}
