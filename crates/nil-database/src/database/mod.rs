// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod user;
mod game;

use crate::error::AnyResult;
use crate::migration::run_pending_migrations;
use diesel::prelude::*;
use std::sync::Arc;
use std::sync::nonpoison::{Mutex, MutexGuard};

#[must_use]
#[derive(Clone)]
pub struct Database(Arc<Mutex<SqliteConnection>>);

impl Database {
  pub fn new(url: &str) -> AnyResult<Self> {
    let mut conn = SqliteConnection::establish(url)?;
    run_pending_migrations(&mut conn);
    Ok(Self(Arc::new(Mutex::new(conn))))
  }

  fn conn(&self) -> MutexGuard<'_, SqliteConnection> {
    self.0.lock()
  }
}
