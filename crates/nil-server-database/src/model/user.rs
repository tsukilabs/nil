// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::Database;
use crate::error::{Error, Result};
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserId;
use crate::sql_types::player_id::PlayerId;
use crate::sql_types::zoned::Zoned;
use diesel::prelude::*;
use nil_crypto::password::Password;
use std::fmt;
use tokio::task::spawn_blocking;

#[derive(Identifiable, Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
  pub id: UserId,
  pub player_id: PlayerId,
  pub password: HashedPassword,
  pub created_at: Zoned,
  pub updated_at: Zoned,
}

impl User {
  #[inline]
  pub async fn verify_password(&self, password: Password) -> Result<bool> {
    let hashed = self.password.clone();
    spawn_blocking(move || hashed.verify(&password))
      .await?
      .map_err(Into::into)
  }
}

impl fmt::Debug for User {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("User")
      .field("id", &self.id.to_string())
      .field("player_id", &self.player_id)
      .field("created_at", &self.created_at.to_string())
      .field("updated_at", &self.updated_at.to_string())
      .finish_non_exhaustive()
  }
}

#[derive(Insertable, Clone)]
#[diesel(table_name = crate::schema::user)]
pub struct NewUser {
  player_id: PlayerId,
  password: HashedPassword,
  created_at: Zoned,
  updated_at: Zoned,
}

impl NewUser {
  pub async fn new(player_id: impl Into<PlayerId>, password: Password) -> Result<Self> {
    let player_id: PlayerId = player_id.into();
    let id_len = player_id.trim().chars().count();

    if !(1..=20).contains(&id_len) {
      return Err(Error::InvalidUsername(player_id));
    }

    Ok(Self {
      player_id,
      password: hash_password(password).await?,
      created_at: Zoned::now(),
      updated_at: Zoned::now(),
    })
  }

  pub fn player_id(&self) -> PlayerId {
    self.player_id.clone()
  }

  pub async fn create(self, database: &Database) -> Result<usize> {
    database.create_user(self).await
  }
}

impl fmt::Debug for NewUser {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("NewUser")
      .field("player_id", &self.player_id)
      .field("created_at", &self.created_at.to_string())
      .field("updated_at", &self.updated_at.to_string())
      .finish_non_exhaustive()
  }
}

async fn hash_password(password: Password) -> Result<HashedPassword> {
  let pass_len = password.trim().chars().count();
  if !(3..=50).contains(&pass_len) {
    return Err(Error::InvalidPassword);
  }

  spawn_blocking(move || HashedPassword::new(&password))
    .await?
    .map_err(Into::into)
}
