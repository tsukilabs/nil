// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(nonpoison_mutex, str_as_str, sync_nonpoison, try_blocks)]

pub mod error;
mod migration;
pub mod model;
mod schema;
pub mod sql_types;

use crate::model::user_data::{NewUserData, UserData};
use crate::sql_types::user::User;
use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use error::{AnyResult, Result};
use nil_server_types::Password;
use std::sync::Arc;
use std::sync::nonpoison::{Mutex, MutexGuard};

#[must_use]
#[derive(Clone)]
pub struct DatabaseHandle(Arc<Mutex<SqliteConnection>>);

impl DatabaseHandle {
  pub fn new(url: &str) -> AnyResult<Self> {
    let mut conn = SqliteConnection::establish(url)?;
    migration::run_pending_migrations(&mut conn);
    Ok(Self(Arc::new(Mutex::new(conn))))
  }

  fn conn(&self) -> MutexGuard<'_, SqliteConnection> {
    self.0.lock()
  }

  #[inline]
  pub fn create_user(&self, user: User, password: &Password) -> Result<()> {
    NewUserData::new(user, password)?.create(self)
  }

  #[inline]
  pub fn get_user(&self, user: &User) -> Result<UserData> {
    UserData::get(self, user)
  }
}
