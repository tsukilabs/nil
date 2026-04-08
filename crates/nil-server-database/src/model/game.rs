// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::Database;
use crate::error::{Error, Result};
use crate::sql_types::duration::Duration;
use crate::sql_types::game_id::GameId;
use crate::sql_types::hashed_password::HashedPassword;
use crate::sql_types::id::UserId;
use crate::sql_types::version::Version;
use crate::sql_types::zoned::Zoned;
use diesel::prelude::*;
use nil_core::world::World;
use nil_crypto::password::Password;
use nil_server_types::round::RoundDuration;
use std::fmt;
use tokio::task::spawn_blocking;

#[derive(Identifiable, Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::game)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(UserData, foreign_key = created_by))]
pub struct Game {
  pub id: GameId,
  pub password: Option<HashedPassword>,
  pub description: Option<String>,
  pub round_duration: Option<Duration>,
  pub created_by: UserId,
  pub created_at: Zoned,
  pub updated_at: Zoned,
}

impl From<GameWithBlob> for Game {
  fn from(game: GameWithBlob) -> Self {
    Self {
      id: game.id,
      password: game.password,
      description: game.description,
      round_duration: game.round_duration,
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
      .field("round_duration", &self.round_duration)
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
  pub round_duration: Option<Duration>,
  pub server_version: Version,
  pub created_by: UserId,
  pub created_at: Zoned,
  pub updated_at: Zoned,
  pub world_blob: Vec<u8>,
}

impl GameWithBlob {
  #[inline]
  pub fn to_world(&self) -> Result<World> {
    Ok(World::load(&self.world_blob)?)
  }
}

impl fmt::Debug for GameWithBlob {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("GameWithBlob")
      .field("id", &self.id.to_string())
      .field("round_duration", &self.round_duration)
      .field("created_by", &self.created_by)
      .field("created_at", &self.created_at.to_string())
      .field("updated_at", &self.updated_at.to_string())
      .finish_non_exhaustive()
  }
}

#[derive(Insertable, Clone)]
#[diesel(table_name = crate::schema::game)]
pub struct NewGame {
  id: GameId,
  password: Option<HashedPassword>,
  description: Option<String>,
  round_duration: Option<Duration>,
  server_version: Version,
  created_by: UserId,
  created_at: Zoned,
  updated_at: Zoned,
  world_blob: Vec<u8>,
}

#[bon::bon]
impl NewGame {
  #[builder]
  pub async fn new(
    #[builder(start_fn, into)] id: GameId,
    #[builder(start_fn)] blob: Vec<u8>,
    password: Option<Password>,
    mut description: Option<String>,
    round_duration: Option<RoundDuration>,
    #[builder(into)] server_version: Version,
    created_by: UserId,
  ) -> Result<Self> {
    if let Some(description) = &mut description {
      let chars = description.chars().count();
      let excess = chars.saturating_sub(1000);
      if excess > 0 {
        for _ in 0..excess {
          description.pop();
        }
      }
    }

    let now = Zoned::now();

    Ok(Self {
      id,
      password: hash_password(password).await?,
      description,
      round_duration: round_duration.map(Into::into),
      server_version,
      created_by,
      created_at: now.clone(),
      updated_at: now,
      world_blob: blob,
    })
  }

  #[inline]
  pub fn blob(&self) -> &[u8] {
    &self.world_blob
  }

  #[inline]
  pub async fn create(self, database: &Database) -> Result<usize> {
    database.create_game(self).await
  }
}

impl fmt::Debug for NewGame {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("NewGame")
      .field("id", &self.id.to_string())
      .field("round_duration", &self.round_duration)
      .field("server_version", &self.server_version)
      .field("created_by", &self.created_by)
      .field("created_at", &self.created_at.to_string())
      .field("updated_at", &self.updated_at.to_string())
      .finish_non_exhaustive()
  }
}

async fn hash_password(password: Option<Password>) -> Result<Option<HashedPassword>> {
  let Some(password) = password else { return Ok(None) };
  let pass_len = password.trim().chars().count();

  if pass_len == 0 {
    return Ok(None);
  } else if !(3..=50).contains(&pass_len) {
    return Err(Error::InvalidPassword);
  }

  spawn_blocking(move || HashedPassword::new(&password))
    .await?
    .map(Some)
    .map_err(Into::into)
}
