// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::Database;
use crate::error::{Error, Result};
use crate::sql_types::game_id::GameId;
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserId;
use crate::sql_types::version::SqlVersion;
use crate::sql_types::zoned::SqlZoned;
use diesel::prelude::*;
use nil_core::world::World;
use nil_util::password::Password;

#[derive(Identifiable, Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::game)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(UserData, foreign_key = created_by))]
pub struct Game {
  pub id: GameId,
  pub password: Option<HashedPassword>,
  pub description: Option<String>,
  pub created_by: UserId,
  pub created_at: SqlZoned,
  pub updated_at: SqlZoned,
}

#[derive(Identifiable, Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::game)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(UserData, foreign_key = created_by))]
pub struct GameWithBlob {
  pub id: GameId,
  pub password: Option<HashedPassword>,
  pub description: Option<String>,
  pub created_by: UserId,
  pub created_at: SqlZoned,
  pub updated_at: SqlZoned,
  pub server_version: SqlVersion,
  pub world_blob: Vec<u8>,
}

impl GameWithBlob {
  #[inline]
  pub fn into_world(self) -> Result<World> {
    Ok(World::load(&self.world_blob)?)
  }
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::game)]
pub struct NewGame {
  id: GameId,
  password: Option<HashedPassword>,
  description: Option<String>,
  created_by: UserId,
  created_at: SqlZoned,
  updated_at: SqlZoned,
  server_version: SqlVersion,
  world_blob: Vec<u8>,
}

#[bon::bon]
impl NewGame {
  #[builder]
  pub fn new(
    #[builder(start_fn, into)] id: GameId,
    #[builder(start_fn)] blob: Vec<u8>,
    password: Option<&Password>,
    mut description: Option<String>,
    created_by: UserId,
    server_version: SqlVersion,
  ) -> Result<Self> {
    if let Some(password) = password {
      let pass_len = password.trim().chars().count();
      if !(3..=50).contains(&pass_len) {
        return Err(Error::InvalidPassword);
      }
    }

    if let Some(description) = description.as_mut() {
      while description.len() > 1000 {
        description.pop();
      }
    }

    Ok(Self {
      id,
      password: password
        .map(HashedPassword::new)
        .transpose()?,
      description,
      created_by,
      created_at: SqlZoned::now(),
      updated_at: SqlZoned::now(),
      server_version,
      world_blob: blob,
    })
  }

  #[inline]
  pub fn blob(&self) -> &[u8] {
    &self.world_blob
  }

  #[inline]
  pub fn create(self, database: &Database) -> Result<usize> {
    database.create_game(&self)
  }
}
