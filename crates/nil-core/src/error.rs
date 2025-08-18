// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::{ContinentIndex, Coord};
use crate::infrastructure::building::{BuildingId, BuildingLevel, MineId, StorageId};
use crate::npc::bot::BotId;
use crate::npc::precursor::PrecursorId;
use crate::player::PlayerId;
use crate::script::ScriptId;
use mlua::ExternalError as _;
use regex::Regex;
use serde::Serialize;
use serde::ser::Serializer;
use std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Clone, Debug, thiserror::Error)]
#[remain::sorted]
pub enum Error {
  #[error("Bot not found: {0}")]
  BotNotFound(BotId),

  #[error("No stats found for building \"{0}\"")]
  BuildingStatsNotFound(BuildingId),

  #[error("No stats found for building \"{0}\" at level {1}")]
  BuildingStatsNotFoundForLevel(BuildingId, BuildingLevel),

  #[error("Building \"{0}\" is already at its minimum level")]
  CannotDecreaseBuildingLevel(BuildingId),

  #[error("Building \"{0}\" is already at its maximum level")]
  CannotIncreaseBuildingLevel(BuildingId),

  #[error("Cheating is not allowed in this world")]
  CheatingNotAllowed,

  #[error("City not found: {0}")]
  CityNotFound(Coord),

  #[error("{}", display_lua_err(.0))]
  FailedToExecuteScript(#[from] mlua::Error),

  #[error("Failed to load world")]
  FailedToLoadWorld,

  #[error("Failed to save world")]
  FailedToSaveWorld,

  #[error("Not authorized to execute this action")]
  Forbidden,

  #[error("Index out of bounds: {0}")]
  IndexOutOfBounds(ContinentIndex),

  #[error("Insufficient resources")]
  InsufficientResources,

  #[error("No stats found for mine \"{0}\"")]
  MineStatsNotFound(MineId),

  #[error("No stats found for mine \"{0}\" at level {1}")]
  MineStatsNotFoundForLevel(MineId, BuildingLevel),

  #[error("No players in the world")]
  NoPlayer,

  #[error("Player already spawned: {0}")]
  PlayerAlreadySpawned(PlayerId),

  #[error("Player \"{0}\" has already taken their turn")]
  PlayerIsNotPending(PlayerId),

  #[error("Player not found: {0}")]
  PlayerNotFound(PlayerId),

  #[error("Precursor not found: {0}")]
  PrecursorNotFound(PrecursorId),

  #[error("Round already started")]
  RoundAlreadyStarted,

  #[error("Round has pending players")]
  RoundHasPendingPlayers,

  #[error("Round has not started yet")]
  RoundNotStarted,

  #[error("Script not found: {0}")]
  ScriptNotFound(ScriptId),

  #[error("No stats found for storage \"{0}\"")]
  StorageStatsNotFound(StorageId),

  #[error("No stats found for storage \"{0}\" at level {1}")]
  StorageStatsNotFoundForLevel(StorageId, BuildingLevel),

  #[error("World is full")]
  WorldIsFull,
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}

impl From<Error> for mlua::Error {
  fn from(err: Error) -> Self {
    err.into_lua_err()
  }
}

fn display_lua_err(err: &mlua::Error) -> String {
  let err = err.to_string();
  Regex::new(r"crates.+?\.rs:\d+?:\d+?:")
    .expect("regex should be valid")
    .replace_all(&err, "")
    .split("stack traceback")
    .next()
    .unwrap_or_default()
    .trim()
    .to_owned()
}

pub trait WrapOk<T> {
  fn wrap_ok(self) -> Result<T>;
}

impl<T> WrapOk<T> for T {
  fn wrap_ok(self) -> Result<T> {
    Ok(self)
  }
}
