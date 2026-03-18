// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::Database;
use crate::error::{Error, Result};
use crate::sql_types::duration::SqlDuration;
use crate::sql_types::game_id::GameId;
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserId;
use crate::sql_types::version::SqlVersion;
use crate::sql_types::zoned::SqlZoned;
use diesel::prelude::*;
use nil_core::world::World;
use nil_crypto::password::Password;
use nil_server_types::round::RoundDuration;
use std::fmt;

#[derive(Identifiable, Queryable, Selectable, Clone)]
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

impl From<GameWithBlob> for Game {
  fn from(game: GameWithBlob) -> Self {
    Self {
      id: game.id,
      password: game.password,
      description: game.description,
      created_by: game.created_by,
      created_at: game.created_at,
      updated_at: game.updated_at,
    }
  }
}

impl fmt::Debug for Game {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Game")
      .field("id", &self.id.to_string())
      .field("created_by", &self.created_by)
      .field("created_at", &self.created_at.to_string())
      .field("updated_at", &self.updated_at.to_string())
      .finish_non_exhaustive()
  }
}

#[derive(Identifiable, Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::game)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(UserData, foreign_key = created_by))]
pub struct GameWithBlob {
  pub id: GameId,
  pub password: Option<HashedPassword>,
  pub description: Option<String>,
  pub round_duration: Option<SqlDuration>,
  pub server_version: SqlVersion,
  pub created_by: UserId,
  pub created_at: SqlZoned,
  pub updated_at: SqlZoned,
  pub world_blob: Vec<u8>,
}

impl GameWithBlob {
  #[inline]
  pub fn to_world(&self) -> Result<World> {
    Ok(World::load(&self.world_blob)?)
  }
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::game)]
pub struct NewGame {
  id: GameId,
  password: Option<HashedPassword>,
  description: Option<String>,
  round_duration: Option<SqlDuration>,
  server_version: SqlVersion,
  created_by: UserId,
  created_at: SqlZoned,
  updated_at: SqlZoned,
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
    round_duration: Option<RoundDuration>,
    #[builder(into)] server_version: SqlVersion,
    created_by: UserId,
  ) -> Result<Self> {
    if let Some(description) = &mut description {
      while description.len() > 1000 {
        description.pop();
      }
    }

    Ok(Self {
      id,
      password: hash_password(password)?,
      description,
      round_duration: round_duration.map(Into::into),
      server_version,
      created_by,
      created_at: SqlZoned::now(),
      updated_at: SqlZoned::now(),
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

fn hash_password(password: Option<&Password>) -> Result<Option<HashedPassword>> {
  let Some(password) = password else { return Ok(None) };
  let pass_len = password.trim().chars().count();

  if !(3..=50).contains(&pass_len) {
    return Err(Error::InvalidPassword);
  }

  Ok(Some(HashedPassword::new(password)?))
}
