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
use diesel::{SqliteConnection, sql_query};
use diesel_async::RunQueryDsl;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::{Hook, HookError, Pool};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use std::borrow::Cow;

type Conn = SyncConnectionWrapper<SqliteConnection>;

#[must_use]
#[derive(Clone)]
pub struct Database {
  pool: Pool<Conn>,
}

impl Database {
  pub fn new(url: &str) -> Result<Self> {
    run_pending_migrations(url)?;
    let manager = AsyncDieselConnectionManager::new(url);
    let pool = Pool::builder(manager)
      .max_size(10)
      .post_create(Hook::async_fn(move |conn: &mut Conn, _| {
        Box::pin(async move {
          execute(conn, "PRAGMA journal_mode=WAL;").await?;
          execute(conn, "PRAGMA synchronous=NORMAL;").await?;
          execute(conn, "PRAGMA busy_timeout=5000;").await?;
          execute(conn, "PRAGMA foreign_keys=ON;").await?;
          Ok(())
        })
      }))
      .build()?;

    Ok(Self { pool })
  }
}

#[doc(hidden)]
#[macro_export]
macro_rules! conn {
  ($database:expr) => {
    &mut *$database.pool.get().await?
  };
}

async fn execute(conn: &mut Conn, query: &str) -> Result<(), HookError> {
  sql_query(query)
    .execute(conn)
    .await
    .map_err(|err| HookError::Message(Cow::Owned(err.to_string())))?;

  Ok(())
}
