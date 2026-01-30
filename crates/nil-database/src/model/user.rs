// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::Database;
use crate::error::{Error, Result};
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserId;
use crate::sql_types::player_id::SqlPlayerId;
use crate::sql_types::zoned::SqlZoned;
use diesel::prelude::*;
use nil_util::password::Password;

#[derive(Identifiable, Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
  pub id: UserId,
  pub player_id: SqlPlayerId,
  pub password: HashedPassword,
  pub created_at: SqlZoned,
  pub updated_at: SqlZoned,
}

impl User {
  #[inline]
  pub fn verify_password(&self, password: &Password) -> bool {
    self.password.verify(password)
  }
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::user)]
pub struct NewUser {
  player_id: SqlPlayerId,
  password: HashedPassword,
  created_at: SqlZoned,
  updated_at: SqlZoned,
}

impl NewUser {
  pub fn new(player_id: impl Into<SqlPlayerId>, password: &Password) -> Result<Self> {
    let player_id: SqlPlayerId = player_id.into();
    let id_len = player_id.trim().chars().count();
    let pass_len = password.trim().chars().count();

    if !(1..=20).contains(&id_len) {
      return Err(Error::InvalidUsername(player_id));
    } else if !(3..=50).contains(&pass_len) {
      return Err(Error::InvalidPassword);
    }

    Ok(Self {
      player_id,
      password: HashedPassword::new(password)?,
      created_at: SqlZoned::now(),
      updated_at: SqlZoned::now(),
    })
  }

  pub fn player_id(&self) -> SqlPlayerId {
    self.player_id.clone()
  }

  pub fn create(self, database: &Database) -> Result<usize> {
    database.create_user(&self)
  }
}
